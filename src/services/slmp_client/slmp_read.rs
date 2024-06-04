use std::{
    hash::BuildHasherDefault, net::TcpStream,
    sync::{mpsc::Sender, Arc, Mutex, RwLock},
    thread::{self, JoinHandle}, time::Duration,
};
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};
use crate::{
    conf::{diag_keywd::DiagKeywd, point_config::name::Name, slmp_client_config::slmp_client_config::SlmpClientConfig},
    core_::{
        failure::errors_limit::ErrorLimit, point::point_type::PointType, state::{change_notify::ChangeNotify, exit_notify::ExitNotify},
        status::status::Status, types::map::IndexMapFxHasher
    },
    services::{
        diagnosis::diag_point::DiagPoint,
        slmp_client::slmp_db::SlmpDb,
        task::service_cycle::ServiceCycle,
    }
};
///
/// Cyclicaly reads SLMP data ranges (DB's) specified in the [conf]
/// - exit - external signal to stop the main read cicle and exit the thread
/// - exit_pair - exit signal from / to notify 'Write' partner to exit the thread
pub struct SlmpRead {
    tx_id: usize,
    id: String,
    name: Name,
    conf: SlmpClientConfig,
    dest: Sender<PointType>,
    dbs: Arc<RwLock<IndexMapFxHasher<String, SlmpDb>>>,
    diagnosis: Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
    exit: Arc<ExitNotify>,
}
impl SlmpRead {
    ///
    /// Creates new instance of the SlpmRead
    pub fn new(
        parent: impl Into<String>,
        tx_id: usize,
        name: Name,
        conf: SlmpClientConfig,
        dest: Sender<PointType>,
        diagnosis: Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
        exit: Arc<ExitNotify>,
    ) -> Self {
        let self_id = format!("{}/SlmpRead", parent.into());
        let dbs = Self::build_dbs(&self_id, tx_id, &conf);
        Self {
            tx_id,
            id: self_id.clone(),
            name,
            conf,
            dest,
            dbs: Arc::new(RwLock::new(dbs)),
            diagnosis,
            exit,
        }
    }
    ///
    /// Sends diagnosis point
    fn yield_diagnosis(
        self_id: &str,
        diagnosis: &Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
        kewd: &DiagKeywd,
        value: Status,
        dest: &Sender<PointType>,
    ) {
        match diagnosis.lock() {
            Ok(mut diagnosis) => {
                match diagnosis.get_mut(kewd) {
                    Some(point) => {
                        debug!("{}.yield_diagnosis | Sending diagnosis point '{}' ", self_id, kewd);
                        if let Some(point) = point.next(value) {
                            if let Err(err) = dest.send(point) {
                                warn!("{}.yield_status | Send error: {}", self_id, err);
                            }
                        }
                    }
                    None => debug!("{}.yield_diagnosis | Diagnosis point '{}' - not configured", self_id, kewd),
                }
            }
            Err(err) => error!("{}.yield_diagnosis | Diagnosis lock error: {:#?}", self_id, err),
        }
    }
    ///
    /// Sends all configured points from the current DB with the given status
    fn yield_status(self_id: &str, status: Status, dbs: &mut IndexMapFxHasher<String, SlmpDb>, dest: &Sender<PointType>) {
        for (db_name, db) in dbs {
            debug!("{}.yield_status | DB '{}' - sending Invalid status...", self_id, db_name);
            match db.yield_status(status, dest) {
                Ok(_) => {}
                Err(err) => {
                    error!("{}.yield_status | send errors: \n\t{:?}", self_id, err);
                }
            };
        }
    }
    ///
    ///
    pub fn build_dbs(self_id: &str, tx_id: usize, conf: &SlmpClientConfig) -> IndexMapFxHasher<String, SlmpDb> {
        let mut dbs = IndexMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
        for (db_name, db_conf) in &conf.dbs {
            info!("{}.build_dbs | Configuring SlmpDb: {:?}...", self_id, db_name);
            let db = SlmpDb::new(self_id, tx_id, &db_conf);
            dbs.insert(db_name.clone(), db);
            info!("{}.build_dbs | Configuring SlmpDb: {:?} - ok", self_id, db_name);
        }
        dbs
    }
    ///
    /// Cyclicaly reads data slice from the device,
    pub fn run(&mut self, mut tcp_stream: TcpStream) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.read | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let dbs = self.dbs.clone();
        let diagnosis = self.diagnosis.clone();
        let dest = self.dest.clone();
        let cycle = conf.cycle.map_or(None, |cycle| if cycle != Duration::ZERO {Some(cycle)} else {None});
        match cycle {
            Some(cycle_interval) => {
                info!("{}.read | Preparing thread...", self_id);
                let handle = thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {
                    let mut is_connected = ChangeNotify::new(
                        &self_id,
                        false,
                        vec![
                            (true,  Box::new(|message| info!("{}", message))),
                            (false, Box::new(|message| warn!("{}", message))),
                        ],
                    );
                    let mut cycle = ServiceCycle::new(&self_id, cycle_interval);
                    let mut status = Status::Ok;
                    let mut dbs = dbs.write().unwrap();
                    'main: while !exit.get() {
                        let mut error_limit = ErrorLimit::new(3);
                        is_connected.add(true, &format!("{}.read | Connection established", self_id));
                        Self::yield_diagnosis(&self_id, &diagnosis, &DiagKeywd::Connection, Status::Ok, &dest);
                        cycle.start();
                        for (db_name, db) in dbs.iter_mut() {
                            trace!("{}.read | SlmpDb '{}' - reading...", self_id, db_name);
                            match db.read(&mut tcp_stream, &dest) {
                                Ok(_) => {
                                    error_limit.reset();
                                    trace!("{}.read | SlmpDb '{}' - reading - ok", self_id, db_name);
                                }
                                Err(err) => {
                                    warn!("{}.read | SlmpDb '{}' - reading - error: {:?}", self_id, db_name, err);
                                    if error_limit.add().is_err() {
                                        error!("{}.read | SlmpDb '{}' - exceeded reading errors limit, trying to reconnect...", self_id, db_name);
                                        status = Status::Invalid;
                                        Self::yield_diagnosis(&self_id, &diagnosis, &DiagKeywd::Connection, Status::Invalid, &dest);
                                        exit.exit_pair();
                                        break 'main;
                                    }
                                }
                            }
                            if exit.get() {
                                break 'main;
                            }
                        }
                        cycle.wait();
                    }
                    if status != Status::Ok {
                        Self::yield_status(&self_id, Status::Invalid, &mut dbs, &dest);
                    }
                    info!("{}.read | Exit", self_id);
                });
                info!("{}.read | Started", self.id);
                handle
            }
            None => {
                info!("{}.read | Disabled", self.id);
                let exit = self.exit.clone();
                thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {
                    while !exit.get() {
                        thread::sleep(Duration::from_millis(64));
                    }
                })
            }
        }
    }
}

use std::{hash::BuildHasherDefault, io::{BufReader, Read}, net::TcpStream, sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};

use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};

use crate::{conf::{diag_keywd::DiagKeywd, point_config::name::Name, slmp_client_config::slmp_client_config::SlmpClientConfig}, core_::{failure::errors_limit::ErrorsLimit, net::connection_status::{ConnectionStatus, SocketState}, point::point_type::PointType, state::change_notify::ChangeNotify, status::status::Status, types::map::IndexMapFxHasher}, services::{diagnosis::diag_point::DiagPoint, slmp_client::slmp_db::SlmpDb, task::service_cycle::ServiceCycle}, tcp::tcp_stream_write::OpResult};

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
    diagnosis: Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
    exit: Arc<AtomicBool>,
    exit_pair: Arc<AtomicBool>,
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
        exit: Option<Arc<AtomicBool>>, 
        exit_pair: Option<Arc<AtomicBool>>,
    ) -> Self {
        Self {
            tx_id,
            id: format!("{}/SlmpRead", parent.into()),
            name,
            conf,
            dest,
            diagnosis,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exit_pair: exit_pair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Sends diagnosis point
    fn yield_diagnosis(
        self_id: &str,
        diagnosis: &Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
        kewd: &DiagKeywd,
        value: Status,
        tx_send: &Sender<PointType>,
    ) {
        match diagnosis.lock() {
            Ok(mut diagnosis) => {
                match diagnosis.get_mut(kewd) {
                    Some(point) => {
                        debug!("{}.yield_diagnosis | Sending diagnosis point '{}' ", self_id, kewd);
                        if let Some(point) = point.next(value) {
                            if let Err(err) = tx_send.send(point) {
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
    fn yield_status(self_id: &str, status: Status, dbs: &mut IndexMapFxHasher<String, SlmpDb>, tx_send: &Sender<PointType>) {
        for (db_name, db) in dbs {
            debug!("{}.yield_status | DB '{}' - sending Invalid status...", self_id, db_name);
            match db.yield_status(status, tx_send) {
                Ok(_) => {}
                Err(err) => {
                    error!("{}.yield_status | send errors: \n\t{:?}", self_id, err);
                }
            };
        }
    }
    ///
    /// Reads data slice from the S7 device,
    pub fn read(&mut self, tcp_stream: TcpStream) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.read | starting...", self.id);
        let self_id = self.id.clone();
        let tx_id = self.tx_id;
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let dest = self.dest.clone();
        let diagnosis = self.diagnosis.clone();
        let cycle = conf.cycle.map_or(None, |cycle| if cycle > Duration::ZERO {Some(cycle)} else {None});
        let mut tcp_stream = BufReader::new(tcp_stream);
        match cycle {
            Some(cycle_interval) => {
                info!("{}.read | Preparing thread...", self_id);
                let handle = thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {
                    let mut is_connected = ChangeNotify::new(
                        self_id,
                        false,
                        vec![
                            (true,  Box::new(|message| info!("{}", message))),
                            (false, Box::new(|message| warn!("{}", message))),
                        ],
                    );
                    let mut dbs = IndexMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
                    for (db_name, db_conf) in conf.dbs {
                        info!("{}.read | configuring DB: {:?}...", self_id, db_name);
                        let db = SlmpDb::new(&self_id, tx_id, &db_conf);
                        dbs.insert(db_name.clone(), db);
                        info!("{}.read | configuring DB: {:?} - ok", self_id, db_name);
                    }
                    let mut cycle = ServiceCycle::new(&self_id, cycle_interval);
                    let mut error_limit = ErrorsLimit::new(3);
                    let mut status;
                    status = Status::Ok;
                    is_connected.add(true, &format!("{}.read | Connection established", self_id));
                    Self::yield_diagnosis(&self_id, &diagnosis, &DiagKeywd::Connection, Status::Ok, &dest);
                    'read: while !exit.load(Ordering::SeqCst) {
                        cycle.start();
                        for (db_name, db) in &mut dbs {
                            trace!("{}.read | DB '{}' - reading...", self_id, db_name);
                            match db.read(&tcp_stream, &dest) {
                                Ok(_) => {
                                    error_limit.reset();
                                    trace!("{}.read | DB '{}' - reading - ok", self_id, db_name);
                                }
                                Err(err) => {
                                    error!("{}.read | DB '{}' - reading - error: {:?}", self_id, db_name, err);
                                    if error_limit.add().is_err() {
                                        error!("{}.read | DB '{}' - exceeded reading errors limit, trying to reconnect...", self_id, db_name);
                                        status = Status::Invalid;
                                        Self::yield_diagnosis(&self_id, &diagnosis, &DiagKeywd::Connection, Status::Invalid, &tx_send);
                                        if let Err(err) = client.close() {
                                            error!("{}.read | {:?}", self_id, err);
                                        };
                                        break 'read;
                                    }
                                }
                            }
                            if exit.load(Ordering::SeqCst) {
                                break 'main;
                            }
                        }
                        cycle.wait();
                    }
                    if status != Status::Ok {
                        Self::yield_status(&self_id, Status::Invalid, &mut dbs, &dest);
                    }
                    // 'main: while !exit.load(Ordering::SeqCst) {
                    //     let mut error_limit = ErrorsLimit::new(3);
                    //     let mut status;
                    //     match client.connect() {
                    //         Ok(_) => {
                    //         }
                    //         Err(err) => {
                    //             is_connected.add(false, &format!("{}.read | Connection lost: {:?}", self_id, err));
                    //             trace!("{}.read | Connection error: {:?}", self_id, err);
                    //         }
                    //     }
                    //     thread::sleep(conf.reconnect_cycle);
                    // }
                    info!("{}.read | Exit", self_id);
                });
                info!("{}.read | Started", self.id);
                handle
            }
            None => {
                info!("{}.read | Disabled", self.id);
                thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {})
            }
        }
    }
}
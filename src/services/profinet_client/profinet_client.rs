use std::{fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Sender}, Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};
use crate::{
    conf::{point_config::{name::Name, point_config::PointConfig}, profinet_client_config::profinet_client_config::ProfinetClientConfig}, 
    core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, failure::errors_limit::ErrorsLimit, object::object::Object, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status}, 
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, profinet_client::{profinet_db::ProfinetDb, s7::s7_client::S7Client}, safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services, task::service_cycle::ServiceCycle
    },
};
///
/// Cyclically reads adressess from the PROFINET device and yields changed to the MultiQueue
/// Writes Point to the protocol (PROFINET device) specific address
pub struct ProfinetClient {
    tx_id: usize,
    id: String,
    name: Name,
    conf: ProfinetClientConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl ProfinetClient {
    ///
    /// 
    pub fn new(conf: ProfinetClientConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            tx_id: PointTxId::fromStr(&conf.name.join()),
            id: format!("{}", conf.name),
            name: conf.name.clone(),
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Returns updated points from the current DB
    ///     - reads data slice from the S7 device,
    ///     - parses raw data into the configured points
    ///     - returns only points with updated value or status
    fn yield_status(self_id: &str, dbs: &mut IndexMap<String, ProfinetDb>, tx_send: &Sender<PointType>) {
        for (db_name, db) in dbs {
            debug!("{}.run | DB '{}' - reading...", self_id, db_name);
            match db.yield_status(Status::Invalid, tx_send) {
                Ok(_) => {},
                Err(err) => {
                    error!("{}.lostConnection | send errors: \n\t{:?}", self_id, err);
                },
            };
        }
    }
    ///
    /// Reads data slice from the S7 device,
    fn read(&mut self, tx_send: Sender<PointType>) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.read | starting...", self.id);
        let self_id = self.id.clone();
        let self_name = self.name.clone();
        let tx_id = self.tx_id;
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        match conf.cycle {
            Some(cycle_interval) => {
                if cycle_interval > Duration::ZERO {
                    info!("{}.read | Preparing thread...", self_id);
                    let handle = thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {
                        let mut is_connected = ConnectionNotify::new(
                            None, 
                            Box::new(|message| info!("{}", message)), 
                            Box::new(|message| warn!("{}", message)),
                        );
                        let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
                        for (db_name, db_conf) in conf.dbs {
                            info!("{}.read | configuring DB: {:?}...", self_id, db_name);
                            let db = ProfinetDb::new(&self_id, &self_name, tx_id, &db_conf);
                            dbs.insert(db_name.clone(), db);
                            info!("{}.read | configuring DB: {:?} - ok", self_id, db_name);
                        }
                        let mut cycle = ServiceCycle::new(&self_id, cycle_interval);
                        let mut client = S7Client::new(self_id.clone(), conf.ip.clone());
                        'main: while !exit.load(Ordering::SeqCst) {
                            let mut error_limit = ErrorsLimit::new(3);
                            let mut status = Status::Ok;
                            match client.connect() {
                                Ok(_) => {
                                    is_connected.add(true, &format!("{}.read | Connection established", self_id));
                                    'read: while !exit.load(Ordering::SeqCst) {
                                        cycle.start();
                                        for (db_name, db) in &mut dbs {
                                            trace!("{}.read | DB '{}' - reading...", self_id, db_name);
                                            match db.read(&client, &tx_send) {
                                                Ok(_) => {
                                                    error_limit.reset();
                                                    trace!("{}.read | DB '{}' - reading - ok", self_id, db_name);
                                                },
                                                Err(err) => {
                                                    error!("{}.read | DB '{}' - reading - error: {:?}", self_id, db_name, err);
                                                    if error_limit.add().is_err() {
                                                        error!("{}.read | DB '{}' - exceeded reading errors limit, trying to reconnect...", self_id, db_name);
                                                        status = Status::Invalid;
                                                        if let Err(err) = client.close() {
                                                            error!("{}.read | {:?}", self_id, err);
                                                        };
                                                        break 'read;
                                                    }
                                                },
                                            }
                                            if exit.load(Ordering::SeqCst) {
                                                break 'main;
                                            }
                                        }
                                        cycle.wait();
                                    }
                                    if status != Status::Ok {
                                        Self::yield_status(&self_id, &mut dbs, &tx_send);
                                    }
                                },
                                Err(err) => {
                                    is_connected.add(false, &format!("{}.read | Connection lost: {:?}", self_id, err));
                                    trace!("{}.read | Connection error: {:?}", self_id, err);
                                },
                            }
                            thread::sleep(conf.reconnect_cycle);
                        }
                    });
                    info!("{}.read | Started", self.id);
                    handle
                } else {
                    info!("{}.read | Disabled", self.id);
                    thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {})
                }
            },
            None => {
                info!("{}.read | Disabled", self.id);
                thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {})
            },
        }
    }
    ///
    /// Writes Point to the protocol (PROFINET device) specific address
    fn write(&mut self, tx_send: Sender<PointType>) -> Result<JoinHandle<()>, std::io::Error> {
        let self_id = self.id.clone();
        let self_name = self.name.clone();
        let tx_id = self.tx_id;
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        info!("{}.write | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.write", self_id.clone())).spawn(move || {
            let mut is_connected = ConnectionNotify::new(
                None, 
                Box::new(|message| info!("{}", message)), 
                Box::new(|message| warn!("{}", message)),
            );
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            let mut points: Vec<PointConfig> = vec![];
            for (db_name, db_conf) in conf.dbs {
                info!("{}.write | configuring DB: {:?}...", self_id, db_name);
                let db = ProfinetDb::new(&self_id, &self_name, tx_id, &db_conf);
                dbs.insert(db_name.clone(), db);
                info!("{}.write | configuring DB: {:?} - ok", self_id, db_name);
                points.extend(db_conf.points());
            }
            // debug!("{}.write | Point configs ({}) :", self_id, points.len());
            // for cfg in &points {
            //     println!("\t{:?}", cfg);
            // }
            let points = points.iter().map(|point_conf| {
                SubscriptionCriteria::new(&point_conf.name, Cot::Act)
            }).collect::<Vec<SubscriptionCriteria>>();
            debug!("{}.write | Points subscribed on: ({})", self_id, points.len());
            for name in &points {
                println!("\t{:?}", name);
            }
            let (_, rx_recv) = services.slock().subscribe(&conf.subscribe, &self_id, &points);
            // let mut cycle = ServiceCycle::new(cycle_interval);
            let mut client = S7Client::new(self_id.clone(), conf.ip.clone());
            'main: while !exit.load(Ordering::SeqCst) {
                let mut errors_limit = ErrorsLimit::new(3);
                thread::sleep(conf.reconnect_cycle);
                match client.connect() {
                    Ok(_) => {
                        is_connected.add(true, &format!("{}.write | Connection established", self_id));
                        'write: while !exit.load(Ordering::SeqCst) {
                            // cycle.start();
                            match rx_recv.recv_timeout(RECV_TIMEOUT) {
                                Ok(point) => {
                                    let point_name = point.name();
                                    let point_value = point.value();
                                    let db_name = point_name.split('/').nth(3).unwrap();
                                    debug!("{}.write | DB '{}' - writing point '{}'\t({:?})...", self_id, db_name, point_name, point_value);
                                    // let dbName = point_name.split("/").skip(1).collect::<String>();
                                    match dbs.get_mut(db_name) {
                                        Some(db) => {
                                            match db.write(&client, point) {
                                                Ok(_) => {
                                                    errors_limit.reset();
                                                    debug!("{}.write | DB '{}' - writing point '{}'\t({:?}) - ok", self_id, db_name, point_name, point_value);
                                                    if let Err(err) = tx_send.send(PointType::String(Point::new(
                                                        tx_id,
                                                        &point_name, 
                                                        String::new(),
                                                        Status::Ok,
                                                        Cot::ActCon,
                                                        chrono::offset::Utc::now(),
                                                    ))) {
                                                        error!("{}.write | Error sending to queue: {:?}", self_id, err);
                                                        // break 'main;
                                                    };
                                                },
                                                Err(err) => {
                                                    error!("{}.write | DB '{}' - write - error: {:?}", self_id, db_name, err);
                                                    if errors_limit.add().is_err() {
                                                        error!("{}.write | DB '{}' - exceeded writing errors limit, trying to reconnect...", self_id, db_name);
                                                        if let Err(err) = tx_send.send(PointType::String(Point::new(
                                                            tx_id,
                                                            &point_name, 
                                                            format!("Write error: {}", err),
                                                            Status::Ok,
                                                            Cot::ActErr,
                                                            chrono::offset::Utc::now(),
                                                        ))) {
                                                            error!("{}.write | Error sending to queue: {:?}", self_id, err);
                                                            // break 'main;
                                                        };
                                                        if let Err(err) = client.close() {
                                                            error!("{}.write | {:?}", self_id, err);
                                                        };
                                                        break 'write;
                                                    }
                                                },
                                            }
                                        },
                                        None => {
                                            error!("{}.write | DB '{}' - not found", self_id, db_name);
                                        },
                                    };
                                },
                                Err(err) => {
                                    match err {
                                        mpsc::RecvTimeoutError::Timeout => {},
                                        mpsc::RecvTimeoutError::Disconnected => {
                                            error!("{}.write | Error receiving from queue: {:?}", self_id, err);
                                            break 'main;
                                        },
                                    }
                                }
                            }
                            if exit.load(Ordering::SeqCst) {
                                break 'main;
                            }
                        }
                    },
                    Err(err) => {
                        is_connected.add(false, &format!("{}.write | Connection lost: {:?}", self_id, err));
                        trace!("{}.write | Connection error: {:?}", self_id, err);
                    },
                }
            }
        });
        info!("{}.write | started", self.id);
        handle
    }
}
///
/// 
impl Object for ProfinetClient {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> Name {
        self.name.clone()
    }
}
///
/// 
impl Debug for ProfinetClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProfinetClient")
            .field("id", &self.id)
            .finish()
    }
}
///
/// 
impl Service for ProfinetClient {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        let tx_send = self.services.slock().get_link(&self.conf.tx).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.id, err);
        });
        let handle_read = self.read(tx_send.clone());
        let handle_write = self.write(tx_send);
        info!("{}.run | started", self.id);
        match (handle_read, handle_write) {
            (Ok(handle_read), Ok(handle_write)) => Ok(ServiceHandles::new(vec![
                (format!("{}/read", self.id), handle_read),
                (format!("{}/write", self.id), handle_write),
                ])),
            // TODO Exit 'write if read returns error'
            (Ok(handle_read), Err(err)) => Err(format!("{}.run | Error starting inner thread 'read': {:#?}", self.id, err)),
            // TODO Exit 'read if write returns error'
            (Err(err), Ok(handle_write)) => Err(format!("{}.run | Error starting inner thread 'write': {:#?}", self.id, err)),
            (Err(read_err), Err(write_err)) => Err(format!("{}.run | Error starting inner thread: \n\t  read: {:#?}\n\t write: {:#?}", self.id, read_err, write_err)),
        }
    }
    //
    //
    fn points(&self) -> Vec<PointConfig> {
        self.conf.points()
    }
    //
    // 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}

///
/// Logging connection status on changes only
struct ConnectionNotify {
    is_connected: Option<bool>,
    on_connected: Box<dyn Fn(&str)>,
    on_disconnected: Box<dyn Fn(&str)>,
}
///
/// 
impl ConnectionNotify {
    ///
    /// 
    pub fn new(initial: Option<bool>, on_connected: Box<dyn Fn(&str)>, on_disconnected: Box<dyn Fn(&str)>) -> Self {
        Self {
            is_connected: initial,
            on_connected,
            on_disconnected,
        }
    }
    ///
    /// Add new state
    pub fn add(&mut self, connected: bool, message: &str) {
        if Some(connected) != self.is_connected {
            self.is_connected = Some(connected);
            match connected {
                true => (self.on_connected)(message),
                false => (self.on_disconnected)(message),
            }
        }
    }
}
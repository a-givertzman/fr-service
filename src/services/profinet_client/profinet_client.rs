use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}, time::Duration};
use indexmap::IndexMap;
use log::{debug, error, info, trace};
use crate::{
    conf::{point_config::point_config::PointConfig, profinet_client_config::profinet_client_config::ProfinetClientConfig}, 
    core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, object::object::Object, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status}, 
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, profinet_client::{profinet_db::ProfinetDb, s7::s7_client::S7Client}, 
        service::{service::Service, service_handles::ServiceHandles}, services::Services, task::service_cycle::ServiceCycle
    },
};


///
/// Cyclically reads adressess from the PROFINET device and yields changed to the MultiQueue
/// Writes Point to the protocol (PROFINET device) specific address
pub struct ProfinetClient {
    id: String,
    app: String,
    rx_recv: Vec<Receiver<PointType>>,
    conf: ProfinetClientConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl ProfinetClient {
    ///
    /// 
    pub fn new(app:&str, parent: impl Into<String>, conf: ProfinetClientConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: format!("{}/ProfinetClient({})", parent.into(), conf.name),
            app: app.to_owned(),
            rx_recv: vec![],
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
        let app = self.app.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let (cyclic, cycle_interval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        info!("{}read| Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.read", self_id)).spawn(move || {
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            for (db_name, db_conf) in conf.dbs {
                info!("{}read| configuring DB: {:?}...", self_id, db_name);
                let db = ProfinetDb::new(app.clone(), &self_id, &db_conf);
                dbs.insert(db_name.clone(), db);
                info!("{}read| configuring DB: {:?} - ok", self_id, db_name);
            }
            let mut cycle = ServiceCycle::new(cycle_interval);
            let mut client = S7Client::new(self_id.clone(), conf.ip.clone());
            'main: while !exit.load(Ordering::SeqCst) {
                let mut error_limit = ErrorsLimit::new(3);
                let mut status = Status::Ok;
                match client.connect() {
                    Ok(_) => {
                        'read: while !exit.load(Ordering::SeqCst) {
                            cycle.start();
                            for (db_name, db) in &mut dbs {
                                trace!("{}read| DB '{}' - reading...", self_id, db_name);
                                match db.read(&client, &tx_send) {
                                    Ok(_) => {
                                        error_limit.reset();
                                        trace!("{}read| DB '{}' - reading - ok", self_id, db_name);
                                    },
                                    Err(err) => {
                                        error!("{}read| DB '{}' - reading - error: {:?}", self_id, db_name, err);
                                        if error_limit.add().is_err() {
                                            error!("{}read| DB '{}' - exceeded reading errors limit, trying to reconnect...", self_id, db_name);
                                            status = Status::Invalid;
                                            if let Err(err) = client.close() {
                                                error!("{}read| {:?}", self_id, err);
                                            };
                                            break 'read;
                                        }
                                    },
                                }
                                if exit.load(Ordering::SeqCst) {
                                    break 'main;
                                }
                            }
                            if cyclic {
                                cycle.wait();
                            }
                        }
                        if status != Status::Ok {
                            Self::yield_status(&self_id, &mut dbs, &tx_send);
                        }
                    },
                    Err(err) => {
                        debug!("{}read| Connection error: {:?}", self_id, err);
                    },
                }
                thread::sleep(Duration::from_millis(1000))
            }
        });
        info!("{}read| started", self.id);
        handle
    }
    ///
    /// Writes Point to the protocol (PROFINET device) specific address
    fn write(&mut self, tx_send: Sender<PointType>) -> Result<JoinHandle<()>, std::io::Error> {
        let self_id = self.id.clone();
        let tx_id = PointTxId::fromStr(&self_id);
        let app = self.app.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        info!("{}.write | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.write", self_id.clone())).spawn(move || {
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            let mut points: Vec<PointConfig> = vec![];
            for (db_name, db_conf) in conf.dbs {
                info!("{}.write | configuring DB: {:?}...", self_id, db_name);
                let db = ProfinetDb::new(app.clone(),&self_id, &db_conf);
                dbs.insert(db_name.clone(), db);
                info!("{}.write | configuring DB: {:?} - ok", self_id, db_name);
                points.extend(db_conf.points());
            }
            debug!("{}.write | Point configs ({}) :", self_id, points.len());
            for cfg in &points {
                println!("\t{:?}", cfg);
            }
            let points = points.iter().map(|point_conf| {
                SubscriptionCriteria::new(&point_conf.name, Cot::Act)
            }).collect::<Vec<SubscriptionCriteria>>();
            debug!("{}.write | Points subscribed on: ({})", self_id, points.len());
            for name in &points {
                println!("\t{:?}", name);
            }
            let (_, rx_recv) = services.lock().unwrap().subscribe(&conf.subscribe, &self_id, &points);
            // let mut cycle = ServiceCycle::new(cycle_interval);
            let mut client = S7Client::new(self_id.clone(), conf.ip.clone());
            'main: while !exit.load(Ordering::SeqCst) {
                let mut errors_limit = ErrorsLimit::new(3);
                thread::sleep(Duration::from_millis(50));
                match client.connect() {
                    Ok(_) => {
                        'write: while !exit.load(Ordering::SeqCst) {
                            // cycle.start();
                            match rx_recv.recv_timeout(RECV_TIMEOUT) {
                                Ok(point) => {
                                    let point_name = point.name();
                                    let point_value = point.value();
                                    let db_name = point_name.split('/').nth(2).unwrap();
                                    debug!("{}.write | DB '{}' - writing point '{}'\t({:?})...", self_id, db_name, point_name, point_value);
                                    // let dbName = point_name.split("/").skip(1).collect::<String>();
                                    match dbs.get_mut(db_name) {
                                        Some(db) => {
                                            match db.write(&client, point) {
                                                Ok(_) => {
                                                    errors_limit.reset();
                                                    debug!("{}.write | DB '{}' - writing point '{}'\t({:?}) - ok", self_id, db_name, point_name, point_value);
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
                                                            Cot::ActCon,
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
                        debug!("{}.write | Connection error: {:?}", self_id, err);
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
}
///
/// 
impl Service for ProfinetClient {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        let tx_send = self.services.lock().unwrap().get_link(&self.conf.tx);
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
/// Counts errors by calling method 'add()'
/// - returns Ok if 'limit' of errors is not exceeded
/// - returns Err if count of errors >= 'limit'
struct ErrorsLimit {
    value: usize,
    limit: usize,
}
///
/// 
impl ErrorsLimit {
    ///
    /// Creates new instance of the ErrorLimit wir the [limit]
    pub fn new(limit: usize) -> Self {
        Self { value: limit, limit }
    }
    ///
    /// Counts errors
    /// - returns Ok if 'limit' of errors is not exceeded
    /// - returns Err if count of errors >= 'limit'
    pub fn add(&mut self) -> Result<(), ()> {
        if self.value > 0 {
            self.value -= 1;
            Ok(())
        } else {
            Err(())
        }
    }
    ///
    /// Reset counter
    pub fn reset(&mut self) {
        self.value = self.limit;
    }
}
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}, time::Duration};
use indexmap::IndexMap;
use log::{debug, error, info};
use crate::{
    conf::{point_config::point_config::PointConfig, profinet_client_config::profinet_client_config::ProfinetClientConfig}, 
    core_::{constants::constants::RECV_TIMEOUT, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status}, 
    services::{profinet_client::{profinet_db::ProfinetDb, s7::s7_client::S7Client}, service::Service, services::Services, task::service_cycle::ServiceCycle},
};


///
/// Binds TCP socket server
/// Listening socket for incoming connections
/// Verified incoming connections handles in the separate thread
pub struct ProfinetClient {
    id: String,
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
    pub fn new(parent: impl Into<String>, conf: ProfinetClientConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: format!("{}/ProfinetClient({})", parent.into(), conf.name),
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
            match db.yield_status(Status::Invalid, &tx_send) {
                Ok(_) => {},
                Err(err) => {
                    error!("{}.lostConnection | send errors: \n\t{:?}", self_id, err);
                },
            };
        }
    }
}
///
/// 
impl Service for ProfinetClient {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    // 
    // fn get_link(&mut self, _name: &str) -> Sender<PointType> {
    //     panic!("{}.getLink | Does not support getLink", self.id());
    //     // match self.rx_send.get(name) {
    //     //     Some(send) => send.clone(),
    //     //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
    //     // }
    // }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let tx_send = self.services.lock().unwrap().getLink(&conf.tx);
        let tx_send_write = tx_send.clone();
        let (cyclic, cycle_interval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        info!("{}.run | Preparing Read thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run Read", self_id.clone())).spawn(move || {
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            for (db_name, db_conf) in conf.dbs {
                info!("{}.run | configuring Read DB: {:?}...", self_id, db_name);
                let db = ProfinetDb::new(&self_id, &db_conf);
                dbs.insert(db_name.clone(), db);
                info!("{}.run | configuring Read DB: {:?} - ok", self_id, db_name);
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
                                debug!("{}.run | DB '{}' - reading...", self_id, db_name);
                                match db.read(&client, &tx_send) {
                                    Ok(_) => {
                                        error_limit.reset();
                                        debug!("{}.run | DB '{}' - reading - ok", self_id, db_name);
                                    },
                                    Err(err) => {
                                        error!("{}.run | DB '{}' - reading - error: {:?}", self_id, db_name, err);
                                        if error_limit.add().is_err() {
                                            error!("{}.run | DB '{}' - exceeded reading errors limit, trying to reconnect...", self_id, db_name);
                                            status = Status::Invalid;
                                            if let Err(err) = client.close() {
                                                error!("{}.run | {:?}", self_id, err);
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
                        debug!("{}.run | Connection error: {:?}", self_id, err);
                    },
                }
                thread::sleep(Duration::from_millis(1000))
            }
        });
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        info!("{}.run | Preparing Read thread...", self_id);
        let handle_write = thread::Builder::new().name(format!("{}.run Write", self_id.clone())).spawn(move || {
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            let mut points: Vec<PointConfig> = vec![];
            for (db_name, db_conf) in conf.dbs {
                info!("{}.run | configuring Write DB: {:?}...", self_id, db_name);
                let db = ProfinetDb::new(&self_id, &db_conf);
                dbs.insert(db_name.clone(), db);
                info!("{}.run | configuring Write DB: {:?} - ok", self_id, db_name);
                points.extend(db_conf.points());
            }
            debug!("{}.run | Point configs ({}) :", self_id, points.len());
            for cfg in &points {
                println!("\t{:?}", cfg);
            }
            let points = points.iter().map(|point_conf| {
                point_conf.name.clone()
            }).collect::<Vec<String>>();
            debug!("{}.run | Points: ({})", self_id, points.len());
            for name in &points {
                println!("\t{:?}", name);
            }
            let rx_recv = services.lock().unwrap().subscribe(&conf.rx, &self_id, &points);
            let mut cycle = ServiceCycle::new(cycle_interval);
            let mut client = S7Client::new(self_id.clone(), conf.ip.clone());
            'main: while !exit.load(Ordering::SeqCst) {
                let mut errors_limit = ErrorsLimit::new(3);
                match client.connect() {
                    Ok(_) => {
                        'write: while !exit.load(Ordering::SeqCst) {
                            cycle.start();
                            match rx_recv.recv_timeout(RECV_TIMEOUT) {
                                Ok(point) => {
                                    let point_name = point.name();
                                    let db_name = point_name.split("/").skip(1).next().unwrap();
                                    debug!("{}.run | DB '{}' - writing point '{}'...", self_id, db_name, point.name());
                                    // let dbName = point.name().split("/").skip(1).collect::<String>();
                                    match dbs.get_mut(db_name) {
                                        Some(db) => {
                                            match db.write(&client, point) {
                                                Ok(_) => {
                                                    errors_limit.reset();
                                                    debug!("{}.run | DB '{}' - write - ok", self_id, db_name);
                                                },
                                                Err(err) => {
                                                    error!("{}.run | DB '{}' - write - error: {:?}", self_id, db_name, err);
                                                    if errors_limit.add().is_err() {
                                                        error!("{}.run | DB '{}' - exceeded writing errors limit, trying to reconnect...", self_id, db_name);
                                                        match tx_send_write.send(PointType::String(Point::new_string(
                                                            PointTxId::fromStr(&self_id), 
                                                            &point_name, 
                                                            format!("Error write point '': {}", err),
                                                        ))) {
                                                            Ok(_) => {},
                                                            Err(err) => {
                                                                error!("{}.run | Error sending to queue: {:?}", self_id, err);
                                                                // break 'main;
                                                            },
                                                        };
                                                        if let Err(err) = client.close() {
                                                            error!("{}.run | {:?}", self_id, err);
                                                        };
                                                        break 'write;
                                                    }
                                                },
                                            }
                                        },
                                        None => {
                                            error!("{}.run | DB '{}' - not found", self_id, db_name);
                                        },
                                    };
                                },
                                Err(err) => {
                                    match err {
                                        mpsc::RecvTimeoutError::Timeout => {},
                                        mpsc::RecvTimeoutError::Disconnected => {
                                            error!("{}.run | Error receiving from queue: {:?}", self_id, err);
                                            break 'main;
                                        },
                                    }
                                }
                            }
                            if exit.load(Ordering::SeqCst) {
                                break 'main;
                            }
                            if cyclic {
                                cycle.wait();
                            }
                        }
                        // if status != Status::Ok {
                        //     Self::yieldStatus(&selfId, &mut dbs, &txSend);
                        // }
                    },
                    Err(err) => {
                        debug!("{}.run | Connection error: {:?}", self_id, err);
                    },
                }
                thread::sleep(Duration::from_millis(1000))
            }
        });        
        info!("{}.run | started", self.id);
        handle
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}

///
/// 
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
    /// 
    pub fn add(&mut self) -> Result<(), ()> {
        if self.value > 0 {
            self.value -= 1;
            Ok(())
        } else {
            Err(())
        }
    }
    ///
    /// 
    pub fn reset(&mut self) {
        self.value = self.limit;
    }
}
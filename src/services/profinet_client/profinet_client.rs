#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}, time::Duration};
use indexmap::IndexMap;
use log::{debug, error, info};
use crate::{
    conf::profinet_client_config::profinet_client_config::ProfinetClientConfig, core_::{constants::constants::RECV_TIMEOUT, point::point_type::PointType, status::status::Status}, services::{profinet_client::{profinet_db::ProfinetDb, s7::s7_client::S7Client}, service::Service, services::Services, task::service_cycle::ServiceCycle}
};


///
/// Binds TCP socket server
/// Listening socket for incoming connections
/// Verified incoming connections handles in the separate thread
pub struct ProfinetClient {
    id: String,
    rxRecv: Vec<Receiver<PointType>>,
    rxSend: HashMap<String, Sender<PointType>>,
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
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/ProfinetClient({})", parent.into(), conf.name),
            rxRecv: vec![recv],
            rxSend: HashMap::from([(conf.rx.clone(), send)]),
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
    fn yieldStatus(selfId: &str, dbs: &mut IndexMap<String, ProfinetDb>, txSend: &Sender<PointType>) {
        for (dbName, db) in dbs {
            debug!("{}.run | DB '{}' - reading...", selfId, dbName);
            match db.yieldStatus(Status::Invalid, &txSend) {
                Ok(_) => {},
                Err(err) => {
                    error!("{}.lostConnection | send errors: \n\t{:?}", selfId, err);
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
    fn getLink(&mut self, name: &str) -> Sender<PointType> {
        // panic!("{}.getLink | Does not support getLink", self.id());
        match self.rxSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let txSend = self.services.lock().unwrap().getLink(&conf.tx);
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        info!("{}.run | Preparing Read thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.run Read", selfId.clone())).spawn(move || {
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            for (dbName, dbConf) in conf.dbs {
                info!("{}.run | configuring Read DB: {:?}...", selfId, dbName);
                let db = ProfinetDb::new(&selfId, dbConf);
                dbs.insert(dbName.clone(), db);
                info!("{}.run | configuring Read DB: {:?} - ok", selfId, dbName);
            }
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut client = S7Client::new(selfId.clone(), conf.ip.clone());
            'main: while !exit.load(Ordering::SeqCst) {
                let mut errorLimit = ErrorsLimit::new(3);
                let mut status = Status::Ok;
                match client.connect() {
                    Ok(_) => {
                        'read: while !exit.load(Ordering::SeqCst) {
                            cycle.start();
                            for (dbName, db) in &mut dbs {
                                debug!("{}.run | DB '{}' - reading...", selfId, dbName);
                                match db.read(&client, &txSend) {
                                    Ok(_) => {
                                        errorLimit.reset();
                                        debug!("{}.run | DB '{}' - reading - ok", selfId, dbName);
                                    },
                                    Err(err) => {
                                        error!("{}.run | DB '{}' - reading - error: {:?}", selfId, dbName, err);
                                        if errorLimit.add().is_err() {
                                            error!("{}.run | DB '{}' - exceeded reading errors limit, trying to reconnect...", selfId, dbName);
                                            status = Status::Invalid;
                                            client.close();
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
                            Self::yieldStatus(&selfId, &mut dbs, &txSend);
                        }
                    },
                    Err(err) => {
                        debug!("{}.run | Connection error: {:?}", selfId, err);
                    },
                }
                thread::sleep(Duration::from_millis(1000))
            }
        });
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let rxRecv = self.rxRecv.pop().unwrap();
        info!("{}.run | Preparing Read thread...", selfId);
        let handleWrite = thread::Builder::new().name(format!("{}.run Write", selfId.clone())).spawn(move || {
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            for (dbName, dbConf) in conf.dbs {
                info!("{}.run | configuring Write DB: {:?}...", selfId, dbName);
                let db = ProfinetDb::new(&selfId, dbConf);
                dbs.insert(dbName.clone(), db);
                info!("{}.run | configuring Write DB: {:?} - ok", selfId, dbName);
            }
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut client = S7Client::new(selfId.clone(), conf.ip.clone());
            'main: while !exit.load(Ordering::SeqCst) {
                let mut errorsLimit = ErrorsLimit::new(3);
                match client.connect() {
                    Ok(_) => {
                        'write: while !exit.load(Ordering::SeqCst) {
                            cycle.start();
                            match rxRecv.recv_timeout(RECV_TIMEOUT) {
                                Ok(point) => {
                                    let pointName = point.name();
                                    let dbName = pointName.split("/").skip(1).next().unwrap();
                                    debug!("{}.run | DB '{}' - writing point '{}'...", selfId, dbName, point.name());
                                    // let dbName = point.name().split("/").skip(1).collect::<String>();
                                    match dbs.get_mut(dbName) {
                                        Some(db) => {
                                            match db.write(&client, point) {
                                                Ok(_) => {
                                                    errorsLimit.reset();
                                                    debug!("{}.run | DB '{}' - write - ok", selfId, dbName);
                                                },
                                                Err(err) => {
                                                    error!("{}.run | DB '{}' - write - error: {:?}", selfId, dbName, err);
                                                    if errorsLimit.add().is_err() {
                                                        error!("{}.run | DB '{}' - exceeded writing errors limit, trying to reconnect...", selfId, dbName);
                                                        // status = Status::Invalid;
                                                        client.close();
                                                        break 'write;
                                                    }
                                                },
                                            }
                                        },
                                        None => {
                                            error!("{}.run | DB '{}' - not found", selfId, dbName);
                                        },
                                    };
                                },
                                Err(err) => {
                                    match err {
                                        mpsc::RecvTimeoutError::Timeout => {},
                                        mpsc::RecvTimeoutError::Disconnected => {
                                            error!("{}.run | Error receiving from queue: {:?}", selfId, err);
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
                        debug!("{}.run | Connection error: {:?}", selfId, err);
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
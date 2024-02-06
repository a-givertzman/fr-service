#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}, time::Duration};
use indexmap::IndexMap;
use log::{debug, error, info};
use crate::{
    conf::profinet_client_config::profinet_client_config::ProfinetClientConfig, core_::{point::point_type::PointType, status::status::Status}, services::{profinet_client::{profinet_db::ProfinetDb, s7::s7_client::S7Client}, service::Service, services::Services, task::service_cycle::ServiceCycle}
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
        info!("{}.run | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
            for (dbName, dbConf) in conf.dbs {
                info!("{}.run | configuring DB: {:?}...", selfId, dbName);
                let db = ProfinetDb::new(&selfId, dbConf);
                dbs.insert(dbName.clone(), db);
                info!("{}.run | configuring DB: {:?} - ok", selfId, dbName);
            }
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut client = S7Client::new(selfId.clone(), conf.ip.clone());
            'main: while !exit.load(Ordering::SeqCst) {
                let mut errorsLimit: i8 = 3;
                let mut status = Status::Ok;
                match client.connect() {
                    Ok(_) => {
                        'read: while !exit.load(Ordering::SeqCst) {
                            cycle.start();
                            for (dbName, db) in &mut dbs {
                                debug!("{}.run | DB '{}' - reading...", selfId, dbName);
                                match db.read(&client, &txSend) {
                                    Ok(_) => {
                                        debug!("{}.run | DB '{}' - reading - ok", selfId, dbName);
                                    },
                                    Err(err) => {
                                        error!("{}.run | DB '{}' - reading - error: {:?}", selfId, dbName, err);
                                        errorsLimit -= 1;
                                        if errorsLimit <= 0 {
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
        info!("{}.run | started", self.id);
        handle
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}
#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}, time::Duration};
use indexmap::IndexMap;
use log::{debug, error, info};
use crate::{
    services::{profinet_client::{profinet_db::ProfinetDb, s7::s7_client::S7Client}, service::Service, services::Services, task::service_cycle::ServiceCycle}, 
    conf::profinet_client_config::profinet_client_config::ProfinetClientConfig, core_::point::point_type::PointType,
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
        let mut dbs: IndexMap<String, ProfinetDb> = IndexMap::new();
        for (dbName, dbConf) in conf.dbs {
            info!("{}.run | configuring DB: {:?}...", selfId, dbName);
            let db = ProfinetDb::new(&selfId, dbConf);
            dbs.insert(dbName.clone(), db);
            info!("{}.run | configuring DB: {:?} - ok", selfId, dbName);
        }
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut client = S7Client::new(selfId.clone(), conf.ip.clone(), None);
            client.connect();
            loop {
                cycle.start();
                for (dbName, db) in &mut dbs {
                    debug!("{}.run | DB '{}' - reading...", selfId, dbName);
                    match db.read(&client, &txSend) {
                        Ok(_) => {
                            debug!("{}.run | DB '{}' - reading - ok", selfId, dbName);
                        },
                        Err(err) => {
                            error!("{}.run | DB '{}' - reading - error: {:?}", selfId, dbName, err);
                        },
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                if cyclic {
                    cycle.wait();
                }
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
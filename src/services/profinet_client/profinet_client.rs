#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}}, thread::{self, JoinHandle}, time::Duration};
use indexmap::IndexMap;
use log::info;
use crate::{
    services::{profinet_client::profinet_db::ProfinetDb, service::Service, services::Services, task::service_cycle::ServiceCycle}, 
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
    dbs: Arc<Mutex<IndexMap<String, ProfinetDb>>>,
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
            dbs: Arc::new(Mutex::new(IndexMap::new())),
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
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let dbs = self.dbs.clone();
        info!("{}.run | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            let mut cycle = ServiceCycle::new(cycleInterval);
            for (dbName, dbConf) in conf.dbs {
                info!("{}.run | configuring DB: {:?}...", selfId, dbName);
                let db = ProfinetDb::new(&selfId, dbConf);
                dbs.lock().unwrap().insert(
                    dbName.clone(),
                    db
                );
                info!("{}.run | configuring DB: {:?} - ok", selfId, dbName);
            }
            loop {
                cycle.start();

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
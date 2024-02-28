//!
//! JdsService implements behavior on the JDS communication protocol for the following kinds of requests:
//! - "Start" - after this request points transmission begins
//! - "Points" - all points configurations requested
//! - "Auth" request - authentication requested//! ```yaml
//! service JdsService Id:
//!     parameter: value    # meaning
//!     parameter: value    # meaning
//! ```
use std::{collections::HashMap, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use log::info;
use crate::{
    core_::point::point_type::PointType, 
    conf::jds_service_config::jds_service_config::JdsServiceConfig, 
    services::{service::Service, services::Services},
};


///
/// Binds TCP socket server
/// Listening socket for incoming connections
/// Verified incoming connections handles in the separate thread
pub struct JdsService {
    id: String,
    rxSend: HashMap<String, Sender<PointType>>,
    rxRecv: Vec<Receiver<PointType>>,
    conf: JdsServiceConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl JdsService {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: JdsServiceConfig, services: Arc<Mutex<Services>>) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/JdsService({})", parent.into(), conf.name),
            rxSend: HashMap::from([(conf.rx.clone(), send)]),
            rxRecv: vec![recv],
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Service for JdsService {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    // 
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        panic!("{}.get_link | Does not support get_link", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            loop {
                if exit.load(Ordering::SeqCst) {
                    break;
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
//!
//! Service implements kind of bihavior
//! Basic configuration parameters:
//! ```yaml
//! service ServiceName Id:
//!     parameter: value    # meaning
//!     parameter: value    # meaning
//! ```
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{self, JoinHandle}};
use log::info;
use crate::{
    services::{
        services::Services,
        service::service::Service,
        service::service_handles::ServiceHandles, 
    }, 
    conf::tcp_server_config::ServiceNameConfig, core_::point::point_type::PointType,
};

///
/// Binds TCP socket server
/// Listening socket for incoming connections
/// Verified incoming connections handles in the separate thread
pub struct ServiceName {
    id: String,
    conf: ServiceNameConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl ServiceName {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: ServiceNameConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: format!("{}/ServiceName({})", parent.into(), conf.name),
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Object for TcpServer {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for ServiceName {
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
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
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
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            },
            Err(err) => {
                let message = format!("{}.run | Start faled: {:#?}", self.id, err);
                warn!("{}", message);
                Err(message)
            },
        }
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}
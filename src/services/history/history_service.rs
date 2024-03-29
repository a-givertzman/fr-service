//!
//! Service sending historian events to the database
//! - Subscribe on points with history flag activated
//! - Cyclically insert accumulated points into the history database via ApiClient
//! Basic configuration parameters:
//! ```yaml
//! service History History1:
//!     cycle: 100 ms
//!     table: history
//!     suscribe:
//!         MultiQueue:
//!             Inf: *
//! ```
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread};
use log::{info, warn};
use crate::{
    core_::{object::object::Object, point::point_type::PointType}, 
    conf::history_config::HistoryConfig, 
    services::{
        services::Services,
        service::{service::Service, service_handles::ServiceHandles}, 
    } 
};

///
/// History service
/// - Subscribe on points with history flag activated
/// - Cyclically insert accumulated points into the history database via ApiClient
pub struct History {
    id: String,
    conf: HistoryConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl History {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: HistoryConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: format!("{}/History({})", parent.into(), conf.name),
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Object for History {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Debug for History {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("History")
            .field("id", &self.id)
            .finish()
    }
}
///
/// 
impl Service for History {
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
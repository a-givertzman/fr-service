//!
//! Service storing all received events in the memory,
//! - Subscribe on points by configured criteria
//! - Storing all received events on the disk if 'retain' option is true
//! - Storing received points into the HasMap by name as key
//! - Cyclically delyed stores accumulated changes to the disk if 'retain' option is true
//! Basic configuration parameters:
//! ```yaml
//! service CacheService Cache:
//!     retain: true    # true / false - enables storing cache on the disk
//!     suscribe:
//!         /App/MultiQueue: []
//! ```
use std::{collections::HashMap, fmt::Debug, hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, mpsc::{Receiver, RecvTimeoutError, Sender}, Arc, Mutex, RwLock}, thread};
use hashers::fx_hash::FxHasher;
use log::{debug, error, info, trace, warn};
use crate::{
    conf::{cache_service_config::CacheServiceConfig, point_config::name::Name}, core_::{constants::constants::RECV_TIMEOUT, object::object::Object, point::point_type::PointType}, services::{
        cache, safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services 
    } 
};

///
/// CacheService service
/// - Subscribe on points by configured criteria
/// - Storing all received events on the disk if 'retain' option is true
pub struct CacheService {
    id: String,
    name: Name,
    conf: CacheServiceConfig,
    services: Arc<Mutex<Services>>,
    cache: Arc<RwLock<HashMap<String, PointType, BuildHasherDefault<FxHasher>>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl CacheService {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: CacheServiceConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: format!("{}/CacheService({})", parent.into(), conf.name),
            name: conf.name.clone(),
            conf: conf.clone(),
            services,
            cache: Arc::new(RwLock::new(HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()))),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn subscribe(&mut self, conf: &CacheServiceConfig, services: &Arc<Mutex<Services>>) -> Receiver<PointType> {
        if conf.subscribe.is_empty() {
            panic!("{}.subscribe | Error. Subscription configuration can`t be empty: {:#?}", self.id, conf);
        } else {
            debug!("{}.subscribe | requesting points...", self.id);
            let points = services.slock().points(&self.id);
            debug!("{}.subscribe | rceived points: {:#?}", self.id, points.len());
            trace!("{}.subscribe | rceived points: {:#?}", self.id, points);
            debug!("{}.subscribe | conf.subscribe: {:#?}", self.id, conf.subscribe);
            let subscriptions = conf.subscribe.with(&points);
            trace!("{}.subscribe | subscriptions: {:#?}", self.id, subscriptions);
            if subscriptions.len() > 1 {
                panic!("{}.run | Error. Task does not supports multiple subscriptions for now: {:#?}.\n\tTry to use single subscription.", self.id, subscriptions);
            } else {
                let subscriptions_first = subscriptions.clone().into_iter().next();
                match subscriptions_first {
                    Some((service_name, Some(points))) => {
                        let (_, rx_recv) = services.slock().subscribe(
                                &service_name,
                                &self.name.join(), 
                                &points,
                            );
                        rx_recv
                    },
                    Some((_, None)) => panic!("{}.run | Error. Subscription configuration error in: {:#?}", self.id, subscriptions),
                    None => panic!("{}.run | Error. Subscription configuration error in: {:#?}", self.id, subscriptions),
                }
            }
        }
    }

}
///
/// 
impl Object for CacheService {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> crate::conf::point_config::name::Name {
        self.name.clone()
    }
}
///
/// 
impl Debug for CacheService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CacheService")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}
///
/// 
impl Service for CacheService {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let self_name = self.name.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        let cache = self.cache.clone();
        let rx_recv = self.subscribe(&conf, &services);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            'main: loop {
                match rx_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        cache.write().unwrap().insert(point.dest(), point);
                    },
                    Err(err) => {
                        match err {
                            RecvTimeoutError::Timeout => {
                                trace!("{}.run | Receive error: {:?}", self_id, err);
                            },
                            RecvTimeoutError::Disconnected => {
                                error!("{}.run | Error receiving from queue: {:?}", self_id, err);
                                break 'main;
                            },
                        }
                    },

                }
                if exit.load(Ordering::SeqCst) {break}
            }
            info!("{}.run | Exit", self_id);
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            },
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.id, err);
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
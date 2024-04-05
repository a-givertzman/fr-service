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
use std::{collections::HashMap, env, fmt::Debug, fs, hash::BuildHasherDefault, path::Path, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, RecvTimeoutError, Sender}, Arc, Mutex, RwLock}, thread};
use concat_string::concat_string;
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};
use serde::Serialize;
use crate::{
    conf::{cache_service_config::CacheServiceConfig, conf_subscribe::ConfSubscribe, point_config::name::Name}, 
    core_::{constants::constants::RECV_TIMEOUT, object::object::Object, point::point_type::PointType, status::status::Status}, 
    services::{
        cache::delay_store::DelydStore, multi_queue::subscription_criteria::SubscriptionCriteria, safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services
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
    cache: Arc<RwLock<IndexMap<String, PointType, BuildHasherDefault<FxHasher>>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl CacheService {
    ///
    /// 
    pub fn new(conf: CacheServiceConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: conf.name.join(),
            name: conf.name.clone(),
            conf: conf.clone(),
            services,
            cache: Arc::new(RwLock::new(IndexMap::with_hasher(BuildHasherDefault::<FxHasher>::default()))),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn subscribe(&mut self, conf: &CacheServiceConfig, services: &Arc<Mutex<Services>>) -> Receiver<PointType> {
        if conf.subscribe.is_empty() {
            panic!("{}.subscribe | Error. Subscription can`t be empty: {:#?}", self.id, conf.subscribe);
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
                // let subscriptions_first = subscriptions.clone().into_iter().next();
                match subscriptions.clone().into_iter().next() {
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
    ///
    /// Storing cache on the disk
    ///
    /// Writes file json map to the file:
    /// ```json
    /// {
    ///     "/path/Point.name1": 0,
    ///     "/path/Point.name2": 1,
    ///     ...
    /// }
    /// ```
    fn write<S: Serialize>(self_id: &str, name: &Name, points: S) -> Result<(), String> {
        let path = env::current_dir().unwrap();
        let mut name_join = name.join();
        let name_join = if name_join.starts_with('/') {
            name_join.replace_range(..1, "");
            name_join
        } else {
            name_join
        };
        let path = path.join("assets/cache/").join(name_join);
        let path_exists = match path.exists() {
            true => Ok(()),
            false => {
                match fs::create_dir_all(&path) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        let message = format!("{}.read | Error create path: '{:?}'\n\terror: {:?}", self_id, path, err);
                        error!("{}", message);
                        Err(message)
                    },
                }
            },
        };
        let path = path.join("cache.json");
        debug!("{}.write | path: {:?}", self_id, path);
        match path_exists {
            Ok(_) => {
                match fs::OpenOptions::new().create(true).write(true).open(&path) {
                    Ok(f) => {
                        match serde_json::to_writer_pretty(f, &points) {
                            Ok(_) => Ok(()),
                            Err(err) => {
                                let message = format!("{}.write | Error writing to file: '{:?}'\n\terror: {:?}", self_id, path, err);
                                error!("{}", message);
                                Err(message)
                            },
                        }
                    },
                    Err(err) => {
                        let message = format!("{}.write | Error open file: '{:?}'\n\terror: {:?}", self_id, path, err);
                        error!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => {
                error!("{:#?}", err);
                Err(err)
            },
        }
    }
    ///
    /// 
    fn store<T>(self_id: &str, name: &Name, points: &IndexMap<String, PointType, T>) -> Result<(), String> {
        let points: IndexMap<String, PointType> = points.into_iter().map(|(dest, point)| {
            let point = match point.clone() {
                PointType::Bool(mut point) => {
                    point.status = Status::Obsolete;
                    PointType::Bool(point)
                },
                PointType::Int(mut point) => {
                    point.status = Status::Obsolete;
                    PointType::Int(point)
                },
                PointType::Real(mut point) => {
                    point.status = Status::Obsolete;
                    PointType::Real(point)
                },
                PointType::Double(mut point) => {
                    point.status = Status::Obsolete;
                    PointType::Double(point)
                },
                PointType::String(mut point) => {
                    point.status = Status::Obsolete;
                    PointType::String(point)
                },
            };
            (dest.to_owned(), point)
        }).collect();
        Self::write(self_id, name, points)

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
        let mut dely_store = DelydStore::new(10);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            'main: loop {
                match rx_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        match cache.write() {
                            Ok(mut cache) => {
                                cache.insert(point.dest(), point);
                                if let None = dely_store.next() {
                                    if let Ok(_) = Self::store(&self_id, &self_name, &cache) {
                                        dely_store.set_stored();
                                    };
                                }
                            },
                            Err(err) => {
                                error!("{}.run | Error writing to cache: {:?}", self_id, err);
                            },
                        }
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
                if exit.load(Ordering::SeqCst) {
                    if !dely_store.stored() {
                        _ = Self::store(&self_id, &self_name, &cache.read().unwrap());
                    }
                    break;
                }
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
    //
    //
    fn gi(&self, receiver_name: &str, points: &[SubscriptionCriteria]) -> Receiver<PointType> {
        let (send, recv) = mpsc::channel();
        let self_id = self.id.clone();
        let self_cache = self.cache.clone();
        let points = points.to_owned();
        thread::spawn(move || {
            if points.is_empty() {
                match self_cache.read() {
                    Ok(cache) => {
                        for point in cache.values() {
                            match send.send(point.clone()) {
                                Ok(_) => {},
                                Err(err) => {
                                    error!("{}.gi | Send error: {:#?}", self_id, err);
                                },
                            }
                        }
    
                    },
                    Err(err) => {
                        error!("{}.gi | Error read cache: {:#?}", self_id, err);
                    },
                }
            } else {
                match self_cache.read() {
                    Ok(cache) => {
                        for point in points {
                            match cache.get(&point.destination()) {
                                Some(point) => {
                                    match send.send(point.clone()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            error!("{}.gi | Send error: {:#?}", self_id, err);
                                        },
                                    }
                                },
                                None => {
                                    error!("{}.gi | Error, requested point '{}' - not found", self_id, point.destination());
                                },
                            }
                        }
    
                    },
                    Err(err) => {
                        error!("{}.gi | Error read cache: {:#?}", self_id, err);
                    },
                }
            }
        });
        recv
    }    
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}

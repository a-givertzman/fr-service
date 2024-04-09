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
use std::{
    env, fmt::Debug, fs, hash::{BuildHasher, BuildHasherDefault}, io::Write, path::{Path, PathBuf}, sync::{atomic::{AtomicBool, Ordering}, 
    mpsc::{self, Receiver, RecvTimeoutError}, Arc, Mutex, RwLock}, thread, time::Duration
};
use concat_string::concat_string;
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};
use serde::Serialize;
use serde_json::json;
use crate::{
    conf::{cache_service_config::CacheServiceConfig, point_config::name::Name}, 
    core_::{constants::constants::RECV_TIMEOUT, object::object::Object, point::point_type::PointType, status::status::Status}, 
    services::{
        cache::delay_store::DelyStore, 
        multi_queue::subscription_criteria::SubscriptionCriteria, 
        safe_lock::SafeLock, 
        service::{service::Service, service_handles::ServiceHandles}, 
        services::Services,
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
    fn subscriptions(&mut self, conf: &CacheServiceConfig, services: &Arc<Mutex<Services>>) -> (String, Vec<SubscriptionCriteria>) {
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
                match subscriptions.clone().into_iter().next() {
                    Some((service_name, Some(points))) => {
                        (service_name, points)
                    },
                    Some((_, None)) => panic!("{}.run | Error. Subscription configuration error in: {:#?}", self.id, subscriptions),
                    None => panic!("{}.run | Error. Subscription configuration error in: {:#?}", self.id, subscriptions),
                }
            }
        }
    }
    ///
    /// Creates directiry (all necessary folders in the 'path' if not exists)
    ///  - path is relative, will be joined with current working dir
    fn create_dir(self_id: &str, path: &str) -> Result<PathBuf, String> {
        let current_dir = env::current_dir().unwrap();
        let path = current_dir.join(path);
        match path.exists() {
            true => Ok(path),
            false => {
                match fs::create_dir_all(&path) {
                    Ok(_) => Ok(path),
                    Err(err) => {
                        let message = format!("{}.read | Error create path: '{:?}'\n\terror: {:?}", self_id, path, err);
                        error!("{}", message);
                        Err(message)
                    },
                }
            },
        }
    }
    ///
    /// 
    fn read(&mut self, name: &Name) {
        let mut self_cache = self.cache.write().unwrap();
        let path = Name::new("assets/cache/", &name.join()).join().trim_start_matches('/').to_owned();
        let path = Path::new(&path).join("cache.json");
        match fs::OpenOptions::new().read(true).open(&path) {
            Ok(f) => {
                match serde_json::from_reader::<_, Vec<PointType>>(f) {
                    Ok(v) => {
                        for point in v {
                            self_cache.insert(point.dest(), point);
                        }
                    },
                    Err(err) => {
                        let message = format!("{}.read | Error open file: '{:?}'\n\terror: {:?}", self.id, path, err);
                        error!("{}", message);
                    },
                };
            },
            Err(err) => {
                let message = format!("{}.read | Error open file: '{:?}'\n\terror: {:?}", self.id, path, err);
                error!("{}", message);
            },
        }
    }
    ///
    /// Storing cache on the disk
    ///
    /// Writes file json map to the file:
    /// ```json
    /// [
    ///     {
    ///         "type": "Bool",
    ///         "value": 1,
    ///         "name": "/App/path/Point.name1",
    ///         "status": 2,
    ///         "cot": "Inf",
    ///         "timestamp": "2024-04-08T08:52:32.656576549+00:00"
    ///     },
    ///     {,
    ///     ...
    /// ]
    /// ```
    fn write<S: Serialize>(self_id: &str, name: &Name, points: Vec<S>) -> Result<(), String> {
        match Self::create_dir(self_id, Name::new("assets/cache/", &name.join()).join().trim_start_matches('/')) {
            Ok(path) => {
                let path = path.join("cache.json");
                debug!("{}.write | path: {:?}", self_id, path);
                let mut message = String::new();
                let mut cache = String::new();
                cache.push('[');
                let content: String = points.into_iter().fold(String::new(), |mut points, point| {
                    points.push_str(concat_string!("\n", json!(point).to_string(), ",").as_str());
                    points
                }).trim_end_matches(",").to_owned();
                cache.push_str(content.as_str());
                cache.push_str("\n]");
                match fs::OpenOptions::new().truncate(true) .create(true).write(true).open(&path) {
                    Ok(mut f) => {
                        match f.write_all(cache.as_bytes()) {
                            Ok(_) => {},
                            Err(err) => {
                                message = format!("{}.write | Error writing to file: '{:?}'\n\terror: {:?}", self_id, path, err);
                                error!("{}", message);
                            },
                        };
                        if message.is_empty() {Ok(())} else {Err(message)}
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
    fn store<T: BuildHasher>(self_id: &str, name: &Name, points: &IndexMap<String, PointType, T>) -> Result<(), String> {
        let points: Vec<PointType> = points.into_iter().map(|(_dest, point)| {
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
            point
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
        let (service_name, points) = self.subscriptions(&conf, &services);
        let (_, rx_recv) = services.slock().subscribe(
            &service_name,
            &self.name.join(), 
            &points,
        );
        let mut dely_store = DelyStore::new(conf.retain_delay);
        self.read(&self_name);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            'main: loop {
                match rx_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        match cache.write() {
                            Ok(mut cache) => {
                                cache.insert(point.dest(), point);
                                if dely_store.exceeded() {
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
            if let Err(err) = services.slock().unsubscribe(&service_name, &self_name.join(), &points) {
                error!("{}.run | Unsubscribe error: {:#?}", self_id, err);
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

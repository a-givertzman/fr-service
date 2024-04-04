use std::{collections::HashMap, ffi::OsStr, fs, hash::BuildHasherDefault, io::Write, path::Path, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}};
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use log::{debug, error, trace};
use serde::Serialize;
use crate::{
    core_::point::point_type::PointType, 
    conf::point_config::point_config::PointConfig, 
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, queue_name::QueueName, service::service::Service
    }
};

///
/// Holds a map of the all services in app by there names
pub struct ServicesBasic {
    id: String,
    map: HashMap<String, Arc<Mutex<dyn Service + Send>>>,
}
///
/// 
impl ServicesBasic {
    pub const API_CLIENT: &'static str = "ApiClient";
    pub const MULTI_QUEUE: &'static str = "MultiQueue";
    pub const PROFINET_CLIENT: &'static str = "ProfinetClient";
    pub const TASK: &'static str = "Task";
    pub const TCP_CLIENT: &'static str = "TcpClient";
    pub const TCP_SERVER: &'static str = "TcpServer";
    ///
    /// Creates new instance of the ServicesBasic
    pub fn new(parent: impl Into<String>) -> Self {
        let self_id = format!("{}/ServicesBasic", parent.into());
        Self {
            id: self_id.clone(),
            map: HashMap::new(),
        }
    }
    ///
    /// 
    pub fn all(&self) -> HashMap<String, Arc<Mutex<dyn Service + Send>>> {
        self.map.clone()
    }
    ///
    /// 
    pub fn insert(&mut self, id:&str, service: Arc<Mutex<dyn Service + Send>>) {
        if self.map.contains_key(id) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.id, id);
        }
        self.map.insert(id.to_string(), service);
    }
    ///
    /// Returns Service
    pub fn get(&self, name: &str) -> Arc<Mutex<dyn Service>> {
        match self.map.get(name) {
            Some(srvc) => srvc.clone(),
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// Returns copy of the Sender - service's incoming queue
    pub fn get_link(&self, name: &str) -> Sender<PointType> {
        let name = QueueName::new(name);
        match self.map.get(name.service()) {
            Some(srvc) => srvc.lock().unwrap().get_link(name.queue()),
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// Returns Receiver
    /// - service - the name of the service to subscribe on
    pub fn subscribe(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> (Sender<PointType>, Receiver<PointType>) {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.subscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.lock().unwrap().subscribe(receiver_id, points);
                debug!("{}.subscribe | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns ok if subscription extended sucessfully
    /// - service - the name of the service to extend subscribtion on
    pub fn extend_subscription(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        // panic!("{}.extend_subscription | Not implemented yet", self.id);
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.extend_subscription | Lock service '{:?}'...", self.id, service);
                let r = srvc.lock().unwrap().extend_subscription(receiver_id, points);
                debug!("{}.extend_subscription | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    fn unsubscribe(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.unsubscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.lock().unwrap().unsubscribe(receiver_id, points);
                debug!("{}.unsubscribe | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns list of point configurations over the all services
    pub fn points(&self) -> Vec<PointConfig> {
        let mut points = vec![];
        for service in self.map.values() {
        debug!("{}.points | service: '{:?}'", self.id, service.lock().unwrap().id());
        let mut service_points = service.lock().unwrap().points();
            points.append(&mut service_points);
        };
        trace!("{}.points | points: '{:#?}'", self.id, points);
        points
    }
}


///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    services_basic: Arc<Mutex<ServicesBasic>>,
    retain_points: Arc<Mutex<RetainPointId>>,
}
///
/// 
impl Services {
    ///
    /// Creates new instance of the Services
    pub fn new(parent: impl Into<String>) -> Self {
        let self_id = format!("{}/Services", parent.into());
        let services_basic = Arc::new(Mutex::new(ServicesBasic::new(&self_id)));
        let retain_points = Arc::new(Mutex::new(RetainPointId::new(&self_id, "assets/retain_points.json", services_basic.clone())));
        Self {
            id: self_id.clone(),
            services_basic,
            retain_points,
        }
    }
    ///
    /// 
    pub fn all(&self) -> HashMap<String, Arc<Mutex<dyn Service + Send>>> {
        debug!("{}.all | Lock services_basic...", self.id);
        let all = self.services_basic.lock().unwrap().all();
        debug!("{}.all | Lock services_basic - ok", self.id);
        all
    }
    ///
    /// 
    pub fn insert(&mut self, id:&str, service: Arc<Mutex<dyn Service + Send>>) {
        debug!("{}.insert | Lock services_basic...", self.id);
        let insert = self.services_basic.lock().unwrap().insert(id, service);
        debug!("{}.insert | Lock services_basic - ok", self.id);
        insert
    }
    ///
    /// Returns Service
    pub fn get(&self, name: &str) -> Arc<Mutex<dyn Service>> {
        debug!("{}.get | Lock services_basic...", self.id);
        let get = self.services_basic.lock().unwrap().get(name);
        debug!("{}.get | Lock services_basic - ok", self.id);
        get
    }
    ///
    /// Returns copy of the Sender - service's incoming queue
    pub fn get_link(&self, name: &str) -> Sender<PointType> {
        debug!("{}.get_link | Lock services_basic...", self.id);
        let get_link = self.services_basic.lock().unwrap().get_link(name);
        debug!("{}.get_link | Lock services_basic - ok", self.id);
        get_link
    }
    ///
    /// Returns Receiver
    /// - service - the name of the service to subscribe on
    pub fn subscribe(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> (Sender<PointType>, Receiver<PointType>) {
        debug!("{}.subscribe | Lock services_basic...", self.id);
        let subscribe = self.services_basic.lock().unwrap().subscribe(service, receiver_id, points);
        debug!("{}.subscribe | Lock services_basic - ok", self.id);
        subscribe
    }
    ///
    /// Returns ok if subscription extended sucessfully
    /// - service - the name of the service to extend subscribtion on
    pub fn extend_subscription(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        debug!("{}.extend_subscription | Lock services_basic...", self.id);
        let extend_subscription = self.services_basic.lock().unwrap().extend_subscription(service, receiver_id, points);
        debug!("{}.extend_subscription | Lock services_basic - ok", self.id);
        extend_subscription
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    fn unsubscribe(&mut self, service: &str, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        debug!("{}.unsubscribe | Lock services_basic...", self.id);
        let unsubscribe = self.services_basic.lock().unwrap().unsubscribe(service, receiver_id, points);
        debug!("{}.unsubscribe | Lock services_basic - ok", self.id);
        unsubscribe
    }
    ///
    /// Returns list of point configurations over the all services
    pub fn points(&mut self) -> Vec<PointConfig> {
        debug!("{}.all | Lock services_basic...", self.id);
        let points = self.services_basic.lock().unwrap().points();
        let points = self.retain_points.lock().unwrap().points(points);
        debug!("{}.all | Lock services_basic - ok", self.id);
        // let mut points = vec![];
        // for service in self.map.values() {
        // debug!("{}.points | service: '{:?}'", self.id, service.lock().unwrap().id());
        // let mut service_points = service.lock().unwrap().points();
        //     points.append(&mut service_points);
        // };
        // trace!("{}.points | points: '{:#?}'", self.id, points);
        points
    }
}

///
/// Stores unique Point ID in the json file
struct RetainPointId {
    id: String,
    path: String,
    services: Arc<Mutex<ServicesBasic>>,
    cache: Vec<PointConfig>,
}
///
/// 
impl RetainPointId {
    ///
    /// Creates new instance of the RetainPointId
    ///  - parent - the name of the parent object
    ///  - services - Services thread safe mutable reference
    ///  - path - path to the file, where point id's will be stored
    pub fn new(parent: &str, path: &str, services: Arc<Mutex<ServicesBasic>>) -> Self {
        Self {
            id: format!("{}/RetainPointId", parent),
            path: path.to_owned(),
            services,
            cache: vec![],
        }
    }
    ///
    /// 
    pub fn points<'a>(&mut self, points: Vec<PointConfig>) -> Vec<PointConfig> {
        if self.cache.is_empty() {
            let mut update_retained = false;
            // let json_value = self.read(self.path.clone());
            let mut retained = self.read(self.path.clone());
            debug!("{}.points | retained: {:#?}", self.id, retained);
            // debug!("{}.points | Lock services_basic ...", self.id);
            // let points = self.services.lock().unwrap().points();
            // debug!("{}.points | Lock services_basic - ok", self.id);
            for point in points {
                debug!("{}.points | point: {}...", self.id, point.name);
                let cached = retained.get(&point.name);
                let id = match cached {
                    Some(id) => {
                        debug!("{}.points |     found: {}", self.id, id);
                        *id
                    },
                    None => {
                        debug!("{}.points |     not found, calculating max...",self.id);
                        update_retained = true;
                        let id = retained
                            .values()
                            .max()
                            .map_or(0, |id| id + 1);
                        retained.insert(point.name.clone(), id);
                        debug!("{}.points |     calculated: {}", self.id, id);
                        id
                    },
                };
                self.cache.push(
                    PointConfig {
                        id,
                        name: point.name,
                        _type: point._type,
                        history: point.history,
                        alarm: point.alarm,
                        address: point.address,
                        filters: point.filters,
                        comment: point.comment,
                    }
                );
            }
            if update_retained {
                self.write(&self.path, retained).unwrap();
            }
        }
        self.cache.clone()
    }
    ///
    /// Reads file contains json map:
    /// ```json
    /// {
    ///     "/path/Point.name1": 0,
    ///     "/path/Point.name2": 1,
    ///     ...
    /// }
    /// ```
    fn read<P: AsRef<Path> + AsRef<OsStr> + std::fmt::Display>(&self, path: P) -> IndexMap<String, usize, BuildHasherDefault<FxHasher>> {
        // Self::create_path_if_not_exitst(&self.id, &path).unwrap();
        let mut retained = IndexMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
        match fs::read_to_string(&path) {
            Ok(json_string) => {
                match serde_json::from_str(&json_string) {
                    Ok(config) => {
                        let config: serde_json::Map<String, serde_json::Value> = config;
                        for (key, value) in config {
                            match value.as_u64() {
                                Some(value) => {
                                    retained.insert(key, value as usize);
                                },
                                None => {
                                    error!("{}.read | Error parsing usize value in pair: {}: {:?}", self.id, key, value);
                                },
                            }
                        };
                    },
                    Err(err) => {
                        error!("{}.read | Error in config: {:?}\n\terror: {:?}", self.id, json_string, err);
                    },
                }
            },
            Err(err) => {
                error!("{}.read | File {} reading error: {:?}", self.id, path, err);
            },
        };
        retained
    }
    ///
    /// Writes file json map to the file:
    /// ```json
    /// {
    ///     "/path/Point.name1": 0,
    ///     "/path/Point.name2": 1,
    ///     ...
    /// }
    /// ```
    fn write<P: AsRef<Path> + AsRef<OsStr> + std::fmt::Display, S: Serialize>(&self, path: P, points: S) -> Result<(), String> {
        // let points: HashMap<String, usize> = points.into_iter().map(|point| {
        //     (point.name.clone(), point.id)
        // }).collect();
        match fs::OpenOptions::new().create(true).write(true).open(&path) {
            Ok(f) => {
                match serde_json::to_writer_pretty(f, &points) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(format!("{}.read | Error writing to file: '{}'\n\terror: {:?}", self.id, path, err)),
                }
            },
            Err(err) => {
                Err(format!("{}.read | Error open file: '{}'\n\terror: {:?}", self.id, path, err))
            },
        }
    }
}
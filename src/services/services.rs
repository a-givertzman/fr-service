use std::{collections::HashMap, ffi::OsStr, fmt::Debug, fs, hash::BuildHasherDefault, path::Path, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}};
use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use concat_string::concat_string;
use log::{debug, error, trace, warn};
use serde::{Deserialize, Serialize};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_type::PointConfigType}, core_::point::point_type::PointType,
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, 
        queue_name::QueueName, 
        safe_lock::SafeLock, 
        service::service::Service,
    }
};
///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    map: HashMap<String, Arc<Mutex<dyn Service + Send>>>,
    retain: RetainPointId,
}
///
/// 
impl Services {
    pub const API_CLIENT: &'static str = "ApiClient";
    pub const MULTI_QUEUE: &'static str = "MultiQueue";
    pub const PROFINET_CLIENT: &'static str = "ProfinetClient";
    pub const TASK: &'static str = "Task";
    pub const TCP_CLIENT: &'static str = "TcpClient";
    pub const TCP_SERVER: &'static str = "TcpServer";
    pub const PRODUCER_SERVICE: &'static str = "ProducerService";
    ///
    /// Creates new instance of the Services
    pub fn new(parent: impl Into<String>) -> Self {
        let self_id = format!("{}/Services", parent.into());
        Self {
            id: self_id.clone(),
            map: HashMap::new(),
            retain: RetainPointId::new(&self_id, "assets/retain_points.json"),
        }
    }
    ///
    /// Returns all holding services in the map<service id, service reference>
    pub fn all(&self) -> HashMap<String, Arc<Mutex<dyn Service + Send>>> {
        self.map.clone()
    }
    ///
    /// 
    pub fn insert(&mut self, service: Arc<Mutex<dyn Service + Send>>) {
        let name = service.slock().name().join();
        if self.map.contains_key(&name) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.id, name);
        }
        self.map.insert(name, service);
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
    pub fn get_link(&self, name: &str) -> Result<Sender<PointType>, String> {
        let name = QueueName::new(name);
        match self.map.get(name.service()) {
            Some(srvc) => Ok(srvc.slock().get_link(name.queue())),
            None => Err(format!("{}.get | service '{:?}' - not found", self.id, name)),
        }
    }
    ///
    /// Returns Receiver
    /// - service - the name of the service to subscribe on
    pub fn subscribe(&mut self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> (Sender<PointType>, Receiver<PointType>) {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.subscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.slock().subscribe(receiver_name, points);
                debug!("{}.subscribe | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns ok if subscription extended sucessfully
    /// - service - the name of the service to extend subscribtion on
    pub fn extend_subscription(&mut self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        // panic!("{}.extend_subscription | Not implemented yet", self.id);
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.extend_subscription | Lock service '{:?}'...", self.id, service);
                let r = srvc.slock().extend_subscription(receiver_name, points);
                debug!("{}.extend_subscription | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    fn unsubscribe(&mut self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.unsubscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.slock().unsubscribe(receiver_name, points);
                debug!("{}.unsubscribe | Lock service '{:?}' - ok", self.id, service);
                r
            },
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns list of point configurations over the all services
    pub fn points(&mut self, requester_name: &str) -> Vec<PointConfig> {
        debug!("{}.points | requester_id: '{}'", self.id, requester_name);
        let mut points = vec![];
        for (service_id, service) in &self.map {
            if service_id != requester_name {
                // debug!("{}.points | Lock service: '{}'...", self.id, service_id);
                let mut service_points = service.slock().points();
                // debug!("{}.points | Lock service: '{}' - ok", self.id, service_id);
                points.append(&mut service_points);
            }
        };
        let points = self.retain.points(points);
        debug!("{}.points | points: '{:#?}'", self.id, points.len());
        trace!("{}.points | points: '{:#?}'", self.id, points);
        points
    }
}
///
/// 
impl Debug for Services {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Services")
            .field("id", &self.id)
            .finish()
    }
}
///
/// Stores unique Point ID in the json file
#[derive(Debug)]
struct RetainPointId {
    id: String,
    path: String,
    cache: Vec<PointConfig>,
    api_address: String, 
    api_auth_token: String, 
    api_database: String, 
}
///
/// 
impl RetainPointId {
    ///
    /// Creates new instance of the RetainPointId
    ///  - parent - the name of the parent object
    ///  - services - Services thread safe mutable reference
    ///  - path - path to the file, where point id's will be stored
    pub fn new(parent: &str, path: &str) -> Self {
        Self {
            id: format!("{}/RetainPointId", parent),
            path: path.to_owned(),
            cache: vec![],
            api_address: "0.0.0.0:8080".to_owned(),
            api_auth_token: "123!@#".to_owned(),
            api_database: "crane_data_server".to_owned(),
        }
    }
    ///
    /// 
    pub fn points(&mut self, points: Vec<PointConfig>) -> Vec<PointConfig> {
        if self.cache.is_empty() {
            let mut update_retained = false;
            let mut retained: HashMap<String, RetainedPointConfig, BuildHasherDefault<FxHasher>> = self.read(self.path.clone());
            debug!("{}.points | retained: {:#?}", self.id, retained);
            for mut point in points {
                debug!("{}.points | point: {}...", self.id, point.name);
                let cached = retained.get(&point.name);
                let id = match cached {
                    Some(conf) => {
                        debug!("{}.points |     found: {}", self.id, conf.id);
                        conf.id
                    },
                    None => {
                        debug!("{}.points |     not found, calculating max...",self.id);
                        update_retained = true;
                        let id = retained
                            .values()
                            .map(|conf| conf.id)
                            .max()
                            .map_or(0, |id| id + 1);
                        point.id = id;
                        retained.insert(point.name.clone(), RetainedPointConfig { id: point.id, name: point.name.clone(), _type: point._type.clone() });
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
                self.write(&self.path, &retained).unwrap();
                self.sql_write(&retained)
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
    fn read<P: AsRef<Path> + AsRef<OsStr> + std::fmt::Display>(&self, path: P) -> HashMap<String, RetainedPointConfig, BuildHasherDefault<FxHasher>> {
        match fs::read_to_string(&path) {
            Ok(json_string) => {
                match serde_json::from_str(&json_string) {
                    Ok(config) => {
                        return config
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
        HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default())
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
    ///
    /// 
    fn sql_write(&self, retained: &HashMap<String, RetainedPointConfig, BuildHasherDefault<FxHasher>>) {
        let api_keep_alive = true;
        let sql_keep_alive = true;
        let mut request = ApiRequest::new(
            &self.id, 
            &self.api_address, 
            &self.api_auth_token, 
            ApiQuery::new(
                ApiQueryKind::Sql(ApiQuerySql::new(&self.api_database, "select 1;")), 
                sql_keep_alive,
            ),
            api_keep_alive, 
            false,
        );
        _ = self.sql_request(&mut request, "truncate public.tags;", api_keep_alive);
        for (_, point) in retained {
            let sql = format!("insert into public.tags (id, type, name) values ({},'{:?}','{}');", point.id, point._type, point.name);
            _ = self.sql_request(&mut request, &sql, api_keep_alive);
        }
    }
    ///
    /// 
    fn sql_request(&self, request: &mut ApiRequest, sql: &str, keep_alive: bool) -> Result<ApiReply, String> {
        let query = ApiQuery::new(
            ApiQueryKind::Sql(ApiQuerySql::new(&self.api_database, sql)),
            true,
        );
        match request.fetch(&query, keep_alive) {
            Ok(reply) => {
                if log::max_level() > log::LevelFilter::Debug {
                    let reply_str = std::str::from_utf8(&reply).unwrap();
                    debug!("{}.send | reply str: {:?}", &self.id, reply_str);
                }
                match serde_json::from_slice(&reply) {
                    Ok(reply) => Ok(reply),
                    Err(err) => {
                        let reply = match std::str::from_utf8(&reply) {
                            Ok(reply) => reply.to_string(),
                            Err(err) => concat_string!(self.id, ".send | Error parsing reply to utf8 string: ", err.to_string()),
                        };
                        let message = concat_string!(self.id, ".send | Error parsing API reply: {:?} \n\t reply was: {:?}", err.to_string(), reply);
                        warn!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => {
                let message = concat_string!(self.id, ".send | Error sending API request: {:?}", err);
                warn!("{}", message);
                Err(message)
            },
        }
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetainedPointConfig {
    pub id: usize,
    pub name: String,
    #[serde(rename = "type")]
    #[serde(alias = "type", alias = "Type")]
    pub _type: PointConfigType,
}

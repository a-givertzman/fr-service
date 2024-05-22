use std::{collections::HashMap, fmt::Debug, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}};
use log::{debug, trace};
use crate::{
    conf::point_config::point_config::PointConfig, core_::point::point_type::PointType,
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, 
        queue_name::QueueName, 
        safe_lock::SafeLock, 
        service::service::Service,
        retain_point_id::RetainPointId,
    }
};
///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    map: HashMap<String, Arc<Mutex<dyn Service + Send>>>,
    retain: RetainPointId,
}
//
//
impl Services {
    pub const API_CLIENT: &'static str = "ApiClient";
    pub const MULTI_QUEUE: &'static str = "MultiQueue";
    pub const PROFINET_CLIENT: &'static str = "ProfinetClient";
    pub const TASK: &'static str = "Task";
    pub const TCP_CLIENT: &'static str = "TcpClient";
    pub const TCP_SERVER: &'static str = "TcpServer";
    pub const PRODUCER_SERVICE: &'static str = "ProducerService";
    pub const CACHE_SERVICE: &'static str = "CacheService";
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
    /// Inserts a new service into the collection
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
            }
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
            }
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    pub fn unsubscribe(&mut self, service: &str, receiver_name: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.unsubscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.slock().unsubscribe(receiver_name, points);
                debug!("{}.unsubscribe | Lock service '{:?}' - ok", self.id, service);
                r
            }
            None => panic!("{}.get | service '{:?}' - not found", self.id, service),
        }
    }
    ///
    /// Returns list of point configurations over the all services
    ///  - requester_name - Service name !!!
    pub fn points(&mut self, requester_name: &str) -> Vec<PointConfig> {
        let points = if !self.retain.is_cached() {
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
            self.retain.points(points)
        } else {
            self.retain.points(vec![])
        };
        debug!("{}.points | points: '{:#?}'", self.id, points.len());
        trace!("{}.points | points: '{:#?}'", self.id, points);
        points
    }
    ///
    /// Sends the General Interogation request to all services
    pub fn gi(&self, _service: &str, _points: &[SubscriptionCriteria]) -> Receiver<PointType> {
        panic!("{}.gi | Not implemented yet", self.id);
    }
}
//
// 
impl Debug for Services {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Services")
            .field("id", &self.id)
            .finish()
    }
}

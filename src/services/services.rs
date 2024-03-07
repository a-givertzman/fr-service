use std::{collections::HashMap, sync::{Arc, Mutex, mpsc::{Sender, Receiver}}};
use log::debug;
use crate::{
    core_::point::point_type::PointType, 
    conf::point_config::point_config::PointConfig,
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria,
        queue_name::QueueName,
        service::service::Service,
    }
};

///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    map: HashMap<String, Arc<Mutex<dyn Service + Send>>>,
}
///
/// 
impl Services {
    ///
    /// Creates new instance of the ReatinBuffer
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            id: format!("{}/Services", parent.into()),
            map: HashMap::new(),
        }
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
    pub fn subscribe(&mut self, service: &str, receiver_id: &str, points: &Vec<SubscriptionCriteria>) -> Receiver<PointType> {
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
    pub fn extend_subscription(&mut self, service: &str, receiver_id: &str, points: &Vec<SubscriptionCriteria>) -> Result<(), String> {
        panic!("{}.extend_subscription | Not implemented yet", self.id);
    }
    ///
    /// Returns ok if subscription removed sucessfully
    /// - service - the name of the service to unsubscribe on
    fn unsubscribe(&mut self, service: &str, receiver_id: &str, points: &Vec<SubscriptionCriteria>) -> Result<(), String> {
        panic!("{}.extend_subscription | Not implemented yet", self.id);
    }
    ///
    /// Returns list of point configurations over the all services
    pub fn points(&self) -> Vec<PointConfig> {
        let mut points = vec![];
        for service in self.map.values() {
            let mut service_points = service.lock().unwrap().points();
            points.append(&mut service_points);
        };
        points
    }
    // ///
    // /// 
    // pub fn get_mut(&mut self, name: &str) -> Arc<Mutex<dyn Service>> {
    //     match self.map.get_mut(name) {
    //         Some(srvc) => srvc,
    //         None => panic!("{}.get | service '{:?}' - not found", self.id, name),
    //     }
    // }
}
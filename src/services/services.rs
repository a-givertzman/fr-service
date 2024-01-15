#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex, mpsc::{Sender, Receiver}}};

use log::debug;

use crate::{core_::point::point_type::PointType, conf::point_config::point_config::PointConfig};

use super::{service::Service, queue_name::QueueName};

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
    /// Returns Service's link
    pub fn get(&self, name: &str) -> Arc<Mutex<dyn Service>> {
        match self.map.get(name) {
            Some(srvc) => srvc.clone(),
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// Returns copy of the Sender - service's incoming queue
    pub fn getLink(&self, name: &str) -> Sender<PointType> {
        let name = QueueName::new(name);
        match self.map.get(name.service()) {
            Some(srvc) => srvc.lock().unwrap().getLink(name.queue()),
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// Returns Receiver
    pub fn subscribe(&mut self, service: &str, receiverId: &str, points: &Vec<String>) -> Receiver<PointType> {
        match self.map.get(service) {
            Some(srvc) => {
                debug!("{}.subscribe | Lock service '{:?}'...", self.id, service);
                let r = srvc.lock().unwrap().subscribe(receiverId, points);
                debug!("{}.subscribe | Lock service '{:?}' - ok", self.id, service);
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
            let mut servicePoints = service.lock().unwrap().points();
            points.append(&mut servicePoints);
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
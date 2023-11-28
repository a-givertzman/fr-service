#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{Arc, Mutex}};

use super::service::Service;

///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    map: HashMap<String, Arc<Mutex<dyn Service>>>,
}
///
/// 
impl Services {
    ///
    /// Creates new instance of the ReatinBuffer
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            id: format!("{}/RetainBuffer({})", parent.into(), "Services"),
            map: HashMap::new(),
        }
    }
    ///
    /// 
    pub fn insert(&mut self, id:&str, service: Arc<Mutex<dyn Service>>) {
        if self.map.contains_key(id) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.id, id);
        }
        self.map.insert(id.to_string(), service);
    }
    ///
    /// 
    pub fn get(&self, name: &str) -> Arc<Mutex<dyn Service>> {
        match self.map.get(name) {
            Some(srvc) => srvc.clone(),
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
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
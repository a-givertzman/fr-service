#![allow(non_snake_case)]

use std::collections::HashMap;

use super::service::Service;

///
/// Holds a map of the all services in app by there names
pub struct Services {
    id: String,
    map: HashMap<String, Box<dyn Service>>,
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
    pub fn insert(&mut self, id:&str, service: Box<dyn Service>) {
        if self.map.contains_key(id) {
            panic!("{}.insert | Duplicated service name '{:?}'", self.id, id);
        }
        self.map.insert(id.to_string(), service);
    }
    ///
    /// 
    pub fn get(&self, name: &str) -> &Box<dyn Service> {
        match self.map.get(name) {
            Some(srvc) => srvc,
            None => panic!("{}.get | service '{:?}' - not found", self.id, name),
        }
    }
}
#![allow(non_snake_case)]

use std::sync::{Arc, Mutex};

use crate::{services::{services::Services, service::Service}, conf::multi_queue_config::MultiQueueConfig};

///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
struct MultiQueue {

}
///
/// 
impl MultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: MultiQueueConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {

        }
    }
}
///
/// 
impl Service for MultiQueue {
    //
    //
    fn getLink(&self, name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        todo!()
    }
    //
    //
    fn run(&mut self) {
        todo!()
    }
    //
    //
    fn exit(&self) {
        todo!()
    }
}
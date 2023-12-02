#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, mpsc::Sender}, collections::HashMap};

use crate::{services::{services::Services, service::Service}, conf::multi_queue_config::MultiQueueConfig, core_::point::point_type::PointType};

///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
struct MultiQueue {
    subscriptions: Subscriptions,
}
///
/// 
impl MultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: MultiQueueConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            subscriptions: Subscriptions::new(),
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

///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
struct Subscriptions {
    byPoints: HashMap<String, HashMap<String, Sender<PointType>>>,
}
///
/// 
impl Subscriptions {
    ///
    /// Creates new instance of Subscriptions
    pub fn new() -> Self {
        Self {
            byPoints: HashMap::new(),
        }
    }
    ///
    /// Adds subscription to Point ID with receiver ID
    pub fn add(&mut self, receiverId: String, pointId: String, sender: Sender<PointType>) {
        if ! self.byPoints.contains_key(&pointId) {
            self.byPoints.insert(
                pointId.clone(),
                HashMap::new(),
            );
        };
        match self.byPoints.get_mut(&pointId) {
            Some(senders) => {
                senders.insert(
                    receiverId,
                    sender,
                );
            },
            None => {},
        }
    }
    ///
    /// Returns map of Senders
    pub fn get(&self, pointId: String) -> Option<&HashMap<String, Sender<PointType>>> {
        self.byPoints.get(&pointId)
    }
    ///
    /// Removes single subscription by Point Id & receiver ID
    pub fn remove(&mut self, receiverId: String, pointId: String) -> Option<()> {
        match self.byPoints.get_mut(&pointId) {
            Some(senders) => {
                match senders.remove(&receiverId) {
                    Some(_) => Some(()),
                    None => None,
                }
            },
            None => None,
        }
    }
}
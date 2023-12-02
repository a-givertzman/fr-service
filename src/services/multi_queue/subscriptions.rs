#![allow(non_snake_case)]

use std::{collections::{HashMap, hash_map::Iter}, sync::mpsc::Sender};

use crate::core_::point::point_type::PointType;


///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
pub struct Subscriptions {
    id: String,
    multicast: HashMap<String, HashMap<String, Sender<PointType>>>,
    broadcast: HashMap<String, Sender<PointType>>,
    empty: HashMap<String, Sender<PointType>>,
}
///
/// 
impl Subscriptions {
    ///
    /// Creates new instance of Subscriptions
    pub fn new(parent: impl Into<String>, ) -> Self {
        Self {
            id: format!("{}/Subscriptions", parent.into()),
            multicast: HashMap::new(),
            broadcast: HashMap::new(),
            empty: HashMap::new(),
        }
    }
    ///
    /// Adds subscription to Point ID with receiver ID
    pub fn addMulticast(&mut self, receiverId: &str, pointId: &str, sender: Sender<PointType>) {
        if ! self.multicast.contains_key(pointId) {
            self.multicast.insert(
                pointId.to_string(),
                HashMap::new(),
            );
        };
        match self.multicast.get_mut(pointId) {
            Some(senders) => {
                senders.insert(
                    receiverId.to_string(),
                    sender,
                );
            },
            None => {},
        }
    }
    ///
    /// 
    pub fn addBroadcast(&mut self, receiverId: &str, sender: Sender<PointType>) {
        self.broadcast.insert(
            receiverId.to_string(),
            sender,
        );
    }
    ///
    /// Returns map of Senders
    pub fn iter(&self, pointId: &str) -> impl Iterator<Item = (&String, &Sender<PointType>)> {   //HashMap<String, Sender<PointType>>
        match self.multicast.get(pointId) {
            Some(multicast) => {
                multicast.iter().chain(&self.broadcast)
            },
            None => {
                self.broadcast.iter().chain(&self.empty)
            },
        }
    }
    ///
    /// Removes single subscription by Point Id & receiver ID
    pub fn remove(&mut self, receiverId: &str, pointId: &str) -> Result<(), String> {
        match self.multicast.get_mut(pointId) {
            Some(senders) => {
                match senders.remove(receiverId) {
                    Some(_) => Ok(()),
                    None => Err(format!("{}.run | subscription '{}', receiver '{}' - not found", self.id, pointId, receiverId)),
                }
            },
            None => Err(format!("{}.run | subscription '{}' - not found", self.id, pointId)),
        }
    }
}

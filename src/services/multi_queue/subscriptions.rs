#![allow(non_snake_case)]

use std::{collections::HashMap, sync::mpsc::Sender};

use log::{warn, trace};

use crate::core_::point::point_type::PointType;

///
/// Unique id of the service receiving the Point's by the subscription
/// This id used to identify the service produced the Points. 
/// To avoid send back self produced Point's.
type ReceiverId = usize;
///
/// Destination of the point,
/// Currently it's just a concat of the Point.cot & Point.id 
type PointDest = String; 

///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
#[derive(Debug, Clone)]
pub struct Subscriptions {
    id: String,
    multicast: HashMap<PointDest, HashMap<ReceiverId, Sender<PointType>>>,
    broadcast: HashMap<ReceiverId, Sender<PointType>>,
    empty: HashMap<ReceiverId, Sender<PointType>>,
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
    pub fn addMulticast(&mut self, receiverId: usize, destination: &str, sender: Sender<PointType>) {
        if ! self.multicast.contains_key(destination) {
            self.multicast.insert(
                destination.to_string(),
                HashMap::new(),
            );
        };
        match self.multicast.get_mut(destination) {
            Some(senders) => {
                senders.insert(
                    receiverId,
                    sender,
                );
            },
            None => {
                warn!("{}.addMulticast | Subscription '{}' - not found", self.id, destination);
            },
        }
    }
    ///
    /// 
    pub fn addBroadcast(&mut self, receiverId: usize, sender: Sender<PointType>) {
        self.broadcast.insert(
            receiverId,
            sender,
        );
    }
    ///
    /// Returns map of Senders
    pub fn iter(&self, pointId: &str) -> impl Iterator<Item = (&usize, &Sender<PointType>)> {
        match self.multicast.get(pointId) {
            Some(multicast) => {
                trace!("{}.iter | \n\t Multicast: {:?} \n\t Broadcast: {:?}", self.id, multicast, self.broadcast);
                multicast.iter().chain(&self.broadcast)
            },
            None => {
                trace!("{}.iter | \n\t Broadcast: {:?}", self.id, self.broadcast);
                self.broadcast.iter().chain(&self.empty)
            },
        }
    }
    ///
    /// Removes single subscription by Point Id for receiver ID
    pub fn remove(&mut self, receiverId: &usize, pointId: &str) -> Result<(), String> {
        match self.multicast.get_mut(pointId) {
            Some(senders) => {
                match senders.remove(receiverId) {
                    Some(_) => Ok(()),
                    None => Err(format!("{}.run | Subscription '{}', receiver '{}' - not found", self.id, pointId, receiverId)),
                }
            },
            None => Err(format!("{}.run | Subscription '{}' - not found", self.id, pointId)),
        }
    }
    ///
    /// Removes all subscriptions for receiver ID
    pub fn removeAll(&mut self, receiverId: &usize) -> Result<(), String> {
        let mut changed = false;
        let mut messages = vec![];
        let keys: Vec<String> = self.multicast.keys().map(|v| v.clone()).collect();
        for pointId in keys {
            match self.multicast.get_mut(&pointId) {
                Some(senders) => {
                    match senders.remove(receiverId) {
                        Some(_) => {
                            changed = changed | true;
                        },
                        None => {
                            messages.push(format!("{}.run | Multicast Subscription '{}', receiver '{}' - not found", self.id, pointId, receiverId));
                        },
                    }
                },
                None => {
                    messages.push(format!("{}.run | Multicast Subscription '{}' - not found", self.id, pointId));
                }
            }
        }
        match self.broadcast.remove(receiverId) {
            Some(_) => {
                changed = changed | true;
            },
            None => {
                messages.push(format!("{}.run | Broadcast Subscription by receiver '{}' - not found", self.id, receiverId));
            },
        }
        if changed {
            Ok(())
        } else {
            Err(messages.join("\n"))
        }
    }
}

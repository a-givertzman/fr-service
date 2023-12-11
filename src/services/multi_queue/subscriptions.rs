#![allow(non_snake_case)]

use std::{collections::HashMap, sync::mpsc::Sender};

use log::warn;

use crate::core_::point::{point_type::PointType, point_tx_id::PointTxId};

type ReceiverId = usize;
type PointName = String;
type PointId = usize; 

///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
pub struct Subscriptions {
    id: String,
    multicast: HashMap<PointId, HashMap<ReceiverId, Sender<PointType>>>,
    broadcast: HashMap<ReceiverId, Sender<PointType>>,
    empty: HashMap<ReceiverId, Sender<PointType>>,
    dictionary: HashMap<PointId, PointName>,
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
            dictionary: HashMap::new(),
        }
    }
    ///
    /// Adds subscription to Point ID with receiver ID
    pub fn addMulticast(&mut self, receiverId: usize, pointName: &str, sender: Sender<PointType>) {
        let pointId = PointTxId::fromStr(pointName);
        if ! self.multicast.contains_key(&pointId) {
            self.multicast.insert(
                pointId,
                HashMap::new(),
            );
            self.dictionary.insert(
                pointId,
                pointName.to_string(),
            );
        };
        match self.multicast.get_mut(&pointId) {
            Some(senders) => {
                senders.insert(
                    receiverId,
                    sender,
                );
            },
            None => {
                warn!("{}.addMulticast | Subscription '{}' - not found", self.id, pointName);
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
    pub fn iter(&self, pointName: &str) -> impl Iterator<Item = (&usize, &Sender<PointType>)> {
        let pointId = PointTxId::fromStr(pointName);
        match self.multicast.get(&pointId) {
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
    pub fn remove(&mut self, receiverId: &usize, pointName: &str) -> Result<(), String> {
        let pointId = PointTxId::fromStr(pointName);
        match self.multicast.get_mut(&pointId) {
            Some(senders) => {
                match senders.remove(receiverId) {
                    Some(_) => Ok(()),
                    None => Err(format!("{}.run | Subscription '{}', receiver '{}' - not found", self.id, pointName, receiverId)),
                }
            },
            None => Err(format!("{}.run | Subscription '{}' - not found", self.id, pointName)),
        }
    }
    ///
    /// Returns Point name by it's ID
    pub fn pointName(&self, pointId: usize) -> Option<&String> {
        self.dictionary.get(&pointId)
    }
}

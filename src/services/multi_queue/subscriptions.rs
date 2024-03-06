use log::{warn, trace};
use std::{collections::HashMap, hash::BuildHasherDefault, sync::mpsc::Sender};
use hashers::fx_hash::FxHasher;
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
    multicast: HashMap<PointDest, HashMap<ReceiverId, Sender<PointType>, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>>,
    broadcast: HashMap<ReceiverId, Sender<PointType>, BuildHasherDefault<FxHasher>>,
    empty: HashMap<ReceiverId, Sender<PointType>, BuildHasherDefault<FxHasher>>,
}
///
/// 
impl Subscriptions {
    ///
    /// Creates new instance of Subscriptions
    pub fn new(parent: impl Into<String>, ) -> Self {
        Self {
            id: format!("{}/Subscriptions", parent.into()),
            multicast: HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            broadcast: HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            empty: HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
        }
    }
    ///
    /// Adds subscription to Point ID with receiver ID
    pub fn add_multicast(&mut self, receiver_id: usize, destination: &str, sender: Sender<PointType>) {
        if ! self.multicast.contains_key(destination) {
            self.multicast.insert(
                destination.to_string(),
                HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            );
        };
        match self.multicast.get_mut(destination) {
            Some(senders) => {
                senders.insert(
                    receiver_id,
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
    pub fn add_broadcast(&mut self, receiver_id: usize, sender: Sender<PointType>) {
        self.broadcast.insert(
            receiver_id,
            sender,
        );
    }
    ///
    /// Returns map of Senders
    pub fn iter(&self, point_id: &str) -> impl Iterator<Item = (&usize, &Sender<PointType>)> {
        match self.multicast.get(point_id) {
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
    pub fn remove(&mut self, receiver_id: &usize, point_id: &str) -> Result<(), String> {
        match self.multicast.get_mut(point_id) {
            Some(senders) => {
                match senders.remove(receiver_id) {
                    Some(_) => Ok(()),
                    None => Err(format!("{}.run | Subscription '{}', receiver '{}' - not found", self.id, point_id, receiver_id)),
                }
            },
            None => Err(format!("{}.run | Subscription '{}' - not found", self.id, point_id)),
        }
    }
    ///
    /// Removes all subscriptions for receiver ID
    pub fn remove_all(&mut self, receiver_id: &usize) -> Result<(), String> {
        let mut changed = false;
        let mut messages = vec![];
        let keys: Vec<String> = self.multicast.keys().map(|v| v.clone()).collect();
        for pointId in keys {
            match self.multicast.get_mut(&pointId) {
                Some(senders) => {
                    match senders.remove(receiver_id) {
                        Some(_) => {
                            changed = changed | true;
                        },
                        None => {
                            messages.push(format!("{}.run | Multicast Subscription '{}', receiver '{}' - not found", self.id, pointId, receiver_id));
                        },
                    }
                },
                None => {
                    messages.push(format!("{}.run | Multicast Subscription '{}' - not found", self.id, pointId));
                }
            }
        }
        match self.broadcast.remove(receiver_id) {
            Some(_) => {
                changed = changed | true;
            },
            None => {
                messages.push(format!("{}.run | Broadcast Subscription by receiver '{}' - not found", self.id, receiver_id));
            },
        }
        if changed {
            Ok(())
        } else {
            Err(messages.join("\n"))
        }
    }
}

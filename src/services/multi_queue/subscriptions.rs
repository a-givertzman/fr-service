#![allow(non_snake_case)]

use std::{collections::HashMap, sync::mpsc::Sender};

use crate::core_::point::point_type::PointType;


///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
pub struct Subscriptions {
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

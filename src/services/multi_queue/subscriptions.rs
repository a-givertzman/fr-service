#![allow(non_snake_case)]

use std::{collections::HashMap, sync::mpsc::Sender};

use crate::core_::point::point_type::PointType;


///
/// Contains map of Sender's
/// - Where Sender - is pair of String ID & Sender<PointType>
pub struct Subscriptions {
    id: String,
    byPoints: HashMap<String, HashMap<String, Sender<PointType>>>,
}
///
/// 
impl Subscriptions {
    ///
    /// Creates new instance of Subscriptions
    pub fn new(parent: impl Into<String>, ) -> Self {
        Self {
            id: format!("{}/Subscriptions", parent.into()),
            byPoints: HashMap::new(),
        }
    }
    ///
    /// Adds subscription to Point ID with receiver ID
    pub fn add(&mut self, receiverId: &str, pointId: &str, sender: Sender<PointType>) {
        if ! self.byPoints.contains_key(pointId) {
            self.byPoints.insert(
                pointId.to_string(),
                HashMap::new(),
            );
        };
        match self.byPoints.get_mut(pointId) {
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
    /// Returns map of Senders
    pub fn get(&self, pointId: &str) -> Option<&HashMap<String, Sender<PointType>>> {
        self.byPoints.get(pointId)
    }
    ///
    /// Removes single subscription by Point Id & receiver ID
    pub fn remove(&mut self, receiverId: &str, pointId: &str) -> Result<(), String> {
        match self.byPoints.get_mut(pointId) {
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

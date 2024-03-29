use std::sync::mpsc::{Sender, Receiver};
use crate::{
    conf::point_config::point_config::PointConfig, 
    core_::{object::object::Object, point::point_type::PointType}, 
    services::multi_queue::subscription_criteria::SubscriptionCriteria,
};
use super::service_handles::ServiceHandles;

///
/// Interface for application service
/// - Running in the individual thread
pub trait Service: Object + std::fmt::Debug {
    // ///
    // /// Returns service's ID
    // fn id(&self) -> &str;
    ///
    /// Returns copy of the Sender - service's incoming queue
    #[allow(unused_variables)]
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        panic!("{}.get_link | Does not supported", self.id())
    }
    ///
    /// Returns Receiver
    #[allow(unused_variables)]
    fn subscribe(&mut self, receiver_id: &str, points: &[SubscriptionCriteria]) -> (Sender<PointType>, Receiver<PointType>) {
        panic!("{}.subscribe | Does not supported", self.id())
    }
    ///
    /// Extends the sucessfully with additiuonal points
    #[allow(unused_variables)]
    fn extend_subscription(&mut self, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        panic!("{}.extend_subscription | Does not supported", self.id())
    }
    ///
    /// Canceling the subsciption
    #[allow(unused_variables)]
    fn unsubscribe(&mut self, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        panic!("{}.unsubscribe | Does not supported", self.id())
    }
    ///
    /// Starts service's main loop in the individual thread
    fn run(&mut self) -> Result<ServiceHandles, String>;
    ///
    /// Returns list of configurations of the defined points
    fn points(&self) -> Vec<PointConfig> {
        vec![]
    }
    ///
    /// Sends "exit" signal to the service's thread
    fn exit(&self);
}
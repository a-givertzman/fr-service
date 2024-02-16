use std::{sync::mpsc::{Sender, Receiver}, thread::JoinHandle};

use crate::{core_::point::point_type::PointType, conf::point_config::point_config::PointConfig};

///
/// Interface for application service
/// - Running in the individual thread
pub trait Service {
    ///
    /// Returns service's ID
    fn id(&self) -> &str;
    ///
    /// Returns copy of the Sender - service's incoming queue
    fn get_link(&mut self, name: &str) -> Sender<PointType>;
    // panic!("{}.getLink | Does not support getLink", self.id())
    ///
    /// Returns Receiver
    #[allow(unused_variables)]
    fn subscribe(&mut self, receiver_id: &str, points: &Vec<String>) -> Receiver<PointType> {
        panic!("{}.subscribe | Does not support subscriptions", self.id())
    }
    ///
    /// Canceling the subsciption
    #[allow(unused_variables)]
    fn unsubscribe(&mut self, receiver_id: &str, points: &Vec<String>) -> Result<(), String> {
        panic!("{}.unsubscribe | Does not support subscriptions", self.id())
    }
    ///
    /// Starts service's main loop in the individual thread
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error>;
    ///
    /// Returns list of configurations of the defined points
    fn points(&self) -> Vec<PointConfig> {
        vec![]
    }
    ///
    /// Sends "exit" signal to the service's thread
    fn exit(&self);
}
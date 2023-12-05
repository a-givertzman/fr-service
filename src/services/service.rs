#![allow(non_snake_case)]

use std::{sync::mpsc::{Sender, Receiver}, thread::JoinHandle};

use crate::core_::point::point_type::PointType;

///
/// Interface for application service
/// - Running in the individual thread
pub trait Service {
    ///
    /// Returns service's ID
    fn id(&self) -> &str;
    ///
    /// Returns copy of the Sender - service's incoming queue
    fn getLink(&self, name: &str) -> Sender<PointType>;
    ///
    /// Returns Receiver
    fn subscribe(&mut self, receiverId: &str, points: &Vec<String>) -> Receiver<PointType> {
        panic!("{}.subscribe | Does not support subscriptions", self.id())
    }
    ///
    /// Canceling the subsciption
    fn unsubscribe(&mut self, receiverId: &str, points: &Vec<String>) -> Result<(), String> {
        panic!("{}.unsubscribe | Does not support subscriptions", self.id())
    }
    ///
    /// Starts service's main loop in the individual thread
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error>;
    ///
    /// Sends "exit" signal to the service's thread
    fn exit(&self);
}
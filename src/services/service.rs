#![allow(non_snake_case)]

use std::sync::mpsc::Sender;

use crate::core_::point::point_type::PointType;

///
/// Interface for application service
/// - Running in the individual thread
pub trait Service {
    ///
    /// Returns copy of the Sender - service's incoming queue
    fn getLink(&self, name: &str) -> Sender<PointType>;
    ///
    /// Starts service's main loop in the individual thread
    fn run(&mut self);
    ///
    /// Sends "exit" signal to the service's thread
    fn exit(&self);
}
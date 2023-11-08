#![allow(non_snake_case)]

///
/// Holds a reference to the specific async queue / channel implementation
/// Sharing standard interface to receive (bloking / non bloking) data from inner queueu
pub trait QueueRecv: std::fmt::Debug {
    fn recv(&mut self);
}
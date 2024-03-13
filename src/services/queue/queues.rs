#![allow(non_snake_case)]

use std::{collections::HashMap, sync::mpsc::{Sender, Receiver}};

use log::trace;

use crate::core_::point::point_type::PointType;


///
/// A container for storing FnInput by name
#[derive(Debug)]
pub struct Queues {
    // vars: HashMap<String, FnInOutRef>,
    sendQueues: HashMap<String, Sender<PointType>>,
    recvQueues: HashMap<String, Receiver<PointType>>,
}
impl Queues {
    ///
    /// Creates new container for storing FnInput
    pub fn new() -> Self {
        Self {
            // vars: HashMap::new(),
            sendQueues: HashMap::new(),
            recvQueues: HashMap::new(),
        }
    }
    // ///
    // /// Adding new variable refeerence
    // pub fn addVar(&mut self, name: impl Into<String> + Clone, input: FnInOutRef) {
    //     assert!(!self.vars.contains_key(name.clone().into().as_str()), "Dublicated variable name: {:?}", name.clone().into());
    //     assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
    //     self.vars.insert(name.into(), input);
    // }
    ///
    /// Adding new send queue
    pub fn addSendQueue(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, send: Sender<PointType>) {
        if self.sendQueues.contains_key(&name.clone().into()) {
            trace!("Queues.addSendQueue | send queue {:?} - already added", &name);
        } else {
            trace!("Queues.addSendQueue | adding send queue {:?}: {:?}", &name, &send);
            self.sendQueues.insert(name.into(), send);
        }
    }
    ///
    /// Adding new send queue
    pub fn addRecvQueue(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, recv: Receiver<PointType>) {
        if self.sendQueues.contains_key(&name.clone().into()) {
            trace!("Queues.addRecvQueue | recv queue {:?} - already added", &name);
        } else {
            trace!("Queues.addRecvQueue | adding recv queue {:?}: {:?}", &name, &recv);
            self.recvQueues.insert(name.into(), recv);
        }
    }
    // ///
    // /// Returns variable by it's name
    // pub fn getVar(&self, name: &str) -> Option<&FnInOutRef> {
    //     self.vars.get(name.into())
    // }
    ///
    /// Returns send queue by it's name
    pub fn getSendQueue(&mut self, name: &str) -> Sender<PointType> {
        match self.sendQueues.get_mut(name.into()) {
            Some(sendQueue) => sendQueue.clone(),
            None => {
                panic!("Queues.getSendQueue | sendQueue {:?} - not found", &name);
            },
        }
    }
    ///
    /// Returns send queue by it's name
    pub fn getRecvQueue(&mut self, name: &str) -> Receiver<PointType> {
        match self.recvQueues.remove(name.into()) {
            Some(recvQueue) => recvQueue,
            None => {
                panic!("Queues.getSendQueue | sendQueue {:?} - not found", &name);
            },
        }
    }
}

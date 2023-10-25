use std::{collections::HashMap, sync::mpsc::{Sender, Receiver}};

use log::trace;


///
/// A container for storing FnInput by name
#[derive(Debug)]
pub struct Queues {
    // vars: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
    sendQueues: HashMap<String, Sender<String>>,
    recvQueues: HashMap<String, Receiver<String>>,
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
    // pub fn addVar(&mut self, name: impl Into<String> + Clone, input: Rc<RefCell<Box<dyn FnInOut>>>) {
    //     assert!(!self.vars.contains_key(name.clone().into().as_str()), "Dublicated variable name: {:?}", name.clone().into());
    //     assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
    //     self.vars.insert(name.into(), input);
    // }
    ///
    /// Adding new send queue
    pub fn addSendQueue(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, send: Sender<String>) {
        if self.sendQueues.contains_key(&name.clone().into()) {
            trace!("Queues.addInput | send queue {:?} - already added", &name);
        } else {
            trace!("Queues.addInput | adding send queue {:?}: {:?}", &name, &send);
            self.sendQueues.insert(name.into(), send);
        }
    }
    // ///
    // /// Returns variable by it's name
    // pub fn getVar(&self, name: &str) -> Option<&Rc<RefCell<Box<dyn FnInOut>>>> {
    //     self.vars.get(name.into())
    // }
    ///
    /// Returns send queue by it's name
    pub fn getSendQueue(&mut self, name: &str) -> Sender<String> {
        match self.sendQueues.remove(name.into()) {
            Some(sendQueue) => sendQueue,
            None => {
                panic!("Queues.getSendQueue | sendQueue {:?} - not found", &name);
            },
        }
    }
}

use std::{collections::HashMap, rc::Rc, cell::RefCell, sync::mpsc::{Sender, Receiver}};

use log::trace;

use super::nested_function::fn_::FnInOut;



///
/// A container for storing FnInput by name
#[derive(Debug)]
pub struct TaskStuff {
    inputs: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
    vars: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
    sendQueues: HashMap<String, Sender<String>>,
    recvQueues: HashMap<String, Receiver<String>>,
}
impl TaskStuff {
    ///
    /// Creates new container for storing FnInput
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            vars: HashMap::new(),
            sendQueues: HashMap::new(),
            recvQueues: HashMap::new(),
        }
    }
    ///
    /// Adding new input refeerence
    pub fn addInput(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, input: Rc<RefCell<Box<dyn FnInOut>>>) {
        if self.inputs.contains_key(&name.clone().into()) {
            trace!("TaskStuff.addInput | input {:?} - already added", &name);
        } else {
            trace!("TaskStuff.addInput | adding input {:?}: {:?}", &name, input);
            self.inputs.insert(name.into(), input);
        }
    }
    ///
    /// Adding new variable refeerence
    pub fn addVar(&mut self, name: impl Into<String> + Clone, input: Rc<RefCell<Box<dyn FnInOut>>>) {
        assert!(!self.vars.contains_key(name.clone().into().as_str()), "Dublicated variable name: {:?}", name.clone().into());
        assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
        self.vars.insert(name.into(), input);
    }
    ///
    /// Adding new send queue
    pub fn addSendQueue(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, send: Sender<String>) {
        if self.sendQueues.contains_key(&name.clone().into()) {
            trace!("TaskStuff.addInput | send queue {:?} - already added", &name);
        } else {
            trace!("TaskStuff.addInput | adding send queue {:?}: {:?}", &name, &send);
            self.sendQueues.insert(name.into(), send);
        }
    }
    // ///
    // /// Adding new Bool input refeerence
    // pub fn addBool(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn FnOut<Bool>>>>) {
    //     self.refs.insert(name.into(), FnInType::Bool(input));
    // }
    // ///
    // /// Adding new Int input refeerence
    // pub fn addInt(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn FnOut<i64>>>>) {
    //     self.refs.insert(name.into(), FnInType::Int(input));
    // }
    // ///
    // /// Adding new Float input refeerence
    // pub fn addFloat(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn FnOut<f64>>>>) {
    //     self.refs.insert(name.into(), FnInType::Float(input));
    // }
    ///
    /// Returns input by it's name
    pub fn getInput(&self, name: &str) -> Option<&Rc<RefCell<Box<dyn FnInOut>>>> {
        self.inputs.get(name.into())
    }
    ///
    /// Returns variable by it's name
    pub fn getVar(&self, name: &str) -> Option<&Rc<RefCell<Box<dyn FnInOut>>>> {
        self.vars.get(name.into())
    }
    ///
    /// Returns send queue by it's name
    pub fn getSendQueue(&mut self, name: &str) -> Sender<String> {
        match self.sendQueues.remove(name.into()) {
            Some(sendQueue) => sendQueue,
            None => {
                panic!("TaskStuff.getSendQueue | sendQueue {:?} - not found", &name);
            },
        }
    }


    // ///
    // /// Returns input::Bool by it's name
    // pub fn getBool(&self, name: &str) -> Rc<RefCell<Box<dyn FnOut<Bool>>>> {
    //     match self.refs.get(name.into()) {
    //         Some(input) => {
    //             match input {
    //                 FnInType::Bool(input) => input.clone(),
    //                 _ => panic!("invalid type Bool of requested input {:?}", name),
    //             }
    //         },
    //         None => panic!("Unknown input name {:?}", name),
    //     }
    // }
    // ///
    // /// Returns input::Int by it's name
    // pub fn getInt(&self, name: &str) -> Rc<RefCell<Box<dyn FnOut<i64>>>> {
    //     match self.refs.get(name.into()) {
    //         Some(input) => {
    //             match input {
    //                 FnInType::Int(input) => input.clone(),
    //                 _ => panic!("invalid type Int of requested input {:?}", name),
    //             }
                
    //         },
    //         None => panic!("Unknown input name {:?}", name),
    //     }
    // }
    // ///
    // /// Returns input::Float by it's name
    // pub fn getFloat(&self, name: &str) -> Rc<RefCell<Box<dyn FnOut<f64>>>> {
    //     match self.refs.get(name.into()) {
    //         Some(input) => {
    //             match input {
    //                 FnInType::Float(input) => input.clone(),
    //                 _ => panic!("invalid type Float of requested input {:?}", name),
    //             }                
    //         },
    //         None => panic!("Unknown input name {:?}", name),
    //     }
    // }
}

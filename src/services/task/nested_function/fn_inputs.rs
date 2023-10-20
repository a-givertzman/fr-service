use std::{collections::HashMap, rc::Rc, cell::RefCell};

use super::fn_::FnInOut;



///
/// A container for storing FnInput by name
#[derive(Debug)]
pub struct FnInputs {
    inputs: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
    vars: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
}
impl FnInputs {
    ///
    /// Creates new container for storing FnInput
    pub fn new() -> Self {
        Self {
            inputs: HashMap::new(),
            vars: HashMap::new(),
        }
    }
    ///
    /// Adding new input refeerence
    pub fn addInput(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn FnInOut>>>) {
        self.inputs.insert(name.into(), input);
    }
    ///
    /// Adding new variable refeerence
    pub fn addVar(&mut self, name: impl Into<String>, input: Rc<RefCell<Box<dyn FnInOut>>>) {
        self.vars.insert(name.into(), input);
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

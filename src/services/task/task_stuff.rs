use std::{collections::HashMap, rc::Rc, cell::RefCell, sync::mpsc::{Sender, Receiver}};

use log::trace;

use super::nested_function::fn_::FnInOut;



///
/// A container for storing FnInput by name
#[derive(Debug)]
pub struct TaskStuffInputs {
    inputs: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
    vars: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
}
impl TaskStuffInputs {
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
}

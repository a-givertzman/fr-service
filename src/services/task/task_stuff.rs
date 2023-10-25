use std::{collections::HashMap, rc::Rc, cell::RefCell};

use log::trace;

use super::nested_function::fn_::FnInOut;



///
/// A container for storing FnInput & valiavles by name
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

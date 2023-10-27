#![allow(non_snake_case)]

use std::collections::{HashMap, HashSet};

use log::trace;

use crate::core_::types::fn_in_out_ref::FnInOutRef;


///
/// A container for storing storing variable & input names 
/// during configuring single TaskEvalNode only
#[derive(Debug)]
pub struct TaskNodeStuff {
    inputs: HashSet<String>,
    vars: HashSet<String>,
}
impl TaskNodeStuff {
    ///
    /// Creates new container for storing variable & input names
    /// during configuring single TaskEvalNode only
    pub fn new() -> Self {
        Self {
            inputs: HashSet::new(),
            vars: HashSet::new(),
        }
    }
    ///
    /// Adding new input name
    pub fn addInput(&mut self, name: impl Into<String> + std::fmt::Debug + Clone) {
        if self.inputs.contains(&name.clone().into()) {
            trace!("TaskNodeStuff.addInput | input {:?} - already added", &name);
        } else {
            trace!("TaskNodeStuff.addInput | adding input {:?}", &name);
            self.inputs.insert(name.into());
        }
    }
    ///
    /// Adding new variable name
    pub fn addVar(&mut self, name: impl Into<String> + Clone) {
        assert!(!self.vars.contains(name.clone().into().as_str()), "Dublicated variable name: {:?}", name.clone().into());
        assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
        trace!("TaskNodeStuff.addVar | adding variable {:?}", name.clone().into());
        self.vars.insert(name.into());
    }
    ///
    /// 
    fn names(collection: &HashMap<String, FnInOutRef>) -> Vec<String> {
        collection.keys().map(|v| v.clone()).collect()
    }
    ///
    /// Returns all collected input names
    pub fn getInputs(&mut self) -> HashSet<String> {
        self.inputs.clone()
    }
    ///
    /// Returns all collected var names
    pub fn getVars(&mut self) -> HashSet<String> {
        self.vars.clone()
    }
}
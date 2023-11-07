#![allow(non_snake_case)]

use std::collections::HashSet;

use crate::core_::types::fn_in_out_ref::FnInOutRef;

///
/// Holds Variable reference & all input names it depend on
#[derive(Debug)]
pub struct TaskNodeVar {
    var: FnInOutRef,
    inputs: HashSet<String>,
}
///
/// 
impl TaskNodeVar {
    ///
    /// 
    pub fn new(var: FnInOutRef, inputs: HashSet<String>) -> Self {
        Self {
            var,
            inputs: HashSet::from_iter(inputs),
        }
    }
    ///
    /// Returns variable reference
    pub fn var(&self) -> FnInOutRef {
        self.var.clone()
    }
    ///
    /// Returns all input names depend on
    pub fn inputs(&self) -> HashSet<String> {
        self.inputs.clone()
    }
}
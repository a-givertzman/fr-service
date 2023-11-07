#![allow(non_snake_case)]

use crate::core_::types::fn_in_out_ref::FnInOutRef;

///
/// Holds Variable reference & all input names it depend on
#[derive(Debug)]
pub struct TaskNodeVar {
    var: FnInOutRef,
    inputs: Vec<String>,
}
///
/// 
impl TaskNodeVar {
    ///
    /// 
    pub fn new(var: FnInOutRef, inputs: Vec<String>) -> Self {
        Self {
            var,
            inputs: Vec::from_iter(inputs),
        }
    }
    ///
    /// Returns variable reference
    pub fn var(&self) -> FnInOutRef {
        self.var.clone()
    }
    ///
    /// Returns all input names depend on
    pub fn inputs(&self) -> Vec<String> {
        self.inputs.clone()
    }
}
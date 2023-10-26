#![allow(non_snake_case)]

use crate::core_::types::fn_in_out_ref::FnInOutRef;

///
/// Holds Task input and all dipendent variables & outputs
#[derive(Debug)]
struct TaskInputDependent {
    name: String,
    input: FnInOutRef,
    outs: Vec<FnInOutRef>,
}
///
/// 
impl TaskInputDependent {
    ///
    /// Creates new instance from input name, input it self and dependent vars & outs
    pub fn new(name: impl Into<String>, input: FnInOutRef, outs: Vec<FnInOutRef>) -> Self {
        TaskInputDependent { 
            name: name.into(), 
            input: input, 
            outs:  outs,
        }
    }
    ///
    /// 
    pub fn getInput(&self) -> FnInOutRef {
        self.input.clone()
    }
    ///
    /// 
    pub fn getOuts(&self) -> & Vec<FnInOutRef> {
        &self.outs
    }
}
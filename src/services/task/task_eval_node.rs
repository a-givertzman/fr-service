#![allow(non_snake_case)]

use crate::core_::types::fn_in_out_ref::FnInOutRef;

use super::task_node_type::TaskNodeType;

///
/// Holds Task input and all dipendent variables & outputs
#[derive(Debug)]
pub struct TaskEvalNode {
    name: String,
    input: FnInOutRef,
    outs: Vec<TaskNodeType>,
}
///
/// 
impl TaskEvalNode {
    ///
    /// Creates new instance from input name, input it self and dependent vars & outs
    pub fn new(name: impl Into<String>, input: FnInOutRef, outs: Vec<TaskNodeType>) -> Self {
        TaskEvalNode { 
            name: name.into(), 
            input: input, 
            outs:  outs,
        }
    }
    ///
    /// 
    pub fn name(&self) ->String {
        self.name.clone()
    }
    ///
    /// 
    pub fn getInput(&self) -> FnInOutRef {
        self.input.clone()
    }
    ///
    /// 
    pub fn getOuts(&self) -> & Vec<TaskNodeType> {
        &self.outs
    }
}
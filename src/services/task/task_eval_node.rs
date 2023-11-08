#![allow(non_snake_case)]

use crate::core_::types::fn_in_out_ref::FnInOutRef;

use super::task_node_type::TaskNodeType;

///
/// Holds Task input and all dipendent variables & outputs
#[derive(Debug)]
pub struct TaskEvalNode {
    name: String,
    input: FnInOutRef,
    vars: Vec<TaskNodeType>,
    outs: Vec<TaskNodeType>,
}
///
/// 
impl TaskEvalNode {
    ///
    /// Creates new instance from input name, input it self and dependent vars & outs
    pub fn new(name: impl Into<String>, input: FnInOutRef) -> Self {
        TaskEvalNode { 
            name: name.into(), 
            input: input, 
            vars:  vec![],
            outs: vec![],
        }
    }
    ///
    /// 
    pub fn addVars(&mut self, vars: &mut Vec<TaskNodeType>) {
        self.vars.append(vars);
    }
    ///
    /// 
    pub fn addOut(&mut self, out: TaskNodeType) {
        self.outs.push(out);
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
        &self.vars
    }
}
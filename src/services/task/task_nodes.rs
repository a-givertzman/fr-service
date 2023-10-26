#![allow(non_snake_case)]

use std::collections::HashMap;

use log::{debug, trace};

use crate::core_::types::fn_in_out_ref::FnInOutRef;

use super::{task_node_inputs::TaskNodeInputs, task_eval_node::TaskEvalNode, task_node_type::TaskNodeType};


/// TaskNodes - holds the HashMap<TaskNode> in the following structure:
///   ```
///   {
///       inputName1: TaskNode {
///           input: FnInOutRef,
///           outs: [
///               var1
///               var2
///               var...
///               metric1
///               metric2
///               metric...
///           ]
///       },
///       inputName1: TaskNode {
///           ...
///       },
///   }
///   ```
#[derive(Debug)]
pub struct TaskNodes {
    inputs: HashMap<String, TaskEvalNode>,
}
///
/// 
impl TaskNodes {
    ///
    /// Creates new empty instance 
    pub fn new() ->Self {
        Self {
            inputs: HashMap::new(),
        }
    }
    ///
    /// 
    pub fn insert(&mut self, node: &mut TaskNodeInputs, out: TaskNodeType) {
        let vars = node.getVars();
        let inputs = node.getInputs();
        let mut outs: Vec<TaskNodeType> = vars.into_values().map(|var| TaskNodeType::Var(var)).collect();
        outs.push(out);
        for (name, input) in inputs {
            self.inputs.insert(
                name.clone(),
                TaskEvalNode::new(
                    name,
                    input,
                    outs.iter().map(|out|out.clone()).collect()
                )
            );
        };
        trace!("\nTaskStuff.add | self.inputs: {:?}\n", self.inputs);
    }
    ///
    /// Returns input by it's name
    pub fn getInput(&self, name: &str) -> Option<&TaskEvalNode> {
        self.inputs.get(name.into())
    }
}
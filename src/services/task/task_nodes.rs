#![allow(non_snake_case)]

use std::collections::HashMap;

use indexmap::IndexMap;
use log::{debug, trace};

use crate::core_::types::fn_in_out_ref::FnInOutRef;

use super::{task_node_stuff::TaskNodeStuff, task_eval_node::TaskEvalNode, task_node_type::TaskNodeType};


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
    vars: IndexMap<String, FnInOutRef>,
    nodeStuff: Option<TaskNodeStuff>,
}
///
/// 
impl TaskNodes {
    ///
    /// Creates new empty instance 
    pub fn new() ->Self {
        Self {
            inputs: HashMap::new(),
            vars: IndexMap::new(),
            nodeStuff: None,
        }
    }
    // ///
    // /// 
    // pub fn insert(&mut self, node: &mut TaskNodeStuff, out: TaskNodeType) {
    //     let vars = node.getVars();
    //     let inputs = node.getInputs();
    //     let mut outs: Vec<TaskNodeType> = vars.into_values().map(|var| TaskNodeType::Var(var)).collect();
    //     outs.push(out);
    //     for (name, input) in inputs {
    //         self.inputs.insert(
    //             name.clone(),
    //             TaskEvalNode::new(
    //                 name,
    //                 input,
    //                 outs.iter().map(|out|out.clone()).collect()
    //             )
    //         );
    //     };
    //     trace!("\nTaskNodes.add | self.inputs: {:?}\n", self.inputs);
    // }
    ///
    /// Returns input by it's name
    pub fn getEvalNode(&self, name: &str) -> Option<&TaskEvalNode> {
        self.inputs.get(name.into())
    }
    ///
    /// Returns input by it's name
    pub fn getInput(&self, name: &str) -> Option<FnInOutRef> {
        match self.inputs.get(name.into()) {
            Some(node) => {
                Some(node.getInput())
            },
            None => None,
        }
    }
    ///
    /// Returns variable by it's name
    pub fn getVar(&self, name: &str) -> Option<&FnInOutRef> {
        trace!("TaskNodes.getVar | trying to find variable {:?} in {:?}", &name, self.vars);
        self.vars.get(name.into())
    }
    ///
    /// 





    ///
    /// Adding new input refeerence
    pub fn addInput(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, input: FnInOutRef) {
        match self.nodeStuff {
            Some(_) => {
                if self.inputs.contains_key(&name.clone().into()) {
                    trace!("TaskNodes.addInput | input {:?} - already added", &name);
                } else {
                    debug!("TaskNodes.addInput | adding input {:?}", &name);
                    trace!("TaskNodes.addInput | adding input {:?}: {:?}", &name, &input);
                    self.inputs.insert(
                        name.clone().into(), 
                        TaskEvalNode::new(name.clone(), input),
                    );
                    self.nodeStuff.as_mut().unwrap().addInput(name);
                }
            },
            None => {
                panic!("TaskNodes.addInput | Call beginNewNode first, then you can add inputs")
            },
        }
    }
    ///
    /// Adding new variable refeerence
    pub fn addVar(&mut self, name: impl Into<String> + Clone, var: FnInOutRef) {
        assert!(!self.vars.contains_key(name.clone().into().as_str()), "Dublicated variable name: {:?}", name.clone().into());
        assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
        match self.nodeStuff {
            Some(_) => {
                if self.vars.contains_key(&name.clone().into()) {
                    self.vars.insert(name.clone().into(), var);
                    self.nodeStuff.as_mut().unwrap().addVar(name.clone().into());
                } else {
                    debug!("TaskNodes.addVar | adding variable {:?}", &name.clone().into());
                    trace!("TaskNodes.addVar | adding variable {:?}: {:?}", name.into(), &var);
                }
            },
            None => panic!("TaskNodes.addInput | Error: call beginNewNode first, then you can add inputs"),
        }
    }
    
    ///
    /// Call this metod if new Task node begins, 
    /// - after that you can add inputs and variables
    /// - to finish call finishNewNode(out) and pass created out
    pub fn beginNewNode(&mut self) {
        self.nodeStuff = Some(TaskNodeStuff::new());
    }
    ///
    /// Call this method if out is ready
    pub fn finishNewNode(&mut self, out: TaskNodeType) {
        match self.nodeStuff {
            Some(_) => {
                let mut outs: Vec<TaskNodeType> = vec![];
                for varName in self.nodeStuff.as_mut().unwrap().getVars() {
                    match self.vars.get(&varName) {
                        Some(var) => {
                            outs.push(
                                TaskNodeType::Var(var.clone())
                            )
                        },
                        None => panic!("TaskNodes.finishNewNode | Variable {:?} - not found", varName),
                    };
                };
                outs.push(out);
                for inputName in self.nodeStuff.as_mut().unwrap().getInputs() {
                    match self.inputs.get_mut(&inputName) {
                        Some(evalNode) => {
                            evalNode.addOuts(&mut outs);
                        },
                        None => panic!("TaskNodes.finishNewNode | Input {:?} - not found", inputName),
                    };
                };
                self.nodeStuff = None;
                trace!("\nTaskNodes.add | self.inputs: {:?}\n", self.inputs);
            },
            None => panic!("TaskNodes.finishNewNode | Call beginNewNode first, then you can add inputs & vars, then finish node"),
        }
    }
}
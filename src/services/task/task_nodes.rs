#![allow(non_snake_case)]

use std::collections::HashSet;

use indexmap::IndexMap;
use log::{debug, trace};

use crate::core_::types::fn_in_out_ref::FnInOutRef;

use super::{task_node_stuff::TaskNodeStuff, task_eval_node::TaskEvalNode, task_node_type::TaskNodeType, task_node_var::TaskNodeVar};


/// TaskNodes - holds the IndexMap<String, TaskNode> in the following structure:
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
    inputs: IndexMap<String, TaskEvalNode>,
    vars: IndexMap<String, TaskNodeVar>,
    newNodeStuff: Option<TaskNodeStuff>,
}
///
/// 
impl TaskNodes {
    ///
    /// Creates new empty instance 
    pub fn new() ->Self {
        Self {
            inputs: IndexMap::new(),
            vars: IndexMap::new(),
            newNodeStuff: None,
        }
    }
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
    pub fn getVar(&self, name: &str) -> Option<&TaskNodeVar> {
        trace!("TaskNodes.getVar | trying to find variable {:?} in {:?}", &name, self.vars);
        self.vars.get(name.into())
    }
    ///
    /// Adding new input refeerence
    pub fn addInput(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, input: FnInOutRef) {
        match self.newNodeStuff {
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
                    self.newNodeStuff.as_mut().unwrap().addInput(name);
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
        // assert!(!self.vars.contains_key(name.clone().into().as_str()), "Dublicated variable name: {:?}", name.clone().into());
        assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
        match self.newNodeStuff {
            Some(_) => {
                if self.vars.contains_key(&name.clone().into()) {
                    panic!("TaskNodes.addVar | Dublicated variable name: {:?}", &name.clone().into());
                } else {
                    debug!("TaskNodes.addVar | adding variable {:?}", &name.clone().into());
                    trace!("TaskNodes.addVar | adding variable {:?}: {:?}", &name.clone().into(), &var);
                    self.vars.insert(
                        name.clone().into(),
                        TaskNodeVar::new(var, self.newNodeStuff.as_ref().unwrap().getInputs()),
                    );
                }
                self.newNodeStuff.as_mut().unwrap().addVar(name.clone().into());
            },
            None => panic!("TaskNodes.addInput | Error: call beginNewNode first, then you can add inputs"),
        }
    }
    ///
    /// Adding already declared variable as out to the newNodeStuff
    pub fn addVarOut(&mut self, name: impl Into<String> + Clone) {
        assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
        match self.newNodeStuff {
            Some(_) => {
                self.newNodeStuff.as_mut().unwrap().addVar(name.clone().into());
            },
            None => panic!("TaskNodes.addInput | Error: call beginNewNode first, then you can add inputs"),
        }
    }    
    ///
    /// Call this metod if new Task node begins, 
    /// - after that you can add inputs and variables
    /// - to finish call [finishNewNode(out: TaskNodeType)] and pass created task node
    pub fn beginNewNode(&mut self) {
        self.newNodeStuff = Some(TaskNodeStuff::new());
    }
    ///
    /// Call this method to finish configuration of jast created task node
    pub fn finishNewNode(&mut self, out: TaskNodeType) {
        match self.newNodeStuff {
            Some(_) => {
                let mut outs: Vec<TaskNodeType> = vec![];
                for varName in self.newNodeStuff.as_mut().unwrap().getVars() {
                    match self.vars.get(&varName) {
                        Some(nodeVar) => {
                            outs.push(
                                TaskNodeType::Var(nodeVar.var().clone())
                            );
                            // let inputsVarDependOn = nodeVar.inputs();
                            // for inputName in inputsVarDependOn {

                            // }
                        },
                        None => panic!("TaskNodes.finishNewNode | Variable {:?} - not found", varName),
                    };
                };
                outs.push(out);
                let mut inputsDependOn = HashSet::<String>::new();
                for varName in self.newNodeStuff.as_mut().unwrap().getVars() {
                    match self.vars.get(&varName) {
                        Some(nodeVar) => {
                            let inputsVarDependOn = nodeVar.inputs();
                            inputsDependOn.extend(inputsVarDependOn);
                        },
                        None => panic!("TaskNodes.finishNewNode | Variable {:?} - not found", varName),
                    };
                };
                inputsDependOn.extend( self.newNodeStuff.as_mut().unwrap().getInputs() );
                for inputName in inputsDependOn {
                    match self.inputs.get_mut(&inputName) {
                        Some(evalNode) => {
                            evalNode.addOuts(&mut outs);
                        },
                        None => panic!("TaskNodes.finishNewNode | Input {:?} - not found", inputName),
                    };
                };
                self.newNodeStuff = None;
                trace!("\nTaskNodes.add | self.inputs: {:?}\n", self.inputs);
            },
            None => panic!("TaskNodes.finishNewNode | Call beginNewNode first, then you can add inputs & vars, then finish node"),
        }
    }
}
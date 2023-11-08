#![allow(non_snake_case)]

use indexmap::IndexMap;
use log::{debug, trace};

use crate::{core_::{types::fn_in_out_ref::FnInOutRef, conf::{task_config::TaskConfig, fn_conf_kind::FnConfKind}}, services::{queues::queues::Queues, task::nested_function::{metric_builder::MetricBuilder, nested_fn::NestedFn}}};

use super::{task_node_vars::TaskNodeVars, task_eval_node::TaskEvalNode, task_node_type::TaskNodeType};


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
    vars: IndexMap<String, FnInOutRef>,
    newNodeVars: Option<TaskNodeVars>,
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
            newNodeVars: None,
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
    pub fn getVar(&self, name: &str) -> Option<&FnInOutRef> {
        trace!("TaskNodes.getVar | trying to find variable {:?} in {:?}", &name, self.vars);
        self.vars.get(name.into())
    }
    ///
    /// Adding new input refeerence
    pub fn addInput(&mut self, name: impl Into<String> + std::fmt::Debug + Clone, input: FnInOutRef) {
        match self.newNodeVars {
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
        match self.newNodeVars {
            Some(_) => {
                if self.vars.contains_key(&name.clone().into()) {
                    panic!("TaskNodes.addVar | Dublicated variable name: {:?}", &name.clone().into());
                } else {
                    debug!("TaskNodes.addVar | adding variable {:?}", &name.clone().into());
                    trace!("TaskNodes.addVar | adding variable {:?}: {:?}", &name.clone().into(), &var);
                    self.vars.insert(
                        name.clone().into(),
                        var,
                    );
                }
                self.newNodeVars.as_mut().unwrap().addVar(name.clone().into());
            },
            None => panic!("TaskNodes.addInput | Error: call beginNewNode first, then you can add inputs"),
        }
    }
    ///
    /// Adding already declared variable as out to the newNodeStuff
    pub fn addVarOut(&mut self, name: impl Into<String> + Clone) {
        assert!(!name.clone().into().is_empty(), "Variable name can't be emty");
        match self.newNodeVars {
            Some(_) => {
                self.newNodeVars.as_mut().unwrap().addVar(name.clone().into());
            },
            None => panic!("TaskNodes.addInput | Error: call beginNewNode first, then you can add inputs"),
        }
    }    
    ///
    /// Call this metod if new Task node begins, 
    /// - after that you can add inputs and variables
    /// - to finish call [finishNewNode(out: TaskNodeType)] and pass created task node
    fn beginNewNode(&mut self) {
        self.newNodeVars = Some(TaskNodeVars::new());
    }
    ///
    /// Call this method to finish configuration of jast created task node
    fn finishNewNode(&mut self, out: TaskNodeType) {
        match self.newNodeVars {
            Some(_) => {
                let mut vars: Vec<TaskNodeType> = vec![];
                for varName in self.newNodeVars.as_mut().unwrap().getVars() {
                    match self.vars.get(&varName) {
                        Some(var) => {
                            vars.push(
                                TaskNodeType::Var(var.clone())
                            );
                        },
                        None => panic!("TaskNodes.finishNewNode | Variable {:?} - not found", varName),
                    };
                };
                let inputs = match &out {
                    TaskNodeType::Var(var) => var.borrow().inputs(),
                    TaskNodeType::Metric(metric) => metric.borrow().inputs(),
                };
                debug!("TaskNodes.finishNewNode | out {:?} \n\tdipending on inputs:: {:?}\n", &out, inputs);
                for inputName in inputs {
                    match self.inputs.get_mut(&inputName) {
                        Some(evalNode) => {
                            debug!("TaskNodes.finishNewNode | updating input: {:?}", inputName);
                            let len = vars.len();
                            evalNode.addVars(&mut vars.clone());
                            evalNode.addOut(out.clone());
                            debug!("TaskNodes.finishNewNode | evalNode '{}' appended: {:?}", evalNode.name(), len);
                        },
                        None => panic!("TaskNodes.finishNewNode | Input {:?} - not found", inputName),
                    };
                };
                self.newNodeVars = None;
                trace!("\nTaskNodes.finishNewNode | self.inputs: {:?}\n", self.inputs);
            },
            None => panic!("TaskNodes.finishNewNode | Call beginNewNode first, then you can add inputs & vars, then finish node"),
        }
    }
    ///
    /// Creates all task nodes depending on it config
    pub fn buildNodes(&mut self, conf: TaskConfig, queues: &mut Queues) {
        for (_nodeName, mut nodeConf) in conf.nodes {
            let nodeName = nodeConf.name.clone();
            debug!("TaskNodes.nodes | node: {:?}", &nodeConf.name);
            self.beginNewNode();
            let out = match nodeConf.fnKind {
                FnConfKind::Metric => {
                    TaskNodeType::Metric(
                        MetricBuilder::new(&mut nodeConf, self, queues)
                    )
                },
                FnConfKind::Fn => {
                    TaskNodeType::Metric(
                        NestedFn::new(&mut nodeConf, self, queues)
                    )
                },
                FnConfKind::Var => {
                    TaskNodeType::Var(
                        NestedFn::new(&mut nodeConf, self, queues)
                    )
                },
                FnConfKind::Const => {
                    panic!("TaskNodes.buildNodes | Const is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Point => {
                    panic!("TaskNodes.buildNodes | Point is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Param => {
                    panic!("TaskNodes.buildNodes | custom parameter: {:?}: {:?}", nodeName, &nodeConf);
                },
            };
            self.finishNewNode(out);
        }
    }

}
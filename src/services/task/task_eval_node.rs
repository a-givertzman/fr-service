#![allow(non_snake_case)]

use log::trace;

use crate::core_::{types::fn_in_out_ref::FnInOutRef, point::point_type::PointType};

///
/// Holds Task input and all dipendent variables & outputs
#[derive(Debug)]
pub struct TaskEvalNode {
    parentName: String,
    name: String,
    input: FnInOutRef,
    vars: Vec<FnInOutRef>,
    outs: Vec<FnInOutRef>,
}
///
/// 
impl TaskEvalNode {
    ///
    /// Creates new instance from input name, input it self and dependent vars & outs
    pub fn new(parentName: impl Into<String>, name: impl Into<String>, input: FnInOutRef) -> Self {
        TaskEvalNode { 
            parentName: parentName.into(), 
            name: name.into(), 
            input, 
            vars:  vec![],
            outs: vec![],
        }
    }
    ///
    /// 
    fn containsVar(&self, var: &FnInOutRef) -> bool {
        let varId = var.borrow().id();
        for selfVar in &self.vars {
            if selfVar.borrow().id() == varId {
                return true;
            }
        }
        false
    }
    ///
    /// 
    fn containsOut(&self, out: &FnInOutRef) -> bool {
        let outId = out.borrow().id();
        for selfOut in &self.outs {
            if selfOut.borrow().id() == outId {
                return true;
            }
        }
        false
    }
    ///
    /// 
    pub fn addVars(&mut self, vars: &Vec<FnInOutRef>) {
        for var in vars {
            if !self.containsVar(var) {
                self.vars.push(var.clone());
            }
        }
    }
    ///
    /// 
    pub fn addOut(&mut self, out: FnInOutRef) {
        if !self.containsOut(&out) {
            self.outs.push(out);
        }
    }
    ///
    /// 
    pub fn name(&self) -> String {
        // format!("{}/{}", self.parentName, self.name)
        self.name.clone()
    }
    ///
    /// 
    pub fn getInput(&self) -> FnInOutRef {
        self.input.clone()
    }
    ///
    /// 
    pub fn getVars(&self) -> &Vec<FnInOutRef> {
        &self.vars
    }
    ///
    /// 
    pub fn getOuts(&self) -> &Vec<FnInOutRef> {
        &self.outs
    }
    ///
    /// Adds new point to the hilding input reference
    pub fn add(&mut self, point: PointType) {
        self.input.borrow_mut().add(point);
    }
    ///
    /// Evaluates node:
    ///  - eval all conaining vars
    ///  - eval all conaining outs
    pub fn eval(&mut self, point: PointType) {
        // self.input.borrow_mut().add(point);
        for evalNodeVar in &self.vars {
            trace!("TaskEvalNode.eval | evalNode '{}/{}' - var '{}' evaluating...", self.parentName, self.name, evalNodeVar.borrow_mut().id());
            evalNodeVar.borrow_mut().eval();
            trace!("TaskEvalNode.eval | evalNode '{}/{}' - var '{}' evaluated", self.parentName, self.name, evalNodeVar.borrow_mut().id());
        };
        for evalNodeOut in &self.outs {
            trace!("TaskEvalNode.eval | evalNode '{}/{}' out...", self.parentName, self.name);
            let out = evalNodeOut.borrow_mut().out();
            trace!("TaskEvalNode.eval | evalNode '{}/{}' out: {:?}", self.parentName, self.name, out);
        };
    }
}
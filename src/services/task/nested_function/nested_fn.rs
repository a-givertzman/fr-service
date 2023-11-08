#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell, str::FromStr, sync::mpsc::Sender};

use crate::{
    core_::{
        point::point_type::{PointType, ToPoint},
        conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType}, types::fn_in_out_ref::FnInOutRef, 
    }, 
    services::{task::{nested_function::{metric_builder::MetricBuilder, fn_var::FnVar}, task_nodes::TaskNodes}, queues::queues::Queues}
};

use super::{fn_::FnInOut, fn_input::FnInput, fn_add::FnAdd, fn_timer::FnTimer, functions::Functions, export::fn_to_api_queue::FnToApiQueue, fn_count::FnCount, fn_const::FnConst};

///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(conf: &mut FnConfig, taskNodes: &mut TaskNodes, queues: &mut Queues) -> FnInOutRef {
        Self::function("", conf, taskNodes, queues)
    }
    ///
    /// 
    fn function(inputName: &str, conf: &mut FnConfig, taskNodes: &mut TaskNodes, queues: &mut Queues) -> FnInOutRef {
        match conf.fnKind {
            FnConfKind::Fn => {
                println!("NestedFn.function | Fn {:?}: {:?}...", inputName, conf.name.clone());
                let c = conf.name.clone();
                let fnName= c.clone();
                let fnName = fnName.as_str(); 
                drop(c);
                match Functions::from_str(fnName).unwrap() {
                    Functions::Count => {
                        println!("NestedFn.function | Fn count detected");
                        let initial = 0;
                        let name = "input";
                        let inputConf = conf.inputConf(name);
                        let input = Self::function(name, inputConf, taskNodes, queues);
                        Self::fnCount(inputName, initial, input)
                    }
                    Functions::Add => {
                        println!("NestedFn.function | Fn add detected");
                        let name = "input1";
                        let inputConf = conf.inputConf(name);
                        let input1 = Self::function(name, inputConf, taskNodes, queues);
                        let name = "input2";
                        let inputConf = conf.inputConf(name);
                        let input2 = Self::function(name, inputConf, taskNodes, queues);
                        Self::fnAdd(inputName, input1, input2)
                    }
                    Functions::Timer => {
                        println!("NestedFn.function | Fn timer detected");
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input = Self::function(name, conf, taskNodes, queues);
                        Self::fnTimer(inputName, 0.0, input, true)
                    },
                    Functions::ToApiQueue => {
                        println!("NestedFn.function | Fn toApiQueue detected");
                        let name = "input";
                        let inputConf = conf.inputConf(name);
                        let input = Self::function(name, inputConf, taskNodes ,queues);
                        let queueName = conf.param("queue").name.clone();
                        let sendQueue = queues.getSendQueue(&queueName);
                        Self::toApiQueue(inputName, input, sendQueue)
                        // Self::toApiQueue(inputName, queue, input)
                    },
                    _ => panic!("NestedFn.function | Unknown function name: {:?}", conf.name)
                }
            },
            FnConfKind::Var => {
                let varName = conf.name.clone();
                println!("NestedFn.function | Var: {:?}...", varName);
                match conf.inputs.iter_mut().next() {
                    //
                    // New var declaration
                    Some((inputConfName, inputConf)) => {
                        let var = Self::fnVar(               
                            varName, 
                            Self::function(&inputConfName, inputConf, taskNodes, queues),
                        );
                        println!("NestedFn.function | Var: {:?}: {:?}", &conf.name, var.clone());
                        taskNodes.addVar(conf.name.clone(), var.clone());
                        // println!("NestedFn.function | Var: {:?}", input);
                        var
                    },
                    // Usage declared variable
                    None => {
                        let var = match taskNodes.getVar(&varName) {
                            Some(var) => var,
                            None => panic!("NestedFn.function | Var {:?} - not declared", &varName),
                        }.to_owned();
                        // let var = nodeVar.var();
                        taskNodes.addVarOut(conf.name.clone());
                        var
                    },
                }
            },
            FnConfKind::Const => {
                let value = conf.name.trim().to_lowercase();
                let name = format!("const {:?} '{}'", conf.type_, value);
                println!("NestedFn.function | Const: {:?}...", &name);
                let value = match conf.type_.clone() {
                    FnConfPointType::Bool => value.parse::<bool>().unwrap().toPoint(&name),
                    FnConfPointType::Int => value.parse::<i64>().unwrap().toPoint(&name),
                    FnConfPointType::Float => value.parse::<f64>().unwrap().toPoint(&name),
                    FnConfPointType::String => value.toPoint(&name),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                let fnConst = Self::fnConst(&name, value);
                // taskNodes.addInput(inputName, input.clone());
                println!("NestedFn.function | Const: {:?} - done", fnConst);
                fnConst
            },
            FnConfKind::Point => {                
                println!("NestedFn.function | Input (Point): {:?} ({:?})...", inputName, conf.name);
                let initial = match conf.type_.clone() {
                    FnConfPointType::Bool => false.toPoint("input initial"),
                    FnConfPointType::Int => 0.toPoint("input initial"),
                    FnConfPointType::Float => 0.0.toPoint("input initial"),
                    FnConfPointType::String => "".toPoint("input initial"),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                let pointName = conf.name.clone();
                taskNodes.addInput(&pointName, Self::fnInput(&pointName, initial));
                let input = taskNodes.getInput(&pointName).unwrap();
                println!("NestedFn.function | input (Point): {:?}", input);
                input
            },
            FnConfKind::Metric => {
                println!("NestedFn.function | Metric {:?}", &conf.name);
                MetricBuilder::new(conf, taskNodes, queues)
            },
            FnConfKind::Param => {
                panic!("NestedFn.function | Custom parameters are not supported in the nested functions");
            }
        }
    }
    ///
    /// 
    /// 
    /// 
    fn fnCount(id: impl Into<String>, initial: i64, input: FnInOutRef,) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(                
            FnCount::new(id, initial, input),
        )))
    }
    /// 
    /// 
    fn fnVar(id: impl Into<String>, input: FnInOutRef,) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(                
            FnVar::new(id, input),
        )))
    }
    /// 
    /// 
    fn toApiQueue(id: impl Into<String>, input: FnInOutRef, sendQueue: Sender<PointType>) -> Rc<RefCell<Box<(dyn FnInOut)>>> {
        Rc::new(RefCell::new(Box::new(
            FnToApiQueue::new(id, input, sendQueue)
        )))
    }
    // ///
    // /// 
    fn fnConst(id: &str, value: PointType) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnConst::new(id, value)
        )))
    }
    // ///
    // /// 
    fn fnInput(id: &str, initial: PointType) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnInput::new(id, initial)
        )))
    }
    // ///
    // /// 
    fn fnAdd(
        id: &str, 
        input1: FnInOutRef, 
        input2: FnInOutRef
    ) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(        
            FnAdd::new(id, input1, input2)
        )))
    }    
    // ///
    // /// 
    fn fnTimer(
        id: &str, 
        initial: impl Into<f64> + Clone,
        input: FnInOutRef, 
        repeat: bool,
    ) -> FnInOutRef {
        Rc::new(RefCell::new(
            Box::new(        
                FnTimer::new(
                    id,
                    initial, 
                    input, 
                    repeat
                )
            )
        ))
    }    
}

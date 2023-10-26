#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell, str::FromStr, sync::mpsc::Sender};

use crate::{
    core_::{
        point::point_type::{PointType, ToPoint},
        conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType}, types::fn_in_out_ref::FnInOutRef, 
    }, 
    services::{task::{nested_function::{metric_builder::MetricBuilder, fn_var::FnVar}, task_node_inputs::TaskNodeStuff}, queues::queues::Queues}
};

use super::{fn_::FnInOut, fn_input::FnInput, fn_add::FnAdd, fn_timer::FnTimer, functions::Functions, export::fn_to_api_queue::FnToApiQueue};

///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(conf: &mut FnConfig, taskNodeStuff: &mut TaskNodeStuff, queues: &mut Queues) -> FnInOutRef {
        Self::function("", conf, taskNodeStuff, queues)
    }
    ///
    /// 
    fn function(inputName: &str, conf: &mut FnConfig, taskNodeStuff: &mut TaskNodeStuff, queues: &mut Queues) -> FnInOutRef {
        match conf.fnKind {
            FnConfKind::Fn => {
                println!("NestedFn.function | Fn {:?}: {:?}...", inputName, conf.name.clone());
                let c = conf.name.clone();
                let fnName= c.clone();
                let fnName = fnName.as_str(); 
                drop(c);
                match Functions::from_str(fnName).unwrap() {
                    Functions::Add => {
                        println!("NestedFn.function | Fn sum detected");
                        let name = "input1";
                        let inputConf = conf.inputConf(name);   // Self::getFnInputConf(name, fnName, conf);
                        let input1 = Self::function(name, inputConf, taskNodeStuff, queues);
                        let name = "input2";
                        let inputConf = conf.inputConf(name);   // Self::getFnInputConf(name, fnName, conf);
                        let input2 = Self::function(name, inputConf, taskNodeStuff, queues);
                        Self::fnSum(inputName, input1, input2)
                    }
                    Functions::Timer => {
                        println!("NestedFn.function | Fn timer detected");
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input = Self::function(name, conf, taskNodeStuff, queues);
                        Self::fnTimer(inputName, 0.0, input, true)
                    },
                    Functions::ToApiQueue => {
                        println!("NestedFn.function | Fn toApiQueue detected");
                        let name = "input";
                        let inputConf = conf.inputConf(name);   // Self::getFnInputConf(name, fnName, conf);
                        let input = Self::function(name, inputConf, taskNodeStuff ,queues);
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
                let (inputConfName, inputConf) = match conf.inputs.iter_mut().next() {
                    Some(inputConf) => inputConf,
                    None => panic!("NestedFn.function | Var {:?} must have exact one input", &varName),
                };
                let input = Self::fnVar(               
                    inputConfName, 
                    Self::function(&inputConfName, inputConf, taskNodeStuff, queues),
                );
                println!("NestedFn.function | Var: {:?}: {:?}", &conf.name, input.clone());
                taskNodeStuff.addVar(conf.name.clone(), input.clone());
                // println!("NestedFn.function | Var: {:?}", input);
                input
            },
            FnConfKind::Const => {
                let value = conf.name.trim().to_lowercase();
                println!("NestedFn.function | Const: {:?}...", value);
                let initial = match conf.type_.clone() {
                    FnConfPointType::Bool => value.parse::<bool>().unwrap().toPoint("const"),
                    FnConfPointType::Int => value.parse::<i64>().unwrap().toPoint("const"),
                    FnConfPointType::Float => value.parse::<f64>().unwrap().toPoint("const"),
                    FnConfPointType::String => value.toPoint("const"),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                let input = Self::fnInput(inputName, initial);
                taskNodeStuff.addInput(inputName, input.clone());
                println!("NestedFn.function | Const: {:?} - done", input);
                input
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
                let input = Self::fnInput(inputName, initial);
                let pointName = conf.name.clone();
                taskNodeStuff.addInput(&pointName, input.clone());
                let input = taskNodeStuff.getInput(&pointName).unwrap().to_owned();
                println!("NestedFn.function | input (Point): {:?}", input);
                input
            },
            FnConfKind::Metric => {
                println!("NestedFn.function | Metric nested in the function is not implemented");
                MetricBuilder::new(conf, taskNodeStuff, queues)
            },
            FnConfKind::Param => {
                panic!("NestedFn.function | Custom parameters are not supported in the nested functions");
            }
        }
    }
    ///
    /// 
    /// 
    fn fnVar(id: impl Into<String>, input: FnInOutRef,) -> FnInOutRef {
        Rc::new(RefCell::new(
            Box::new(                
                FnVar::new(
                    id, 
                    input,
                ),
            )
        ))
    }
    /// 
    /// 
    fn toApiQueue(id: impl Into<String>, input: FnInOutRef, sendQueue: Sender<PointType>) -> Rc<RefCell<Box<(dyn FnInOut)>>> {
        Rc::new(RefCell::new(
            Box::new(
                FnToApiQueue::new(id, input, sendQueue)
            )
        ))
    }
    // ///
    // /// 
    // fn boxFnInput(input: FnInput) -> Box<(dyn FnInOut)> {
    //     Box::new(input)
    // }
    fn fnInput(id: &str, initial: PointType) -> FnInOutRef {
        Rc::new(RefCell::new(
            Box::new(
                FnInput::new( 
                    id,
                    initial, 
                )
            )
        ))
    }
    // ///
    // /// 
    // fn boxFnSum(input: FnAdd) -> Box<(dyn FnInOut)> {
    //     Box::new(input)
    // }
    fn fnSum(
        id: &str, 
        input1: FnInOutRef, 
        input2: FnInOutRef
    ) -> FnInOutRef {
        Rc::new(RefCell::new(
            Box::new(        
                FnAdd::new(
                    id,
                    input1, 
                    input2, 
                )
            )
        ))
    }    
    // ///
    // /// 
    // fn boxFnTimer(input: FnTimer) -> Box<(dyn FnInOut)> {
    //     Box::new(input)
    // }
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

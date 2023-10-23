#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell, str::FromStr, sync::mpsc::Sender};

use crate::{
    core_::{
        point::{point_type::PointType, point::Point},
        conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType}, 
    }, 
    services::task::{nested_function::metric_builder::MetricBuilder, task_stuff::TaskStuff}
};

use super::{fn_::FnInOut, fn_input::FnInput, fn_add::FnAdd, fn_timer::FnTimer, functions::Functions, export::fn_to_api_queue::FnToApiQueue};

///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(conf: &mut FnConfig, taskStuff: &mut TaskStuff) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Self::function("", conf, taskStuff)
    }
    // fn getFnInputConf<'a>(inputName: &str, fnName: &str, conf: &'a mut FnConfig) -> &'a mut FnConfig {
    //     match conf.inputs.get_mut(inputName) {
    //         Some(conf) => conf,
    //         None => panic!("NestedFn.function | function {:?} must have {:?}", fnName, inputName),
    //     }
    // }
    ///
    /// 
    fn function(inputName: &str, conf: &mut FnConfig, taskStuff: &mut TaskStuff) -> Rc<RefCell<Box<dyn FnInOut>>> {
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
                        let input1 = Self::function(name, inputConf, taskStuff);
                        let name = "input2";
                        let inputConf = conf.inputConf(name);   // Self::getFnInputConf(name, fnName, conf);
                        let input2 = Self::function(name, inputConf, taskStuff);
                        Self::fnSum(inputName, input1, input2)
                    }
                    Functions::Timer => {
                        println!("NestedFn.function | Fn timer detected");
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input = Self::function(name, conf, taskStuff);
                        Self::fnTimer(inputName, 0.0, input, true)
                    },
                    Functions::ToApiQueue => {
                        println!("NestedFn.function | Fn toApiQueue detected");
                        let name = "input";
                        let inputConf = conf.inputConf(name);   // Self::getFnInputConf(name, fnName, conf);
                        let input = Self::function(name, inputConf, taskStuff);
                        let sendQueue = taskStuff.getSendQueue("apiQueue");
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
                let input = Self::function(&inputConfName, inputConf, taskStuff);
                taskStuff.addVar(conf.name.clone(), input.clone());
                println!("NestedFn.function | Var: {:?}", input);
                input
            },
            FnConfKind::Const => {
                let value = conf.name.trim().to_lowercase();
                println!("NestedFn.function | Const: {:?}...", value);
                let initial = match conf.type_.clone() {
                    FnConfPointType::Bool => PointType::Bool(Point::newBool("const", value.parse().unwrap())),
                    FnConfPointType::Int => PointType::Int(Point::newInt("const", value.parse().unwrap())),
                    FnConfPointType::Float => PointType::Float(Point::newFloat("const", value.parse().unwrap())),
                    FnConfPointType::String => PointType::String(Point::newString("const", value)),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                let input = Self::fnInput(inputName, initial);
                taskStuff.addInput(inputName, input.clone());
                println!("NestedFn.function | Const: {:?} - done", input);
                input
            },
            FnConfKind::Point => {                
                println!("NestedFn.function | Input (Point): {:?} ({:?})...", inputName, conf.name);
                let initial = match conf.type_.clone() {
                    FnConfPointType::Bool => PointType::Bool(Point::newBool("input initial", false)),
                    FnConfPointType::Int => PointType::Int(Point::newInt("input initial", 0)),
                    FnConfPointType::Float => PointType::Float(Point::newFloat("input initial", 0.0)),
                    FnConfPointType::String => PointType::String(Point::newString("input initial", "")),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                let input = Self::fnInput(inputName, initial);
                let pointName = conf.name.clone();
                taskStuff.addInput(&pointName, input.clone());
                let input = taskStuff.getInput(&pointName).unwrap().to_owned();
                println!("NestedFn.function | input (Point): {:?}", input);
                input
            },
            FnConfKind::Metric => {
                println!("NestedFn.function | Metric nested in the function is not implemented");
                MetricBuilder::new(conf, taskStuff)
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
    fn toApiQueue(id: impl Into<String>, input: Rc<RefCell<Box<dyn FnInOut>>>, sendQueue: Sender<String>) -> Rc<RefCell<Box<(dyn FnInOut)>>> {
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
    fn fnInput(inputName: &str, initial: PointType) -> Rc<RefCell<Box<dyn FnInOut>>> {
        Rc::new(RefCell::new(
            Box::new(
                FnInput::new( 
                    inputName,
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
        input1: Rc<RefCell<Box<dyn FnInOut>>>, 
        input2: Rc<RefCell<Box<dyn FnInOut>>>
    ) -> Rc<RefCell<Box<dyn FnInOut>>> {
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
        input: Rc<RefCell<Box<dyn FnInOut>>>, 
        repeat: bool,
    ) -> Rc<RefCell<Box<dyn FnInOut>>> {
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
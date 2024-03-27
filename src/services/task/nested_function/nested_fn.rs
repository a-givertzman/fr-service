use std::{rc::Rc, cell::RefCell, str::FromStr, sync::{mpsc::Sender, Arc, Mutex}};
use log::{debug, LevelFilter};
use crate::{
    conf::{fn_::{fn_conf_keywd::FnConfPointType, fn_conf_kind::FnConfKind}, point_config::point_config::PointConfig}, core_::{
        point::point_type::{PointType, ToPoint},
        types::fn_in_out_ref::FnInOutRef, 
    }, services::{services::Services, task::{nested_function::{fn_var::FnVar, sql_metric::SqlMetric}, task_nodes::TaskNodes}}
};
use super::{
    export::fn_to_api_queue::FnToApiQueue, fn_add::FnAdd, fn_const::FnConst, fn_count::FnCount, fn_ge::FnGe, fn_input::FnInput, fn_point_id::FnPointId, fn_timer::FnTimer, functions::Functions 
};

///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(parent: &str, tx_id: usize, conf: &mut FnConfKind, task_nodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> FnInOutRef {
        Self::function(parent, tx_id, "", conf, task_nodes, services)
    }
    ///
    /// 
    fn function(parent: &str, tx_id: usize, input_name: &str, conf: &mut FnConfKind, task_nodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> FnInOutRef {
        match conf {
            FnConfKind::Fn(conf) => {
                println!("NestedFn.function | Fn {:?}: {:?}...", input_name, conf.name.clone());
                let c = conf.name.clone();
                let fn_name= c.clone();
                let fn_name = fn_name.as_str(); 
                drop(c);
                let fn_name = Functions::from_str(fn_name).unwrap();
                println!("NestedFn.function | Fn '{}' detected", fn_name.name());
                match fn_name {
                    Functions::Count => {
                        let initial = 0.0;
                        let name = "input";
                        let input_conf = conf.input_conf(name);
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services);
                        Self::fn_count(parent, initial, input)
                    }
                    Functions::Add => {
                        let name = "input1";
                        let input_conf = conf.input_conf(name);
                        let input1 = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let name = "input2";
                        let input_conf = conf.input_conf(name);
                        let input2 = Self::function(parent, tx_id, name, input_conf, task_nodes, services);
                        Self::fn_add(parent, input1, input2)
                    }
                    Functions::Timer => {
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input = Self::function(parent, tx_id, name, conf, task_nodes, services);
                        Self::fn_timer(parent, 0.0, input, true)
                    },
                    Functions::ToApiQueue => {
                        let name = "input";
                        let input_conf = conf.input_conf(name);
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes ,services.clone());
                        let queue_name = conf.param("queue").name();
                        let services_lock = services.lock();
                        let send_queue = services_lock.unwrap().get_link(&queue_name);
                        Self::to_api_queue(parent, input, send_queue)
                        // Self::toApiQueue(inputName, queue, input)
                    },
                    Functions::Ge => {
                        let name = "input1";
                        let input_conf = conf.input_conf(name);
                        let input1 = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let name = "input2";
                        let input_conf = conf.input_conf(name);
                        let input2 = Self::function(parent, tx_id, name, input_conf, task_nodes, services);
                        Self::fn_ge(parent, input1, input2)
                    },
                    Functions::SqlMetric => {
                        debug!("NestedFn.function | fnConf: {:?}: {:?}", conf.name, conf);
                        Rc::new(RefCell::new(                    
                            Box::new(
                                SqlMetric::new(
                                    parent,
                                    conf, 
                                    task_nodes,
                                    services,
                                )
                            )
                        ))        
                    }
                    Functions::PointId => {
                        debug!("NestedFn.function | fnConf: {:?}: {:?}", conf.name, conf);
                        let name = "input";
                        let input_conf = conf.input_conf(name);
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        Self::fn_point_id(parent, input, services.lock().unwrap().points())

                    }
                    _ => panic!("NestedFn.function | Unknown function name: {:?}", conf.name)
                }
            },
            FnConfKind::Var(conf) => {
                let var_name = conf.name.clone();
                println!("NestedFn.function | Var: {:?}...", var_name);
                match conf.inputs.iter_mut().next() {
                    //
                    // New var declaration
                    Some((input_conf_name, input_conf)) => {
                        let var = Self::fn_var(               
                            var_name, 
                            Self::function(parent, tx_id, input_conf_name, input_conf, task_nodes, services),
                        );
                        println!("NestedFn.function | Var: {:?}: {:?}", &conf.name, var.clone());
                        task_nodes.addVar(conf.name.clone(), var.clone());
                        // println!("NestedFn.function | Var: {:?}", input);
                        var
                    },
                    // Usage declared variable
                    None => {
                        let var = match task_nodes.getVar(&var_name) {
                            Some(var) => var,
                            None => panic!("NestedFn.function | Var {:?} - not declared", &var_name),
                        }.to_owned();
                        // let var = nodeVar.var();
                        task_nodes.addVarOut(conf.name.clone());
                        var
                    },
                }
            },
            FnConfKind::Const(conf) => {
                let value = conf.name.trim().to_lowercase();
                let name = format!("const {:?} '{}'", conf.type_, value);
                println!("NestedFn.function | Const: {:?}...", &name);
                let value = match conf.type_.clone() {
                    FnConfPointType::Bool => value.parse::<bool>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::Int => value.parse::<i64>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::Real => value.parse::<f32>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::Double => value.parse::<f64>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::String => value.to_point(tx_id, &name),
                    FnConfPointType::Any => panic!("NestedFn.function | Const of type 'any' - not supported"),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                let fn_const = Self::fn_const(&name, value);
                // taskNodes.addInput(inputName, input.clone());
                println!("NestedFn.function | Const: {:?} - done", fn_const);
                fn_const
            },
            FnConfKind::Point(conf) => {                
                println!("NestedFn.function | Input (Point<{:?}>): {:?} ({:?})...", conf.type_, input_name, conf.name);
                let initial = match conf.type_.clone() {
                    FnConfPointType::Bool => false.to_point(tx_id, &conf.name),
                    FnConfPointType::Int => 0.to_point(tx_id, &conf.name),
                    FnConfPointType::Real => 0.0f32.to_point(tx_id, &conf.name),
                    FnConfPointType::Double => 0.0f64.to_point(tx_id, &conf.name),
                    FnConfPointType::String => "".to_point(tx_id, &conf.name),
                    FnConfPointType::Any => false.to_point(tx_id, &conf.name),
                    FnConfPointType::Unknown => panic!("NestedFn.function | Point type required"),
                };
                println!("NestedFn.function | Input initial: {:?}", initial);
                let point_name = conf.name.clone();
                task_nodes.addInput(&point_name, Self::fn_input(&point_name, initial, conf.type_.clone()));
                let input = task_nodes.getInput(&point_name).unwrap();
                if log::max_level() == LevelFilter::Trace {
                    println!("NestedFn.function | input (Point): {:?}", input);
                }
                input
            },
            FnConfKind::PointConf(_conf) => {
                panic!("NestedFn.function | PointConf is not supported in the nested functions yet");
            }
            // FnConfKind::Metric(conf) => {
            //     println!("NestedFn.function | Metric {:?}", &conf.name);
            //     MetricBuilder::new(parent, conf, taskNodes, services)
            // },
            FnConfKind::Param(_conf) => {
                panic!("NestedFn.function | Custom parameters are not supported in the nested functions");
            }
        }
    }
    ///
    /// 
    /// 
    /// 
    fn fn_count(parent: impl Into<String>, initial: f64, input: FnInOutRef,) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(                
            FnCount::new(parent, initial, input),
        )))
    }
    /// 
    /// 
    fn fn_var(parent: impl Into<String>, input: FnInOutRef,) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(                
            FnVar::new(parent, input),
        )))
    }
    /// 
    /// 
    fn to_api_queue(parent: impl Into<String>, input: FnInOutRef, send_queue: Sender<PointType>) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnToApiQueue::new(parent, input, send_queue)
        )))
    }
    // ///
    // /// 
    fn fn_const(parent: &str, value: PointType) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnConst::new(parent, value)
        )))
    }
    // ///
    // /// 
    fn fn_input(parent: &str, initial: PointType, type_: FnConfPointType) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnInput::new(parent, initial, type_)
        )))
    }
    // ///
    // /// 
    fn fn_add(parent: &str, input1: FnInOutRef, input2: FnInOutRef) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(        
            FnAdd::new(parent, input1, input2)
        )))
    }    
    // ///
    // /// 
    fn fn_timer(parent: &str, initial: impl Into<f64> + Clone,input: FnInOutRef, repeat: bool) -> FnInOutRef {
        Rc::new(RefCell::new(
            Box::new(        
                FnTimer::new(
                    parent,
                    initial, 
                    input, 
                    repeat
                )
            )
        ))
    }    
    // ///
    // /// 
    fn fn_ge(parent: &str, input1: FnInOutRef, input2: FnInOutRef) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(        
            FnGe::new(parent, input1, input2)
        )))
    }
    // ///
    // /// 
    fn fn_point_id(parent: &str, input: FnInOutRef, points: Vec<PointConfig>) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnPointId::new(parent, input, points)
        )))
    }
}

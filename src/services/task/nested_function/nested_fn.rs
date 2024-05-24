use std::{rc::Rc, cell::RefCell, str::FromStr, sync::{Arc, Mutex}};
use log::{debug, warn, LevelFilter};
use crate::{
    conf::{fn_::{fn_conf_keywd::FnConfPointType, fn_conf_kind::FnConfKind}, point_config::name::Name},
    core_::{
        point::point_type::{PointType, ToPoint},
        types::fn_in_out_ref::FnInOutRef,
    },
    services::{
        safe_lock::SafeLock, services::Services,
        task::{
            nested_function::{
                edge_detection::{fn_falling_edge::FnFallingEdge, fn_rising_edge::FnRisingEdge}, export::{fn_export::FnExport, fn_filter::FnFilter, fn_point::FnPoint, fn_to_api_queue::FnToApiQueue}, fn_add::FnAdd, fn_const::FnConst, fn_count::FnCount, fn_debug::FnDebug, fn_ge::FnGe, fn_input::FnInput, fn_point_id::FnPointId, fn_timer::FnTimer, fn_to_int::FnToInt, fn_var::FnVar, functions::Functions, io::fn_retain::FnRetain, sql_metric::SqlMetric
            },
            task_nodes::TaskNodes,
        }
    },
};
///
/// Creates nested functions tree from it config
pub struct NestedFn {}
impl NestedFn {
    ///
    /// Creates nested functions tree from it config
    pub fn new(parent: &Name, tx_id: usize, conf: &mut FnConfKind, task_nodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> FnInOutRef {
        Self::function(parent, tx_id, "", conf, task_nodes, services)
    }
    ///
    ///
    fn function(parent: &Name, tx_id: usize, input_name: &str, conf: &mut FnConfKind, task_nodes: &mut TaskNodes, services: Arc<Mutex<Services>>) -> FnInOutRef {
        let self_id = format!("{}/NestedFn", parent);
        match conf {
            FnConfKind::Fn(conf) => {
                debug!("{}.function | Fn {:?}: {:?}...", self_id, input_name, conf.name.clone());
                let c = conf.name.clone();
                let fn_name= c.clone();
                let fn_name = fn_name.as_str();
                drop(c);
                let fn_name = Functions::from_str(fn_name).unwrap();
                debug!("{}.function | Fn '{}' detected", self_id, fn_name.name());
                debug!("{}.function | fn_conf: {:?}: {:#?}", self_id, conf.name, conf);
                match fn_name {
                    //
                    Functions::Count => {
                        let name = "initial";
                        let input_conf = conf.input_conf(name).map_or(None, |conf| Some(conf));
                        let initial = match input_conf {
                            Some(input_conf) => Some(Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone())),
                            None => None,
                        };
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services);
                        Rc::new(RefCell::new(Box::new(
                            FnCount::new(parent, initial, input),
                        )))
                    }
                    //
                    Functions::Add => {
                        let name = "input1";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input1 = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let name = "input2";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input2 = Self::function(parent, tx_id, name, input_conf, task_nodes, services);
                        Rc::new(RefCell::new(Box::new(
                            FnAdd::new(parent, input1, input2)
                        )))
                    }
                    //
                    Functions::Timer => {
                        let name = "input1";
                        let conf = conf.inputs.get_mut(name).unwrap();
                        let input = Self::function(parent, tx_id, name, conf, task_nodes, services);
                        Rc::new(RefCell::new(Box::new(
                            FnTimer::new(parent, 0.0, input, true)
                        )))
                    }
                    //
                    Functions::ToApiQueue => {
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes ,services.clone());
                        let queue_name = conf.param("queue").unwrap_or_else(|_|
                            panic!("{}.function | Parameter 'queue' - missed in '{}'", self_id, conf.name)
                        ).name();
                        let services_lock = services.slock();
                        let send_queue = services_lock.get_link(&queue_name).unwrap_or_else(|err| {
                            panic!("{}.function | services.get_link error: {:#?}", self_id, err);
                        });
                        Rc::new(RefCell::new(Box::new(
                            FnToApiQueue::new(parent, input, send_queue)
                        )))
                    }
                    //
                    Functions::Ge => {
                        let name = "input1";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input1 = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let name = "input2";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input2 = Self::function(parent, tx_id, name, input_conf, task_nodes, services);
                        Rc::new(RefCell::new(Box::new(
                            FnGe::new(parent, input1, input2)
                        )))
                    }
                    //
                    Functions::SqlMetric => {
                        Rc::new(RefCell::new(Box::new(
                            SqlMetric::new( parent, conf, task_nodes, services)
                        )))
                    }
                    //
                    Functions::PointId => {
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let points = services.slock().points(&parent.join());
                        Rc::new(RefCell::new(Box::new(
                            FnPointId::new(parent, input, points)
                        )))
                    }
                    //
                    Functions::Debug => {
                        let name = "input";
                        let mut inputs = vec![];
                        for (_input_name, input_conf) in &mut conf.inputs {
                            let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                            inputs.push(input);
                        }
                        Rc::new(RefCell::new(Box::new(
                            FnDebug::new(parent, inputs)
                        )))
                    }
                    //
                    Functions::ToInt => {
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        Rc::new(RefCell::new(Box::new(
                            FnToInt::new(parent, input)
                        )))
                    }
                    //
                    Functions::Export => {
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let name = "enable";
                        let input_conf = conf.input_conf(name).map_or(None, |conf| Some(conf));
                        let enable = match input_conf {
                            Some(input_conf) => Some(Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone())),
                            None => None,
                        };
                        let point_conf = match conf.input_conf("conf") {
                            Ok(conf) => {
                                match conf {
                                    FnConfKind::PointConf(conf) => Some(conf.conf.clone()),
                                    _ => panic!("{}.function | Invalid Point config in: {:?}", self_id, conf.name()),
                                }
                            }
                            Err(_) => None,
                        };
                        let send_queue = match conf.param("send-to") {
                            Ok(queue_name) => {
                                let services_lock = services.slock();
                                services_lock.get_link(&queue_name.name()).map_or(None, |send| Some(send))
                            }
                            Err(_) => {
                                warn!("{}.function | Parameter 'send-to' - missed in '{}'", self_id, conf.name);
                                None
                            },
                        };
                        Rc::new(RefCell::new(Box::new(
                            FnExport::new(parent, enable, point_conf, input, send_queue)
                        )))
                    }
                    //
                    Functions::Filter => {
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let name = "pass";
                        let input_conf = conf.input_conf(name).unwrap();
                        let pass = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        let point_conf = match conf.input_conf("conf") {
                            Ok(FnConfKind::PointConf(conf)) => conf.clone(),
                            _ => panic!("{}.function | Invalid Point config in: {:?}", self_id, conf.name),
                        };
                        let send_queue = match conf.param("send-to") {
                            Ok(queue_name) => {
                                let services_lock = services.slock();
                                services_lock.get_link(&queue_name.name()).map_or(None, |send| Some(send))
                            }
                            Err(_) => {
                                warn!("{}.function | Parameter 'send-to' - missed in '{}'", self_id, conf.name);
                                None
                            },
                        };
                        Rc::new(RefCell::new(Box::new(
                            FnFilter::new(parent, point_conf.conf, input, pass, send_queue)
                        )))
                    }
                    //
                    Functions::RisingEdge => {
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        Rc::new(RefCell::new(Box::new(
                            FnRisingEdge::new(parent, input)
                        )))
                    }
                    //
                    Functions::FallingEdge => {
                        let name = "input";
                        let input_conf = conf.input_conf(name).unwrap();
                        let input = Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone());
                        Rc::new(RefCell::new(Box::new(
                            FnFallingEdge::new(parent, input)
                        )))
                    }
                    //
                    Functions::Retain => {
                        let name = "default";
                        let input_conf = conf.input_conf(name).map_or(None, |conf| Some(conf));
                        let default = match input_conf {
                            Some(input_conf) => Some(Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone())),
                            None => None,
                        };
                        let name = "input";
                        let input_conf = conf.input_conf(name).map_or(None, |conf| Some(conf));
                        let input = match input_conf {
                            Some(input_conf) => Some(Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone())),
                            None => None,
                        };
                        let name = "enable";
                        let input_conf = conf.input_conf(name).map_or(None, |conf| Some(conf));
                        let enable = match input_conf {
                            Some(input_conf) => Some(Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone())),
                            None => None,
                        };
                        let name = "every-cycle";
                        let every_cycle = conf.param(name).map_or(false, |param| {
                            match param.name().as_str() {
                                "true" => true,
                                "false" => false,
                                _ => {
                                    warn!("{}.function | Illegal value in 'every_cycle' of '{}'", self_id, conf.name);
                                    false
                                }
                            }
                        });
                        let key = conf.param("key").unwrap_or_else(|_|
                            panic!("{}.function | Parameter 'key' - missed in '{}'", self_id, conf.name)
                        ).name();
                        Rc::new(RefCell::new(Box::new(
                            FnRetain::new(parent, enable, every_cycle, key, default, input)
                        )))
                    }                    
                    //
                    // Add a new function here...
                    _ => panic!("{}.function | Unknown function name: {:?}", self_id, conf.name)
                }
            }
            FnConfKind::Var(conf) => {
                let var_name = conf.name.clone();
                debug!("{}.function | Var: {:?}...", self_id, var_name);
                match conf.inputs.iter_mut().next() {
                    //
                    // New var declaration
                    Some((input_conf_name, input_conf)) => {
                        let var = Self::fn_var(
                            var_name,
                            Self::function(parent, tx_id, input_conf_name, input_conf, task_nodes, services),
                        );
                        debug!("{}.function | Var: {:?}: {:?}", self_id, &conf.name, var.clone());
                        task_nodes.addVar(conf.name.clone(), var.clone());
                        // debug!("{}.function | Var: {:?}", input);
                        var
                    }
                    // Usage declared variable
                    None => {
                        let var = match task_nodes.getVar(&var_name) {
                            Some(var) => var,
                            None => panic!("{}.function | Var {:?} - not declared", self_id, &var_name),
                        }.to_owned();
                        // let var = nodeVar.var();
                        task_nodes.addVarOut(conf.name.clone());
                        var
                    }
                }
            }
            FnConfKind::Const(conf) => {
                let value = conf.name.trim().to_lowercase();
                let name = format!("const {:?} '{}'", conf.type_, value);
                debug!("{}.function | Const: {:?}...", self_id, &name);
                let value = match conf.type_.clone() {
                    FnConfPointType::Bool => value.parse::<bool>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::Int => value.parse::<i64>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::Real => value.parse::<f32>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::Double => value.parse::<f64>().unwrap().to_point(tx_id, &name),
                    FnConfPointType::String => value.to_point(tx_id, &name),
                    FnConfPointType::Any => panic!("{}.function | Const of type 'any' - not supported", self_id),
                    FnConfPointType::Unknown => panic!("{}.function | Point type required", self_id),
                };
                let fn_const = Self::fn_const(&name, value);
                // taskNodes.addInput(inputName, input.clone());
                debug!("{}.function | Const: {:?} - done", self_id, fn_const);
                fn_const
            }
            FnConfKind::Point(conf) => {
                debug!("{}.function | Input (Point<{:?}>): {:?} ({:?})...", self_id, conf.type_, input_name, conf.name);
                let initial = match conf.type_.clone() {
                    FnConfPointType::Bool => false.to_point(tx_id, &conf.name),
                    FnConfPointType::Int => 0.to_point(tx_id, &conf.name),
                    FnConfPointType::Real => 0.0f32.to_point(tx_id, &conf.name),
                    FnConfPointType::Double => 0.0f64.to_point(tx_id, &conf.name),
                    FnConfPointType::String => "".to_point(tx_id, &conf.name),
                    FnConfPointType::Any => false.to_point(tx_id, &conf.name),
                    FnConfPointType::Unknown => panic!("{}.function | Point type required", self_id),
                };
                debug!("{}.function | Input initial: {:?}", self_id, initial);
                let point_name = conf.name.clone();
                task_nodes.addInput(&point_name, Self::fn_input(&point_name, initial, conf.type_.clone()));
                let input = task_nodes.getInput(&point_name).unwrap();
                if log::max_level() == LevelFilter::Trace {
                    debug!("{}.function | input (Point): {:?}", self_id, input);
                }
                input
            }
            FnConfKind::PointConf(conf) => {
                let services_lock = services.slock();
                let send_queue = match &conf.send_to {
                    Some(send_to) => {
                        Some(services_lock.get_link(send_to).unwrap_or_else(|err| {
                            panic!("{}.function | services.get_link error: {:#?}", self_id, err);
                        }))
                    }
                    None => None,
                };
                let name = "input";
                let input_conf = conf.input.as_mut();//.input_conf(name);
                let input = match input_conf {
                    Some(input_conf) => {
                        Some(Self::function(parent, tx_id, name, input_conf, task_nodes, services.clone()))
                    }
                    None => None,
                };
                Rc::new(RefCell::new(Box::new(
                    FnPoint::new(parent, conf.conf.clone(), input, send_queue),
                )))
            }
            FnConfKind::Param(_conf) => {
                panic!("{}.function | Custom parameters are not supported in the nested functions", self_id);
            }
        }
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
    fn fn_const(parent: &str, value: PointType) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnConst::new(parent, value)
        )))
    }
    ///
    ///
    fn fn_input(parent: &str, initial: PointType, type_: FnConfPointType) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnInput::new(parent, initial, type_)
        )))
    }
}

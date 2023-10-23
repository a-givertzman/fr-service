#![allow(non_snake_case)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, clone};
use std::time::Duration;

use log::{info, debug, warn};

use crate::core_::conf::fn_conf_kind::FnConfKind;
use crate::core_::conf::task_config::TaskConfig;
use crate::core_::point::point::Point;
use crate::core_::point::point_type::PointType;
use crate::services::task::nested_function::metric_builder::MetricBuilder;
use crate::services::task::nested_function::nested_fn::NestedFn;
use crate::services::task::task_cycle::TaskCycle;
use crate::services::task::task_stuff::TaskStuff;

use super::nested_function::fn_::FnInOut;
use super::nested_function::fn_inputs::FnInputs;
use super::queue_send::QueueSend;

// pub enum TaskNode {
//     Var(Arc<dyn FnOut>),
//     Metric(Arc<dyn FnOut>),
// }

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    name: String,
    cycle: u64,
    conf: TaskConfig,
    apiQueue: Box<dyn QueueSend<String>>,
    exit: Arc<AtomicBool>,
    // nodes: Arc<Mutex<HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>>>,
}
///
/// 
impl Task {
    ///
    /// 
    pub fn new(cfg: TaskConfig, apiQueue: Box<dyn QueueSend<String>>) ->Self {
        Self {
            name: cfg.name.clone(),
            cycle: cfg.cycle.clone(),
            apiQueue: apiQueue,
            conf: cfg,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn nodes(conf: TaskConfig, inputs: &mut FnInputs) -> HashMap<std::string::String, Rc<RefCell<Box<(dyn FnInOut)>>>> {
        let mut nodes = HashMap::new();
        for (nodeName, mut nodeConf) in conf.nodes {
            debug!("Task.new | node: {:?}", nodeConf.name);
            match nodeConf.fnKind {
                FnConfKind::Metric => {
                    nodes.insert(
                        nodeName.clone(),
                        MetricBuilder::new(&mut nodeConf, inputs),
                    );
                    debug!("Task.new | metricConf: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Fn => {
                    nodes.insert(
                        nodeName.clone(),
                        MetricBuilder::new(&mut nodeConf, inputs),
                    );
                    debug!("Task.new | fnConf: {:?}: {:?}", nodeName, &nodeConf);
                    // NestedFn::new(&mut fnConf, &mut inputs)
                },
                FnConfKind::Var => {
                    nodes.insert(
                        nodeName.clone(),
                        NestedFn::new(&mut nodeConf, inputs),
                    );
                    debug!("Task.new | varConf: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Const => {
                    panic!("Task.new | Const is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Point => {
                    panic!("Task.new | Point is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Param => {
                    debug!("Task.new | custom parameter: {:?}: {:?}", nodeName, &nodeConf);
                }
            }
        }
        nodes
    }
    ///
    /// 
    pub fn run(&mut self) {
        info!("Task({}).run | starting...", self.name);
        let name = self.name.clone();
        let exit = self.exit.clone();
        let cycleInterval = self.cycle;
        let conf = self.conf.clone();
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut cycle = TaskCycle::new(Duration::from_millis(cycleInterval));
            
            let mut taskStuff = FnInputs::new();
            let nodes = Self::nodes(conf, &mut taskStuff);
            debug!("Task({}).run | taskStuff: {:?}", name, taskStuff);

            let mut testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];
            // info!("Task({}).run | prepared", name);
            'inner: loop {
                cycle.start();
                debug!("Task({}).run | calculating step...", name);
                thread::sleep(Duration::from_secs_f32(0.1));
                debug!("Task({}).run | calculating step - done ({:?})", name, cycle.elapsed());
                // TODO impl mathematics here...
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
                match testValues.pop() {
                    Some(value) => {
                        let inputName = "/path/Point.Name";
                        let point = PointType::Float(Point::newFloat(inputName, value));
                        warn!("Task({}).run | input point: {:?}", name, point);
                        match taskStuff.getInput(inputName) {
                            Some(input) => {
                                input.borrow_mut().add(point);
                            },
                            None => {
                                warn!("Task({}).run | input {:?} - not fount", name, inputName);
                            },
                        };
                        for (nodeName, node) in &nodes {
                            node.borrow_mut().out();
                        }
                    },
                    None => {
                        warn!("Task({}).run | No more values", name);
                    },
                };
                cycle.wait();
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
            };
            info!("Task({}).run | stopped", name);
            thread::sleep(Duration::from_secs_f32(2.1));
        }).unwrap();
        info!("Task({}).run | started", self.name);
        // h.join().unwrap();
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::Relaxed);
    }
}
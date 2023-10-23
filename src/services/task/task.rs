#![allow(non_snake_case)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::{thread, clone};
use std::time::Duration;

use log::{info, debug, warn, trace, error};

use crate::core_::conf::fn_conf_kind::FnConfKind;
use crate::core_::conf::task_config::TaskConfig;
use crate::core_::point::point::Point;
use crate::core_::point::point_type::PointType;
use crate::services::task::nested_function::metric_builder::MetricBuilder;
use crate::services::task::nested_function::nested_fn::NestedFn;
use crate::services::task::task_cycle::TaskCycle;

use super::nested_function::fn_::FnInOut;
use super::task_stuff::TaskStuff;

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
    apiQueue: Vec<Sender<String>>,
    recvQueue: Vec<Receiver<PointType>>,
    exit: Arc<AtomicBool>,
    // nodes: Arc<Mutex<HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>>>,
}
///
/// 
impl Task {
    ///
    /// 
    pub fn new(cfg: TaskConfig, apiQueue: Sender<String>, recvQueue: Receiver<PointType>) -> Task {
        Task {
            name: cfg.name.clone(),
            cycle: cfg.cycle.clone(),
            apiQueue: vec![apiQueue],
            recvQueue: vec![recvQueue],
            conf: cfg,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn nodes(conf: TaskConfig, inputs: &mut TaskStuff) -> HashMap<std::string::String, Rc<RefCell<Box<(dyn FnInOut)>>>> {
        let mut nodeIndex = 0;
        let mut nodes = HashMap::new();
        for (_nodeName, mut nodeConf) in conf.nodes {
            let nodeName = nodeConf.name.clone();
            debug!("Task.nodes | node: {:?}", &nodeConf.name);
            match nodeConf.fnKind {
                FnConfKind::Metric => {
                    nodeIndex += 1;
                    nodes.insert(
                        format!("{}-{}", nodeName, nodeIndex),
                        MetricBuilder::new(&mut nodeConf, inputs),
                    );
                    trace!("Task.new | metricConf: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Fn => {
                    nodeIndex += 1;
                    nodes.insert(
                        format!("{}-{}", nodeName, nodeIndex),
                        NestedFn::new(&mut nodeConf, inputs),
                    );
                    trace!("Task.new | fnConf: {:?}: {:?}", nodeName, &nodeConf);
                    // NestedFn::new(&mut fnConf, &mut inputs)
                },
                FnConfKind::Var => {
                    nodes.insert(
                        nodeName.clone(),
                        NestedFn::new(&mut nodeConf, inputs),
                    );
                    trace!("Task.new | varConf: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Const => {
                    panic!("Task.new | Const is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Point => {
                    panic!("Task.new | Point is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Param => {
                    panic!("Task.new | custom parameter: {:?}: {:?}", nodeName, &nodeConf);
                }
            }
        }
        nodes
    }
    ///
    /// 
    pub fn run(&mut self) {
        info!("Task({}).run | starting...", self.name);
        let selfName = self.name.clone();
        let exit = self.exit.clone();
        let cycleInterval = self.cycle;
        let conf = self.conf.clone();
        let apiQueue = self.apiQueue.pop().unwrap();
        let recvQueue = self.recvQueue.pop().unwrap();
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut cycle = TaskCycle::new(Duration::from_millis(cycleInterval));
            let mut taskStuff = TaskStuff::new();
            taskStuff.addSendQueue("apiQueue", apiQueue);
            let nodes = Self::nodes(conf, &mut taskStuff);
            trace!("Task({}).run | taskStuff: {:?}", selfName, taskStuff);
            
            // info!("Task({}).run | prepared", name);
            'inner: loop {
                cycle.start();
                debug!("Task({}).run | calculating step...", selfName);
                thread::sleep(Duration::from_secs_f32(0.1));
                debug!("Task({}).run | calculating step - done ({:?})", selfName, cycle.elapsed());
                // TODO impl mathematics here...
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
                match recvQueue.try_recv() {
                    Ok(point) => {
                        let pointName = point.name();
                        match taskStuff.getInput(&pointName) {
                            Some(input) => {
                                input.borrow_mut().add(point);
                            },
                            None => {
                                warn!("Task({}).run | input {:?} - not fount", selfName, &pointName);
                            },
                        };
                        for (nodeName, node) in &nodes {
                            let out = node.borrow_mut().out();
                            debug!("Task({}).run | node {} out: {:?}", selfName, nodeName, out);
                        }        
                    },
                    Err(err) => {
                        warn!("Task({}).run | Error receiving from queue", err);
                    },
                };
                cycle.wait();
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
            };
            info!("Task({}).run | stopped", selfName);
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
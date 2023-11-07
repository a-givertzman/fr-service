#![allow(non_snake_case)]

use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};

use log::{info, debug, warn, trace};

use crate::{core_::conf::fn_conf_kind::FnConfKind, services::task::{task_nodes::TaskNodes, task_node_type::TaskNodeType}};
use crate::core_::conf::task_config::TaskConfig;
use crate::services::queues::queues::Queues;
use crate::services::task::nested_function::metric_builder::MetricBuilder;
use crate::services::task::nested_function::nested_fn::NestedFn;
use crate::services::task::task_cycle::TaskCycle;

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    name: String,
    cycle: Option<Duration>,
    conf: TaskConfig,
    queues: Vec<Queues>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl Task {
    ///
    /// 
    pub fn new(cfg: TaskConfig, queues: Queues) -> Task {
        Task {
            name: cfg.name.clone(),
            cycle: cfg.cycle.clone(),
            queues: vec![queues],
            // recvQueue: vec![recvQueue],
            conf: cfg,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn nodes(conf: TaskConfig, taskNodes: &mut TaskNodes, queues: &mut Queues) {
        for (_nodeName, mut nodeConf) in conf.nodes {
            let nodeName = nodeConf.name.clone();
            debug!("Task.nodes | node: {:?}", &nodeConf.name);
            taskNodes.beginNewNode();
            let out = match nodeConf.fnKind {
                FnConfKind::Metric => {
                    TaskNodeType::Metric(
                        MetricBuilder::new(&mut nodeConf, taskNodes, queues)
                    )
                },
                FnConfKind::Fn => {
                    TaskNodeType::Metric(
                        NestedFn::new(&mut nodeConf, taskNodes, queues)
                    )
                },
                FnConfKind::Var => {
                    TaskNodeType::Var(
                        NestedFn::new(&mut nodeConf, taskNodes, queues)
                    )
                },
                FnConfKind::Const => {
                    panic!("Task.new | Const is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Point => {
                    panic!("Task.new | Point is not supported in the root of the Task, config: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Param => {
                    panic!("Task.new | custom parameter: {:?}: {:?}", nodeName, &nodeConf);
                },
            };
            taskNodes.finishNewNode(out);
        }
    }
    ///
    /// Tasck main execution loop spawned in the separate thread
    pub fn run(&mut self) {
        info!("Task({}).run | starting...", self.name);
        let selfName = self.name.clone();
        let exit = self.exit.clone();
        let cycleInterval = self.cycle;
        let (cyclic, cycleInterval) = match cycleInterval {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let conf = self.conf.clone();
        let mut queues = self.queues.pop().unwrap();
        let recvQueue = queues.getRecvQueue(&self.conf.recvQueue);
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut cycle = TaskCycle::new(cycleInterval);
            let mut taskNodes = TaskNodes::new();
            Self::nodes(conf, &mut taskNodes, &mut queues);
            debug!("Task({}).run | taskNodes: {:?}", selfName, taskNodes);
            'main: loop {
                cycle.start();
                trace!("Task({}).run | calculation step...", selfName);
                match recvQueue.recv() {
                    Ok(point) => {
                        debug!("Task({}).run | point: {:?}", selfName, &point);                                            
                        let pointName = point.name();
                        match taskNodes.getEvalNode(&pointName) {
                            Some(evalNode) => {
                                evalNode.getInput().borrow_mut().add(point);
                                for evalNodeOut in evalNode.getOuts() {
                                    match evalNodeOut {
                                        TaskNodeType::Var(evalNodeOut) => {
                                            trace!("Task({}).run | evalNode {} - evaluating...", selfName, evalNode.name());                                            
                                            evalNodeOut.borrow_mut().eval();
                                            trace!("Task({}).run | evalNode {} - var evaluated", selfName, evalNode.name());                                            
                                        },
                                        TaskNodeType::Metric(evalNodeOut) => {
                                            trace!("Task({}).run | evalNode {} out...", selfName, evalNode.name());                                            
                                            let out = evalNodeOut.borrow_mut().out();
                                            trace!("Task({}).run | evalNode {} out: {:?}", selfName, evalNode.name(), out);                                            
                                        },
                                    }
                                };
                            },
                            None => {
                                warn!("Task({}).run | evalNode {:?} - not fount", selfName, &pointName);
                            },
                        };
                    },
                    Err(err) => {
                        warn!("Task({}).run | Error receiving from queue: {:?}", selfName, err);
                        break 'main;
                    },
                };
                if exit.load(Ordering::Relaxed) {
                    break 'main;
                }
                trace!("Task({}).run | calculation step - done ({:?})", selfName, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };
            info!("Task({}).run | stopped", selfName);
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
#![allow(non_snake_case)]

use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};

use log::{info, debug, warn, trace, error};

use crate::{core_::{conf::fn_conf_kind::FnConfKind, types::fn_in_out_ref::FnInOutRef}, services::task::{task_nodes::TaskNodes, task_node_type::TaskNodeType}};
use crate::core_::conf::task_config::TaskConfig;
use crate::services::queues::queues::Queues;
use crate::services::task::nested_function::metric_builder::MetricBuilder;
use crate::services::task::nested_function::nested_fn::NestedFn;
use crate::services::task::task_cycle::TaskCycle;

use super::task_node_inputs::TaskNodeInputs;

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    name: String,
    cycle: u64,
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
    // fn nodes(conf: TaskConfig, taskStuff: &mut TaskStuff, queues: &mut Queues) -> HashMap<std::string::String, Rc<RefCell<Box<(dyn FnInOut)>>>> {
    fn nodes(conf: TaskConfig, taskStuff: &mut TaskNodes, queues: &mut Queues) {
        let mut nodeIndex = 0;
        // let mut nodes = HashMap::new();
        for (_nodeName, mut nodeConf) in conf.nodes {
            let nodeName = nodeConf.name.clone();
            debug!("Task.nodes | node: {:?}", &nodeConf.name);
            let mut inputs = TaskNodeInputs::new();
            let out = match nodeConf.fnKind {
                FnConfKind::Metric => {
                    nodeIndex += 1;
                    TaskNodeType::Metric(
                        MetricBuilder::new(&mut nodeConf, &mut inputs, queues)
                    )
                    // nodes.insert(
                    //     format!("{}-{}", nodeName, nodeIndex),
                    //     MetricBuilder::new(&mut nodeConf, inputs, queues),
                    // );
                    // trace!("Task.new | metricConf: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Fn => {
                    nodeIndex += 1;
                    TaskNodeType::Metric(
                        NestedFn::new(&mut nodeConf, &mut inputs, queues)
                    )
                    // nodes.insert(
                    //     format!("{}-{}", nodeName, nodeIndex),
                    //     NestedFn::new(&mut nodeConf, inputs, queues),
                    // );
                    // trace!("Task.new | fnConf: {:?}: {:?}", nodeName, &nodeConf);
                    // NestedFn::new(&mut fnConf, &mut inputs)
                },
                FnConfKind::Var => {
                    TaskNodeType::Var(
                        NestedFn::new(&mut nodeConf, &mut inputs, queues)
                    )
                    // nodes.insert(
                    //     nodeName.clone(),
                    //     NestedFn::new(&mut nodeConf, inputs, queues),
                    // );
                    // trace!("Task.new | varConf: {:?}: {:?}", nodeName, &nodeConf);
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
            taskStuff.insert(&mut inputs, out);
        }
    }
    ///
    /// 
    pub fn run(&mut self) {
        info!("Task({}).run | starting...", self.name);
        let selfName = self.name.clone();
        let exit = self.exit.clone();
        let cycleInterval = self.cycle;
        let cyclic = cycleInterval > 0;
        let conf = self.conf.clone();
        let mut queues = self.queues.pop().unwrap();
        let recvQueue = queues.getRecvQueue(&self.conf.recvQueue);
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut cycle = TaskCycle::new(Duration::from_millis(cycleInterval));
            let mut taskStuff = TaskNodes::new();
            Self::nodes(conf, &mut taskStuff, &mut queues);
            debug!("Task({}).run | taskStuff: {:?}", selfName, taskStuff);
            'main: loop {
                cycle.start();
                trace!("Task({}).run | calculation step...", selfName);
                match recvQueue.recv() {
                    Ok(point) => {
                        let pointName = point.name();
                        match taskStuff.getInput(&pointName) {
                            Some(evalNode) => {
                                evalNode.getInput().borrow_mut().add(point);
                                for evalNodeOut in evalNode.getOuts() {
                                    match evalNodeOut {
                                        TaskNodeType::Var(evalNodeOut) => {
                                            evalNodeOut.borrow_mut().eval();
                                            trace!("Task({}).run | evalNode {} - evaluated", selfName, evalNode.name());                                            
                                        },
                                        TaskNodeType::Metric(evalNodeOut) => {
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
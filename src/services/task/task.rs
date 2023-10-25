#![allow(non_snake_case)]

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};

use log::{info, debug, warn, trace, error};

use crate::{core_::conf::fn_conf_kind::FnConfKind, services::task::task_stuff::TaskStuff};
use crate::core_::conf::task_config::TaskConfig;
use crate::services::queues::queues::Queues;
use crate::services::task::nested_function::metric_builder::MetricBuilder;
use crate::services::task::nested_function::nested_fn::NestedFn;
use crate::services::task::task_cycle::TaskCycle;

use super::nested_function::fn_::FnInOut;
use super::task_stuff_inputs::TaskStuffInputs;

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
    queues: Vec<Queues>,
    // recvQueue: Vec<Receiver<PointType>>,
    exit: Arc<AtomicBool>,
    // nodes: Arc<Mutex<HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>>>,
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
    fn nodes(conf: TaskConfig, taskStuff: &mut TaskStuff, queues: &mut Queues) {
        let mut nodeIndex = 0;
        // let mut nodes = HashMap::new();
        for (_nodeName, mut nodeConf) in conf.nodes {
            let nodeName = nodeConf.name.clone();
            debug!("Task.nodes | node: {:?}", &nodeConf.name);
            let mut inputs = TaskStuffInputs::new();
            match nodeConf.fnKind {
                FnConfKind::Metric => {
                    nodeIndex += 1;
                    let out = MetricBuilder::new(&mut nodeConf, &mut inputs, queues);
                    taskStuff.add(&mut inputs, out);
                    // nodes.insert(
                    //     format!("{}-{}", nodeName, nodeIndex),
                    //     MetricBuilder::new(&mut nodeConf, inputs, queues),
                    // );
                    trace!("Task.new | metricConf: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Fn => {
                    nodeIndex += 1;
                    let out = NestedFn::new(&mut nodeConf, &mut inputs, queues);
                    taskStuff.add(&mut inputs, out);
                    // nodes.insert(
                    //     format!("{}-{}", nodeName, nodeIndex),
                    //     NestedFn::new(&mut nodeConf, inputs, queues),
                    // );
                    trace!("Task.new | fnConf: {:?}: {:?}", nodeName, &nodeConf);
                    // NestedFn::new(&mut fnConf, &mut inputs)
                },
                FnConfKind::Var => {
                    let out = NestedFn::new(&mut nodeConf, &mut inputs, queues);
                    taskStuff.add(&mut inputs, out);
                    // nodes.insert(
                    //     nodeName.clone(),
                    //     NestedFn::new(&mut nodeConf, inputs, queues),
                    // );
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
        // nodes
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
            let mut taskStuff = TaskStuff::new();
            let nodes = Self::nodes(conf, &mut taskStuff, &mut queues);
            trace!("Task({}).run | taskStuff: {:?}", selfName, taskStuff);
            
            // info!("Task({}).run | prepared", name);
            'inner: loop {
                cycle.start();
                trace!("Task({}).run | calculating step...", selfName);
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
                match recvQueue.recv() {
                    Ok(point) => {
                        let pointName = point.name();
                        match taskStuff.getInput(&pointName) {
                            Some(input) => {
                                input.0.borrow_mut().add(point);
                                for (node) in &input.1 {
                                    let out = node.borrow_mut().out();
                                    trace!("Task({}).run | node {} out: {:?}", selfName, pointName, out);
                                };
                            },
                            None => {
                                warn!("Task({}).run | input {:?} - not fount", selfName, &pointName);
                            },
                        };
                    },
                    Err(err) => {
                        warn!("Task({}).run | Error receiving from queue: {:?}", selfName, err);
                        break 'inner;
                    },
                };
                if cyclic {
                    cycle.wait();
                }
                trace!("Task({}).run | calculating step - done ({:?})", selfName, cycle.elapsed());
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
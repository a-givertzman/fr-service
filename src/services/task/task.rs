#![allow(non_snake_case)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, clone};
use std::time::Duration;

use log::{info, debug};

use crate::core_::conf::fn_conf_kind::FnConfKind;
use crate::core_::conf::task_config::{TaskConfig, TaskConfNode};
use crate::services::task::nested_function::metric_builder::MetricBuilder;
use crate::services::task::task_cycle::TaskCycle;
use crate::services::task::task_stuff::TaskStuff;

use super::nested_function::fn_::FnOut;
use super::nested_function::fn_inputs::FnInputs;
use super::queue_send::QueueSend;

pub enum TaskNode {
    Var(Arc<dyn FnOut>),
    Metric(Arc<dyn FnOut>),
}

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    name: String,
    cycle: u64,
    apiQueue: Box<dyn QueueSend<String>>,
    exit: Arc<AtomicBool>,
    nodes: HashMap<String, TaskNode>,
}
///
/// 
impl Task {
    ///
    /// 
    pub fn new(cfg: TaskConfig, apiQueue: Box<dyn QueueSend<String>>) ->Self {
        let mut nodes = HashMap::new();
        let mut inputs = FnInputs::new();
        for (nodeName, mut nodeConf) in cfg.nodes {
            debug!("Task.new | node: {:?}", nodeConf.name);
            match nodeConf.fnKind {
                FnConfKind::Metric => {
                    nodes.insert(
                        nodeName.clone(),
                        MetricBuilder::new(&mut nodeConf, &mut inputs),
                    );
                    debug!("Task.new | metricConf: {:?}: {:?}", nodeName, &nodeConf);
                },
                FnConfKind::Fn => {
                    nodes.insert(
                        nodeName.clone(),
                        MetricBuilder::new(&mut nodeConf, &mut inputs),
                    );
                    debug!("Task.new | fnConf: {:?}: {:?}", nodeName, &nodeConf);
                    // NestedFn::new(&mut fnConf, &mut inputs)
                },
                FnConfKind::Var => {
                    nodes.insert(
                        nodeName.clone(),
                        MetricBuilder::new(&mut nodeConf, &mut inputs),
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
        Self {
            name: cfg.name,
            cycle: cfg.cycle,
            apiQueue: apiQueue,
            exit: Arc::new(AtomicBool::new(false)),
            nodes: nodes,
        }
    }
    ///
    /// 
    pub fn run(&mut self) {
        info!("Task({}).run | starting...", self.name);
        let name = self.name.clone();
        let exit = self.exit.clone();
        let cycleInterval = self.cycle;
        let h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut cycle = TaskCycle::new(Duration::from_millis(cycleInterval));

            let stuff = TaskStuff::new();
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
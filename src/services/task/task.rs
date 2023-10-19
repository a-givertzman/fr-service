use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use log::{info, debug};

use crate::core_::conf::fn_config_type::FnConfigType;
use crate::core_::conf::task_config::{TaskConfig, TaskConfNode};
use crate::core_::nested_function::fn_::FnOutput;
use crate::core_::nested_function::fn_builder::FnBuilder;
use crate::core_::nested_function::metric_builder::MetricBuilder;
use crate::task::{task_cycle::TaskCycle, task_stuff::TaskStuff};

pub enum TaskNode {
    Bool(Arc<dyn FnOutput<bool>>),
    I64(Arc<dyn FnOutput<i64>>),
    F64(Arc<dyn FnOutput<f64>>),
    String(Arc<dyn FnOutput<String>>),
}

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    name: String,
    cycle: u64,
    exit: Arc<AtomicBool>,
    nodes: HashMap<String, TaskNode>,
}
///
/// 
impl Task {
    ///
    /// 
    pub fn new(cfg: TaskConfig) ->Self {
        let mut nodes = HashMap::new();
        for (nodeName, nodeConf) in cfg.nodes {
            match nodeConf {
                TaskConfNode::Fn(fnConf) => {
                    match fnConf.fnType {
                        FnConfigType::Metric => {
                            debug!("Task.new | metricConf: {:?}: {:?}", nodeName, fnConf);
                        },
                        FnConfigType::Fn => {
                            debug!("Task.new | fnConf: {:?}: {:?}", nodeName, fnConf);
                            FnBuilder::new(fnConf)
                        },
                        FnConfigType::Var => {
                            debug!("Task.new | varConf: {:?}: {:?}", nodeName, fnConf);
                        },
                        FnConfigType::Const => {
                            panic!("Task.new | Const is not supported in the root of the Task, config: {:?}: {:?}", nodeName, nodeConf);
                        },
                        FnConfigType::Point => {
                            panic!("Task.new | Point is not supported in the root of the Task, config: {:?}: {:?}", nodeName, nodeConf);
                        },
                    }
                },
                TaskConfNode::Metric(metricConf) => {
                    nodes.insert(
                        nodeName,
                        MetricBuilder::new(metricConf),
                    );
                    debug!("Task.new | metricConf: {:?}: {:?}", nodeName, metricConf)
                },
            }
        }
        Self {
            name: cfg.name,
            cycle: cfg.cycle,
            exit: Arc::new(AtomicBool::new(false)),
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
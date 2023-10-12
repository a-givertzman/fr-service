use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use log::{info, debug};

use crate::core_::conf::fn_config_type::FnConfigType;
use crate::core_::conf::task_config::{TaskConfig, TaskNode};
use crate::task::{task_cycle::TaskCycle, task_stuff::TaskStuff};


/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    name: String,
    cycle: u64,
    exit: Arc<AtomicBool>,
}
///
/// 
impl Task {
    ///
    /// 
    pub fn new(cfg: TaskConfig) ->Self {
        for (nodeName, nodeConf) in cfg.nodes {
            match nodeConf {
                TaskNode::Fn(fnConf) => {
                    match fnConf.fnType {
                        FnConfigType::Fn => {
                            debug!("Task.new | fnConf: {:?}: {:?}", nodeName, fnConf)
                        },
                        FnConfigType::Var => {
                            debug!("Task.new | varConf: {:?}: {:?}", nodeName, fnConf)
                        },
                        FnConfigType::Const => {
                            debug!("Task.new | constConf: {:?}: {:?}", nodeName, fnConf)
                        },
                        FnConfigType::Point => {
                            debug!("Task.new | pointConf: {:?}: {:?}", nodeName, fnConf)
                        },
                        FnConfigType::Metric => {
                            debug!("Task.new | metricConf: {:?}: {:?}", nodeName, fnConf)
                        },
                    }
                },
                TaskNode::Metric(metricConf) => {
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
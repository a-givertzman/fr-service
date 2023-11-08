#![allow(non_snake_case)]

use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};

use log::{info, debug, warn, trace};

use crate::services::task::task_nodes::TaskNodes;
use crate::core_::conf::task_config::TaskConfig;
use crate::services::queues::queues::Queues;
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
            let mut taskNodes = TaskNodes::new(&selfName);
            taskNodes.buildNodes(conf, &mut queues);
            debug!("Task({}).run | taskNodes: {:?}", selfName, taskNodes);
            'main: loop {
                cycle.start();
                trace!("Task({}).run | calculation step...", selfName);
                match recvQueue.recv() {
                    Ok(point) => {
                        debug!("Task({}).run | point: {:?}", selfName, &point);
                        taskNodes.eval(point);
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

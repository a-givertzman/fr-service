#![allow(non_snake_case)]

use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}},
    thread,
    time::Duration,
};

use log::{info, debug, warn, trace};

use crate::services::task::task_nodes::TaskNodes;
use crate::conf::task_config::TaskConfig;
use crate::services::queues::queues::Queues;
use crate::services::task::task_cycle::ServiceCycle;

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    id: String,
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
            id: cfg.name.clone(),
            queues: vec![queues],
            conf: cfg,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Tasck main execution loop spawned in the separate thread
    pub fn run(&mut self) {
        info!("Task({}).run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let mut queues = self.queues.pop().unwrap();
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let recvQueue = queues.getRecvQueue(&conf.recvQueue);
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut taskNodes = TaskNodes::new(&selfId);
            taskNodes.buildNodes(conf, &mut queues);
            debug!("Task({}).run | taskNodes: {:?}", selfId, taskNodes);
            'main: loop {
                cycle.start();
                trace!("Task({}).run | calculation step...", selfId);
                match recvQueue.recv() {
                    Ok(point) => {
                        debug!("Task({}).run | point: {:?}", selfId, &point);
                        taskNodes.eval(point);
                    },
                    Err(err) => {
                        warn!("Task({}).run | Error receiving from queue: {:?}", selfId, err);
                        break 'main;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                trace!("Task({}).run | calculation step - done ({:?})", selfId, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };
            info!("Task({}).run | stopped", selfId);
        }).unwrap();
        info!("Task({}).run | started", self.id);
        // h.join().unwrap();
    }
    ///
    /// Exit thread
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

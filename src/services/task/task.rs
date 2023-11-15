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
    /// Creates new instance of [Task]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: TaskConfig, queues: Queues) -> Task {
        Task {
            id: format!("{}/Task({})", parent.into(), conf.name),
            queues: vec![queues],
            conf,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Tasck main execution loop spawned in the separate thread
    pub fn run(&mut self) {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let mut queues = self.queues.pop().unwrap();
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let recvQueue = queues.getRecvQueue(&conf.recvQueue);
        let _h = thread::Builder::new().name(format!("{} - main", selfId)).spawn(move || {
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut taskNodes = TaskNodes::new(&selfId);
            taskNodes.buildNodes(conf, &mut queues);
            debug!("{}.run | taskNodes: {:?}", selfId, taskNodes);
            'main: loop {
                cycle.start();
                trace!("{}.run | calculation step...", selfId);
                match recvQueue.recv() {
                    Ok(point) => {
                        debug!("{}.run | point: {:?}", selfId, &point);
                        taskNodes.eval(point);
                    },
                    Err(err) => {
                        warn!("{}.run | Error receiving from queue: {:?}", selfId, err);
                        break 'main;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                trace!("{}.run | calculation step - done ({:?})", selfId, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };
            info!("{}.run | stopped", selfId);
        }).unwrap();
        info!("{}.run | started", self.id);
        // h.join().unwrap();
    }
    ///
    /// Exit thread
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

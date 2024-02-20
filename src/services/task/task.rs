#![allow(non_snake_case)]

use std::{
    sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{Sender, Receiver, self, RecvTimeoutError}, Mutex},
    thread::{self, JoinHandle},
    time::Duration, collections::HashMap,
};

use log::{debug, error, info, trace};

use crate::{services::{task::task_nodes::TaskNodes, service::Service, services::Services}, core_::{point::point_type::PointType, constants::constants::RECV_TIMEOUT}, conf::point_config::point_config::PointConfig};
use crate::conf::task_config::TaskConfig;
use crate::services::task::service_cycle::ServiceCycle;

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    id: String,
    inSend: HashMap<String, Sender<PointType>>,
    inRecv: Vec<Receiver<PointType>>,
    services: Arc<Mutex<Services>>,
    conf: TaskConfig,
    exit: Arc<AtomicBool>,
}
///
/// 
impl Task {
    ///
    /// Creates new instance of [Task]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: TaskConfig, services: Arc<Mutex<Services>>) -> Task {
        let (send, recv) = mpsc::channel();
        Task {
            id: format!("{}/Task({})", parent.into(), conf.name),
            inSend: HashMap::from([(conf.rx.clone(), send)]),
            inRecv: vec![recv],
            services,
            conf,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Service for Task {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let inRecv = self.inRecv.pop().unwrap();
        let handle = thread::Builder::new().name(format!("{} - main", self_id)).spawn(move || {
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut taskNodes = TaskNodes::new(&self_id);
            taskNodes.buildNodes(&self_id, conf, services);
            debug!("{}.run | taskNodes: {:?}", self_id, taskNodes);
            'main: loop {
                cycle.start();
                trace!("{}.run | calculation step...", self_id);
                match inRecv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        debug!("{}.run | point: {:?}", self_id, &point);
                        taskNodes.eval(point);
                    },
                    Err(err) => {
                        match err {
                            RecvTimeoutError::Timeout => {
                                debug!("{}.run | {:?}", self_id, err);
                            },
                            RecvTimeoutError::Disconnected => {
                                error!("{}.run | Error receiving from queue: {:?}", self_id, err);
                                break 'main;
                            },
                        }
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                debug!("{}.run | calculation step - done ({:?})", self_id, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };
            info!("{}.run | stopped", self_id);
        });
        info!("{}.run | started", self.id);
        handle
    }
    //
    //
    fn points(&self) -> Vec<PointConfig> {
        self.conf.points()
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        debug!("{}.run | exit: {}", self.id, self.exit.load(Ordering::SeqCst));
    }
}

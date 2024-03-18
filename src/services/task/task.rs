use std::{
    thread,
    sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::{Sender, Receiver, self, RecvTimeoutError}, Mutex},
    time::Duration, collections::HashMap,
};
use log::{debug, error, info, trace, warn};
use crate::{conf::task_config::TaskConfig, core_::object::object::Object, services::service::service_handles::ServiceHandles};
use crate::services::task::service_cycle::ServiceCycle;
use crate::{
    services::{task::task_nodes::TaskNodes, service::service::Service, services::Services}, 
    core_::{point::point_type::PointType, constants::constants::RECV_TIMEOUT}, 
    conf::point_config::point_config::PointConfig,
};

/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    id: String,
    in_send: HashMap<String, Sender<PointType>>,
    in_recv: Vec<Receiver<PointType>>,
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
            in_send: HashMap::from([(conf.rx.clone(), send)]),
            in_recv: vec![recv],
            services,
            conf,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Object for Task {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for Task {
    //
    //
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.in_send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let in_recv = self.in_recv.pop().unwrap();
        let handle = thread::Builder::new().name(format!("{} - main", self_id)).spawn(move || {
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut task_nodes = TaskNodes::new(&self_id);
            task_nodes.buildNodes(&self_id, conf, services);
            debug!("{}.run | taskNodes: {:?}", self_id, task_nodes);
            'main: loop {
                cycle.start();
                trace!("{}.run | calculation step...", self_id);
                match in_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        debug!("{}.run | point: {:?}", self_id, &point);
                        task_nodes.eval(point);
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
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            },
            Err(err) => {
                let message = format!("{}.run | Start faled: {:#?}", self.id, err);
                warn!("{}", message);
                Err(message)
            },
        }        
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

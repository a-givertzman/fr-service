use std::{
    collections::HashMap, fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, RecvTimeoutError, Sender}, Arc, Mutex}, thread, time::Duration
};
use log::{debug, error, info, trace, warn};
use crate::{conf::{point_config::name::Name, task_config::TaskConfig}, core_::object::object::Object, services::{safe_lock::SafeLock, service::service_handles::ServiceHandles}};
use crate::services::task::service_cycle::ServiceCycle;
use crate::{
    services::{task::task_nodes::TaskNodes, service::service::Service, services::Services}, 
    core_::{point::point_type::PointType, constants::constants::RECV_TIMEOUT}, 
    conf::point_config::point_config::PointConfig,
};
///
/// Task implements entity, which provides cyclically (by event) executing calculations
///  - executed in the cycle mode (current impl)
///  - executed event mode (future impl..)
///  - has some number of functions / variables / metrics or additional entities
pub struct Task {
    id: String,
    name: Name,
    in_send: HashMap<String, Sender<PointType>>,
    rx_recv: Vec<Receiver<PointType>>,
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
    pub fn new(conf: TaskConfig, services: Arc<Mutex<Services>>) -> Task {
        let (send, recv) = mpsc::channel();
        Task {
            id: conf.name.join(),
            name: conf.name.clone(),
            in_send: HashMap::from([(conf.rx.clone(), send)]),
            rx_recv: vec![recv],
            services,
            conf,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn subscribe(&mut self, conf: &TaskConfig, services: &Arc<Mutex<Services>>) -> Receiver<PointType> {
        if conf.subscribe.is_empty() {
            self.rx_recv.pop().unwrap()
        } else {
            debug!("{}.subscribe | requesting points...", self.id);
            let points = services.slock().points(&self.id);
            debug!("{}.subscribe | rceived points: {:#?}", self.id, points.len());
            trace!("{}.subscribe | rceived points: {:#?}", self.id, points);
            debug!("{}.subscribe | subscriptions conf: {:#?}", self.id, conf.subscribe);
            let subscriptions = conf.subscribe.with(&points);
            trace!("{}.subscribe | subscriptions: {:#?}", self.id, subscriptions);
            if subscriptions.len() > 1 {
                panic!("{}.run | Error. Task does not supports multiple subscriptions for now: {:#?}.\n\tTry to use single subscription.", self.id, subscriptions);
            } else {
                let subscriptions_first = subscriptions.clone().into_iter().next();
                match subscriptions_first {
                    Some((service_name, Some(points))) => {
                        let (_, rx_recv) = services.slock().subscribe(
                                &service_name,
                                &self.name.join(), 
                                &points,
                            );
                        rx_recv
                    },
                    Some((_, None)) => panic!("{}.run | Error. Task subscription configuration error in:: {:#?}", self.id, subscriptions),
                    None => panic!("{}.run | Error. Task subscription configuration error in:: {:#?}", self.id, subscriptions),
                }
            }
        }
    }
}
///
/// 
impl Object for Task {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> Name {
        self.name.clone()
    }
}
///
/// 
impl Debug for Task {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Task")
            .field("id", &self.id)
            .finish()
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
        let self_name = self.name.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let services = self.services.clone();
        let (cyclic, cycle_interval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let rx_recv = self.subscribe(&conf, &services);
        let handle = thread::Builder::new().name(format!("{} - main", self_id)).spawn(move || {
            let mut cycle = ServiceCycle::new(&self_id, cycle_interval);
            let mut task_nodes = TaskNodes::new(&self_id);
            task_nodes.buildNodes(&self_name, conf, services);
            trace!("{}.run | taskNodes: {:#?}", self_id, task_nodes);
            'main: loop {
                cycle.start();
                trace!("{}.run | calculation step...", self_id);
                match rx_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        debug!("{}.run | point: {:?}", self_id, &point);
                        task_nodes.eval(point);
                        debug!("{}.run | calculation step - done ({:?})", self_id, cycle.elapsed());
                        if cyclic {
                            cycle.wait();
                        }
                    },
                    Err(err) => {
                        match err {
                            RecvTimeoutError::Timeout => {
                                trace!("{}.run | Receive error: {:?}", self_id, err);
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
            };
            info!("{}.run | Stopped", self_id);
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            },
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.id, err);
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
        debug!("{}.run | Exit: {}", self.id, self.exit.load(Ordering::SeqCst));
    }
}

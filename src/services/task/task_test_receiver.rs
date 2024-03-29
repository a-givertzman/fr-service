use std::{collections::HashMap, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread};

use log::{info, warn, trace, debug};

use crate::{core_::{object::object::Object, point::point_type::PointType}, services::service::{service::Service, service_handles::ServiceHandles}};


pub struct TaskTestReceiver {
    id: String,
    iterations: usize, 
    in_send: HashMap<String, Sender<PointType>>,
    in_recv: Vec<Receiver<PointType>>,
    received: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}

impl TaskTestReceiver {
    ///
    /// 
    pub fn new(parent: &str, recv_queue: &str, iterations: usize) -> Self {
        let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        Self {
            id: format!("{}/TaskTestReceiver", parent),
            iterations,
            in_send: HashMap::from([(recv_queue.to_string(), send)]),
            in_recv: vec![recv],
            received: Arc::new(Mutex::new(vec![])),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
    }
}
///
/// 
impl Object for TaskTestReceiver {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for TaskTestReceiver {
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
        let self_id = self.id.clone();
        info!("{}.run | Starting...", self_id);
        let exit = self.exit.clone();
        let received = self.received.clone();
        let mut count = 0;
        // let mut error_count = 0;
        let in_recv = self.in_recv.pop().unwrap();
        let iterations = self.iterations;
        let handle = thread::Builder::new().name(self_id.clone()).spawn(move || {
            // info!("Task({}).run | prepared", name);
            'inner: loop {
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
                match in_recv.recv() {
                    Ok(point) => {
                        debug!("{}.run | received: {}, (value: {:?})", self_id, count, point.value());
                        trace!("{}.run | received SQL: {:?}", self_id, point.as_string().value);
                        // debug!("{}.run | value: {}\treceived SQL: {:?}", value, sql);
                        count += 1;
                        received.lock().unwrap().push(point.clone());
                        if count >= iterations {
                            break 'inner;
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | Error receiving from queue: {:?}", self_id, err);
                        // error_count += 1;
                        // if errorCount > 10 {
                        //     warn!("{}.run | Error receiving count > 10, exit...", self_id);
                        //     break 'inner;
                        // }        
                    },
                };
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
            };
            info!("{}.run | received {} SQL's", self_id, count);
            info!("{}.run | exit", self_id);
            // thread::sleep(Duration::from_secs_f32(2.1));
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
    fn exit(&self) {
        self.exit.store(true, Ordering::Relaxed);
    }
    // pub fn getInputValues(&mut self) -> Receiver<PointType> {
    //     self.recv.pop().unwrap()
    // }
}
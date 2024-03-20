use std::{collections::HashMap, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread};

use log::{info, trace, warn};

use crate::{core_::{constants::constants::RECV_TIMEOUT, object::object::Object, point::point_type::PointType}, services::service::{service::Service, service_handles::ServiceHandles}};


pub struct MockRecvService {
    id: String,
    rx_send: HashMap<String, Sender<PointType>>,
    rx_recv: Vec<Receiver<PointType>>,
    received: Arc<Mutex<Vec<PointType>>>,
    recv_limit: Option<usize>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockRecvService {
    pub fn new(parent: impl Into<String>, rx_queue: &str, recv_limit: Option<usize>) -> Self {
        let self_id = format!("{}/MockRecvService", parent.into());
        let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: self_id.clone(),
            rx_send: HashMap::from([(rx_queue.to_string(), send)]),
            rx_recv: vec![recv],
            received: Arc::new(Mutex::new(vec![])),
            recv_limit,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    // pub fn id(&self) -> String {
    //     self.id.clone()
    // }
    ///
    /// 
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
    }
}
///
/// 
impl Object for MockRecvService {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for MockRecvService {
    //
    //
    fn get_link(&mut self, name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        match self.rx_send.get(name) {
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
        let in_recv = self.rx_recv.pop().unwrap();
        let received = self.received.clone();
        let recv_limit = self.recv_limit.clone();
        let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            match recv_limit {
                Some(recv_limit) => {
                    let mut received_count = 0;
                    loop {
                        match in_recv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", self_id, point);
                                received.lock().unwrap().push(point);
                                received_count += 1;
                            },
                            Err(_) => {},
                        };
                        if received_count >= recv_limit {
                            break;
                        }
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                },
                None => {
                    loop {
                        match in_recv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", self_id, point);
                                received.lock().unwrap().push(point);
                            },
                            Err(_) => {},
                        };
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                },
            }
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
        self.exit.store(true, Ordering::SeqCst);
    }
}
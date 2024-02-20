#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}, time::Duration};

use log::{info, warn, debug, trace};
use testing::entities::test_value::Value;
use crate::{core_::point::point_type::{PointType, ToPoint}, services::{service::Service, services::Services}};


pub struct MockSendService {
    id: String,
    // rxSend: HashMap<String, Sender<PointType>>,
    // inRecv: Vec<Receiver<PointType>>,
    // txSend: HashMap<String, Sender<PointType>>,
    // outRecv: Vec<Receiver<PointType>>,
    sendQueue: String,
    services: Arc<Mutex<Services>>,
    test_data: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    delay: Option<Duration>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockSendService {
    pub fn new(parent: impl Into<String>, _recvQueue: &str, sendQueue: &str, services: Arc<Mutex<Services>>, test_data: Vec<Value>, delay: Option<Duration>) -> Self {
        let self_id = format!("{}/MockSendService", parent.into());
        // let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: self_id.clone(),
            // rxSend: HashMap::from([(recvQueue.to_string(), send)]),
            // inRecv: vec![recv],
            // txSend: HashMap::new(),
            // outRecv: vec![],
            sendQueue: sendQueue.to_string(),
            services,
            test_data,
            sent: Arc::new(Mutex::new(vec![])),
            delay,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn id(&self) -> String {
        self.id.clone()
    }
    ///
    /// 
    pub fn sent(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.sent.clone()
    }
}
///
/// 
impl Service for MockSendService {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn get_link(&mut self, _name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        panic!("{}.getLink | Does not support getLink", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        debug!("{}.run | Lock services...", self_id);
        let services = self.services.lock().unwrap();
        debug!("{}.run | Lock services - ok", self_id);
        let txSend = services.getLink(&self.sendQueue);
        let test_data = self.test_data.clone();
        let sent = self.sent.clone();
        let delay = self.delay.clone();
        let _handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            for value in test_data {
                let point = value.to_point(0,&format!("{}/test", self_id));
                match txSend.send(point.clone()) {
                    Ok(_) => {
                        trace!("{}.run | send: {:?}", self_id, point);
                        sent.lock().unwrap().push(point);
                    },
                    Err(err) => {
                        warn!("{}.run | send error: {:?}", self_id, err);
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                match delay {
                    Some(duration) => {
                        thread::sleep(duration);
                    },
                    None => {},
                }
            }
        });
        _handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
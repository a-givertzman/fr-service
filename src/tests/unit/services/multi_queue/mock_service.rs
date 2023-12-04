#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}};

use log::{info, warn};

use crate::{core_::{point::point_type::PointType, testing::test_stuff::test_value::Value}, services::{services::Services, service::Service}};


pub struct MockService {
    id: String,
    inSend: HashMap<String, Sender<PointType>>,
    inRecv: Vec<Receiver<PointType>>,
    // outSend: HashMap<String, Sender<PointType>>,
    // outRecv: Vec<Receiver<PointType>>,
    sendQueue: String,
    services: Arc<Mutex<Services>>,
    testData: Arc<Mutex<Vec<Value>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockService {
    pub fn new(parent: impl Into<String>, recvQueue: &str, sendQueue: &str, services: Arc<Mutex<Services>>, testData: Arc<Mutex<Vec<Value>>>) -> Self {
        let selfId = format!("{}/MultiQueue", parent.into());
        let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: selfId.clone(),
            inSend: HashMap::from([(recvQueue.to_string(), send)]),
            inRecv: vec![recv],
            // outSend: HashMap::new(),
            // outRecv: vec![],
            sendQueue: sendQueue.to_string(),
            services,
            testData,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn id(&self) -> String {
        self.id.clone()
    }
}
///
/// 
impl Service for MockService {
    //
    //
    fn getLink(&self, name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let outSendService = self.services.lock().unwrap().get(&self.sendQueue);
        let outSend = outSendService.lock().unwrap().getLink(&self.sendQueue);
        let testData = self.testData.clone();
        let _handle = thread::Builder::new().name(format!("{} - MultiQueue.run", selfId)).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let testData = testData.lock().unwrap();
            loop {
                for value in testData.iter() {
                    let point = value.toPoint(&format!("{}/test", selfId));
                    if let Err(err) = outSend.send(point) {
                        warn!("{}.run | send error: {:?}", selfId, err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
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
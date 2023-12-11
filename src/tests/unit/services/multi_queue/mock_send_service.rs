#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{mpsc::{Sender, self}, Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}};

use log::{info, warn, debug, trace};

use crate::{core_::{point::point_type::PointType, testing::test_stuff::test_value::Value}, services::{services::Services, service::Service}};


pub struct MockSendService {
    id: String,
    rxSend: HashMap<String, Sender<PointType>>,
    // inRecv: Vec<Receiver<PointType>>,
    // outSend: HashMap<String, Sender<PointType>>,
    // outRecv: Vec<Receiver<PointType>>,
    sendQueue: String,
    services: Arc<Mutex<Services>>,
    testData: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockSendService {
    pub fn new(parent: impl Into<String>, recvQueue: &str, sendQueue: &str, services: Arc<Mutex<Services>>, testData: Vec<Value>) -> Self {
        let selfId = format!("{}/MockSendService", parent.into());
        let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: selfId.clone(),
            rxSend: HashMap::from([(recvQueue.to_string(), send)]),
            // inRecv: vec![recv],
            // outSend: HashMap::new(),
            // outRecv: vec![],
            sendQueue: sendQueue.to_string(),
            services,
            testData,
            sent: Arc::new(Mutex::new(vec![])),
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
    fn getLink(&mut self, name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        match self.rxSend.get(name) {
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
        debug!("{}.run | Getting services...", selfId);
        let services = self.services.lock().unwrap();
        debug!("{}.run | Getting services - ok", selfId);
        let txSend = services.getLink(&self.sendQueue);
        let testData = self.testData.clone();
        let sent = self.sent.clone();
        let _handle = thread::Builder::new().name(format!("{}.run", selfId)).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            // let mut testData = testData.lock().unwrap();
            for value in testData {
                let point = value.toPoint(0,&format!("{}/test", selfId));
                match txSend.send(point.clone()) {
                    Ok(_) => {
                        trace!("{}.run | send: {:?}", selfId, point);
                        sent.lock().unwrap().push(point);
                    },
                    Err(err) => {
                        warn!("{}.run | send error: {:?}", selfId, err);
                    },
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
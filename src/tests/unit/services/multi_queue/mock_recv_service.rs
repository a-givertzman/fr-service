#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}, time::Duration};

use log::{info, trace};

use crate::{core_::point::point_type::PointType, services::{services::Services, service::Service}};


pub struct MockRecvService {
    id: String,
    inSend: HashMap<String, Sender<PointType>>,
    inRecv: Vec<Receiver<PointType>>,
    // outSend: HashMap<String, Sender<PointType>>,
    // outRecv: Vec<Receiver<PointType>>,
    // sendQueue: String,
    services: Arc<Mutex<Services>>,
    received: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockRecvService {
    pub fn new(parent: impl Into<String>, recvQueue: &str, sendQueue: &str, services: Arc<Mutex<Services>>) -> Self {
        let selfId = format!("{}/MockRecvService", parent.into());
        let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: selfId.clone(),
            inSend: HashMap::from([(recvQueue.to_string(), send)]),
            inRecv: vec![recv],
            // outSend: HashMap::new(),
            // outRecv: vec![],
            // sendQueue: sendQueue.to_string(),
            services,
            received: Arc::new(Mutex::new(vec![])),
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
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
    }
}
///
/// 
impl Service for MockRecvService {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn getLink(&mut self, name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
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
        let inRecv = self.inRecv.pop().unwrap();
        let received = self.received.clone();
        // let parts: Vec<&str> = self.sendQueue.split(".").collect();
        // let outSendServiceName = parts[0];
        // let outSendQueueName = parts[1];
        // debug!("{}.run | Getting services...", selfId);
        // let services = self.services.lock().unwrap();
        // debug!("{}.run | Getting services - ok", selfId);
        // let outSendService = services.get(&outSendServiceName);
        // let outSend = outSendService.lock().unwrap().getLink(&outSendQueueName);
        // let testData = self.testData.clone();
        let _handle = thread::Builder::new().name(format!("{} - MultiQueue.run", selfId)).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            // let testData = testData.lock().unwrap();
            // for value in testData.iter() {
            //     let point = value.toPoint(&format!("{}/test", selfId));
            //     if let Err(err) = outSend.send(point) {
            //         warn!("{}.run | send error: {:?}", selfId, err);
            //     }
            // }
            loop {
                match inRecv.recv_timeout(Duration::from_millis(100)) {
                    Ok(point) => {
                        trace!("{}.run | received: {:?}", selfId, point);
                        received.lock().unwrap().push(point);
                    },
                    Err(_) => {},
                };
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
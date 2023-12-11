#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{mpsc::{Sender, self, Receiver}, Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}, time::Duration};

use log::{info, warn, debug, trace};

use crate::{core_::{point::{point_type::PointType, point_tx_id::PointTxId}, testing::test_stuff::test_value::Value}, services::{services::Services, service::Service}};


pub struct MockRecvSendService {
    id: String,
    inSend: HashMap<String, Sender<PointType>>,
    inRecv: Vec<Receiver<PointType>>,
    // outSend: HashMap<String, Sender<PointType>>,
    // outRecv: Vec<Receiver<PointType>>,
    sendQueue: String,
    services: Arc<Mutex<Services>>,
    testData: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    received: Arc<Mutex<Vec<PointType>>>,
    recvLimit: Option<usize>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockRecvSendService {
    pub fn new(parent: impl Into<String>, recvQueue: &str, sendQueue: &str, services: Arc<Mutex<Services>>, testData: Vec<Value>, recvLimit: Option<usize>) -> Self {
        let selfId = format!("{}/MockRecvSendService", parent.into());
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
            sent: Arc::new(Mutex::new(vec![])),
            received: Arc::new(Mutex::new(vec![])),
            recvLimit,
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
    ///
    /// 
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
    }
}
///
/// 
impl Service for MockRecvSendService {
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
        let recvQueueParts: Vec<&str> = self.sendQueue.split(".").collect();
        let outSendServiceName = recvQueueParts[0];
        let outSendQueueName = recvQueueParts[1];
        debug!("{}.run | Getting services...", selfId);
        let services = self.services.lock().unwrap();
        debug!("{}.run | Getting services - ok", selfId);
        let outSendService = services.get(&outSendServiceName);
        let outSend = outSendService.lock().unwrap().getLink(&outSendQueueName);
        let testData = self.testData.clone();
        let sent = self.sent.clone();
        let _handle = thread::Builder::new().name(format!("{} - MultiQueue.run", selfId)).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let txId = PointTxId::fromStr(&selfId);
            for value in testData.iter() {
                let point = value.toPoint(txId,&format!("{}/test", selfId));
                match outSend.send(point.clone()) {
                    Ok(_) => {
                        trace!("{}.run | send: {:?}", selfId, point);
                        sent.lock().unwrap().push(point);
                    },
                    Err(err) => {
                        warn!("{}.run | send error: {:?}", selfId, err);
                    },
                }
                loop {
                    if exit.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
        });
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let inRecv = self.inRecv.pop().unwrap();
        let received = self.received.clone();
        let recvLimit = self.recvLimit.clone();
        let handle = thread::Builder::new().name(format!("{} - MultiQueue.run", selfId)).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            match recvLimit {
                Some(recvLimit) => {
                    let mut receivedCount = 0;
                    loop {
                        match inRecv.recv_timeout(Duration::from_millis(100)) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", selfId, point);
                                received.lock().unwrap().push(point);
                                receivedCount += 1;
                            },
                            Err(_) => {},
                        };
                        if receivedCount >= recvLimit {
                            break;
                        }
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                },
                None => {
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
                },
            }
        });
        handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
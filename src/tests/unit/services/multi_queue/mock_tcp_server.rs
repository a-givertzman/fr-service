#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{mpsc::{Sender, self, Receiver}, Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}, time::Duration};

use log::{info, warn, debug, trace};

use crate::{core_::{point::{point_type::PointType, point_tx_id::PointTxId}, testing::test_stuff::test_value::Value}, services::{services::Services, service::Service, queue_name::QueueName}};


pub struct MockTcpServer {
    id: String,
    rxSend: HashMap<String, Sender<PointType>>,
    multiQueue: String,
    services: Arc<Mutex<Services>>,
    testData: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    received: Arc<Mutex<Vec<PointType>>>,
    recvLimit: Option<usize>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockTcpServer {
    pub fn new(parent: impl Into<String>, multiQueue: &str, services: Arc<Mutex<Services>>, testData: Vec<Value>, recvLimit: Option<usize>) -> Self {
        let selfId = format!("{}/MockTcpServer", parent.into());
        // let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: selfId.clone(),
            rxSend: HashMap::new(),
            multiQueue: multiQueue.to_string(),
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
impl Service for MockTcpServer {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn getLink(&mut self, name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        panic!("{}.getLink | Does not support static producer", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let mqServiceName = QueueName::new(&self.multiQueue);
        let mqServiceName = mqServiceName.service();
        debug!("{}.run | Getting services...", selfId);
        let mqService = self.services.lock().unwrap().get(mqServiceName);
        debug!("{}.run | Getting services - ok", selfId);
        let rxRecv = mqService.lock().unwrap().subscribe(&selfId, &vec![]);
        let received = self.received.clone();
        let recvLimit = self.recvLimit.clone();
        let handle = thread::Builder::new().name(format!("{}.run | Recv", selfId)).spawn(move || {
            info!("{}.run | Preparing thread Recv - ok", selfId);
            match recvLimit {
                Some(recvLimit) => {
                    let mut receivedCount = 0;
                    loop {
                        match rxRecv.recv_timeout(Duration::from_millis(1000)) {
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
                        match rxRecv.recv_timeout(Duration::from_millis(100)) {
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
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        debug!("{}.run | Getting services...", selfId);
        let txSend = self.services.lock().unwrap().getLink(&self.multiQueue);
        debug!("{}.run | Getting services - ok", selfId);
        let testData = self.testData.clone();
        let sent = self.sent.clone();
        let _handle = thread::Builder::new().name(format!("{}.run | Send", selfId)).spawn(move || {
            info!("{}.run | Preparing thread Send - ok", selfId);
            let txId = PointTxId::fromStr(&selfId);
            for value in testData.iter() {
                let point = value.toPoint(txId,&format!("{}/test", selfId));
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
        handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
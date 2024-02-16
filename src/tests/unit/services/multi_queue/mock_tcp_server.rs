#![allow(non_snake_case)]

use log::{info, warn, debug, trace};
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}};
use testing::entities::test_value::Value;
use crate::{
    core_::{constants::constants::RECV_TIMEOUT, point::{point_tx_id::PointTxId, point_type::{PointType, ToPoint}}}, 
    services::{queue_name::QueueName, service::Service, services::Services},
};


pub struct MockTcpServer {
    id: String,
    // rxSend: HashMap<String, Sender<PointType>>,
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
            // rxSend: HashMap::new(),
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
    // pub fn sent(&self) -> Arc<Mutex<Vec<PointType>>> {
    //     self.sent.clone()
    // }
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
    fn get_link(&mut self, _name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
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
        debug!("{}.run | Lock services...", selfId);
        let rxRecv = self.services.lock().unwrap().subscribe(mqServiceName, &selfId, &vec![]);
        let txSend = self.services.lock().unwrap().getLink(&self.multiQueue);
        debug!("{}.run | Lock services - ok", selfId);
        let received = self.received.clone();
        let recvLimit = self.recvLimit.clone();
        let _handle = thread::Builder::new().name(format!("{}.run | Recv", selfId)).spawn(move || {
            info!("{}.run | Preparing thread Recv - ok", selfId);
            match recvLimit {
                Some(recvLimit) => {
                    let mut receivedCount = 0;
                    loop {
                        match rxRecv.recv_timeout(RECV_TIMEOUT) {
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
                        match rxRecv.recv_timeout(RECV_TIMEOUT) {
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
            info!("{}.run | Exit thread Recv", selfId);
        });
        let selfId = self.id.clone();
        let txId = PointTxId::fromStr(&selfId);
        let exit = self.exit.clone();
        let testData = self.testData.clone();
        let sent = self.sent.clone();
        let handle = thread::Builder::new().name(format!("{}.run | Send", selfId)).spawn(move || {
            info!("{}.run | Preparing thread Send - ok", selfId);
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
            info!("{}.run | Exit thread Send", selfId);
        });
        handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

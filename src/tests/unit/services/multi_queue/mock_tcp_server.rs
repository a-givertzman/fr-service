#![allow(non_snake_case)]

use log::{info, warn, debug, trace};
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread};
use testing::entities::test_value::Value;
use crate::{
    core_::{constants::constants::RECV_TIMEOUT, object::object::Object, point::{point_tx_id::PointTxId, point_type::{PointType, ToPoint}}}, 
    services::{queue_name::QueueName, service::{service::Service, service_handles::ServiceHandles}, services::Services},
};


pub struct MockTcpServer {
    id: String,
    // rxSend: HashMap<String, Sender<PointType>>,
    multiQueue: String,
    services: Arc<Mutex<Services>>,
    test_data: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    received: Arc<Mutex<Vec<PointType>>>,
    recvLimit: Option<usize>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockTcpServer {
    pub fn new(parent: impl Into<String>, multiQueue: &str, services: Arc<Mutex<Services>>, test_data: Vec<Value>, recvLimit: Option<usize>) -> Self {
        let self_id = format!("{}/MockTcpServer", parent.into());
        // let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: self_id.clone(),
            // rxSend: HashMap::new(),
            multiQueue: multiQueue.to_string(),
            services,
            test_data,
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
impl Object for MockTcpServer {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for MockTcpServer {
    //
    //
    fn get_link(&mut self, _name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        panic!("{}.get_link | Does not support static producer", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }
    }
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let mqServiceName = QueueName::new(&self.multiQueue);
        let mqServiceName = mqServiceName.service();
        debug!("{}.run | Lock services...", self_id);
        let (_, rxRecv) = self.services.lock().unwrap().subscribe(mqServiceName, &self_id, &vec![]);
        let txSend = self.services.lock().unwrap().get_link(&self.multiQueue);
        debug!("{}.run | Lock services - ok", self_id);
        let received = self.received.clone();
        let recvLimit = self.recvLimit.clone();
        let handle_recv = thread::Builder::new().name(format!("{}.run | Recv", self_id)).spawn(move || {
            info!("{}.run | Preparing thread Recv - ok", self_id);
            match recvLimit {
                Some(recvLimit) => {
                    let mut receivedCount = 0;
                    loop {
                        match rxRecv.recv_timeout(RECV_TIMEOUT) {
                            Ok(point) => {
                                trace!("{}.run | received: {:?}", self_id, point);
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
            info!("{}.run | Exit thread Recv", self_id);
        });
        let self_id = self.id.clone();
        let txId = PointTxId::fromStr(&self_id);
        let exit = self.exit.clone();
        let test_data = self.test_data.clone();
        let sent = self.sent.clone();
        let handle_send = thread::Builder::new().name(format!("{}.run | Send", self_id)).spawn(move || {
            info!("{}.run | Preparing thread Send - ok", self_id);
            for value in test_data.iter() {
                let point = value.to_point(txId,&format!("{}/test", self_id));
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
            }
            info!("{}.run | Exit thread Send", self_id);
        });
        match (handle_recv, handle_send) {
            (Ok(handle_recv), Ok(handle_send)) => Ok(ServiceHandles::new(vec![
                (format!("{}/read", self.id), handle_recv),
                (format!("{}/write", self.id), handle_send),
                ])),
            // TODO Exit 'write if read returns error'
            (Ok(_handle_recv), Err(err)) => Err(format!("{}.run | Error starting inner thread 'send': {:#?}", self.id, err)),
            // TODO Exit 'read if write returns error'
            (Err(err), Ok(_handle_send)) => Err(format!("{}.run | Error starting inner thread 'recv': {:#?}", self.id, err)),
            (Err(read_err), Err(write_err)) => Err(format!("{}.run | Error starting inner thread: \n\t  recv: {:#?}\n\t send: {:#?}", self.id, read_err, write_err)),
        }

    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

#![allow(non_snake_case)]

use std::{collections::HashMap, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{self, JoinHandle}, time::Duration};

use log::{info, trace};

use crate::{core_::point::point_type::PointType, services::service::Service};


pub struct MockRecvService {
    id: String,
    rxSend: HashMap<String, Sender<PointType>>,
    rxRecv: Vec<Receiver<PointType>>,
    received: Arc<Mutex<Vec<PointType>>>,
    recvLimit: Option<usize>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockRecvService {
    pub fn new(parent: impl Into<String>, rxQueue: &str, recvLimit: Option<usize>) -> Self {
        let selfId = format!("{}/MockRecvService", parent.into());
        let (send, recv) = mpsc::channel::<PointType>();
        Self {
            id: selfId.clone(),
            rxSend: HashMap::from([(rxQueue.to_string(), send)]),
            rxRecv: vec![recv],
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
        let inRecv = self.rxRecv.pop().unwrap();
        let received = self.received.clone();
        let recvLimit = self.recvLimit.clone();
        let handle = thread::Builder::new().name(format!("{}.run", selfId)).spawn(move || {
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
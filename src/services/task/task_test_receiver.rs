#![allow(non_snake_case)]

use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}, collections::HashMap};

use log::{info, warn, trace, debug};

use crate::{core_::point::point_type::PointType, services::service::Service};


pub struct TaskTestReceiver {
    id: String,
    iterations: usize, 
    inSend: HashMap<String, Sender<PointType>>,
    inRecv: Vec<Receiver<PointType>>,
    received: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}

impl TaskTestReceiver {
    ///
    /// 
    pub fn new(parent: &str, recvQueue: &str, iterations: usize) -> Self {
        let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        Self {
            id: format!("{}/TaskTestReceiver", parent),
            iterations,
            inSend: HashMap::from([(recvQueue.to_string(), send)]),
            inRecv: vec![recv],
            received: Arc::new(Mutex::new(vec![])),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
    }
}
///
/// 
impl Service for TaskTestReceiver {
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }        
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        let selfId = self.id.clone();
        info!("{}.run | starting...", selfId);
        let exit = self.exit.clone();
        let received = self.received.clone();
        let mut count = 0;
        let mut errorCount = 0;
        let inRecv = self.inRecv.pop().unwrap();
        let iterations = self.iterations;
        let handle = thread::Builder::new().name(selfId.clone()).spawn(move || {
            // info!("Task({}).run | prepared", name);
            'inner: loop {
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
                match inRecv.recv() {
                    Ok(point) => {
                        count += 1;
                        received.lock().unwrap().push(point.clone());
                        if count >= iterations {
                            break 'inner;
                        }
                        debug!("{}.run | received: {}, (value: {:?})", selfId, count, point.value());
                        trace!("{}.run | received SQL: {:?}", selfId, point.asString().value);
                        // debug!("{}.run | value: {}\treceived SQL: {:?}", value, sql);
                    },
                    Err(err) => {
                        warn!("{}.run | Error receiving from queue: {:?}", selfId, err);
                        errorCount += 1;
                        if errorCount > 10 {
                            warn!("{}.run | Error receiving count > 10, exit...", selfId);
                            break 'inner;
                        }        
                    },
                };
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
            };
            info!("{}.run | received {} SQL's", selfId, count);
            info!("{}.run | exit", selfId);
            // thread::sleep(Duration::from_secs_f32(2.1));
        });
        info!("{}.run | starting - ok", self.id);
        handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::Relaxed);
    }
    // pub fn getInputValues(&mut self) -> Receiver<PointType> {
    //     self.recv.pop().unwrap()
    // }
}
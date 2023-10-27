#![allow(non_snake_case)]

use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering, AtomicUsize}}, thread::{self, JoinHandle}};

use log::{info, debug, warn, trace};

use crate::core_::point::point_type::PointType;


pub struct TaskTestReceiver {
    exit: Arc<AtomicBool>,
    received: Arc<AtomicUsize>,
    handle: Vec<JoinHandle<()>>,
    recv: Vec<Receiver<PointType>>
}

impl TaskTestReceiver {
    pub fn new() -> Self {
        Self {
            exit: Arc::new(AtomicBool::new(false)),
            received: Arc::new(AtomicUsize::new(0)),
            handle: vec![],
            recv: vec![],
        }
    }
    pub fn run(&mut self, recvQueue: Receiver<PointType>, iterations: usize, testValues: Vec<f64>) {
        info!("TaskTestReceiver.run | starting...");
        let exit = self.exit.clone();
        let received = self.received.clone();
        // let mut testValues = testValues.clone();
        let mut count = 0;
        let mut errorCount = 0;
        let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        self.recv.push(recv);
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            // info!("Task({}).run | prepared", name);
            'inner: loop {
                // TODO impl mathematics here...
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
                match recvQueue.recv() {
                    Ok(sql) => {
                        count += 1;
                        received.store(count, Ordering::Relaxed);
                        if count >= iterations {
                            break 'inner;
                        }
                        let _r = send.send(sql.clone());
                        trace!("TaskTestReceiver.run | received SQL: {:?}", sql.asString().value);
                        // debug!("TaskTestReceiver.run | value: {}\treceived SQL: {:?}", value, sql);
                        // assert!()
                    },
                    Err(err) => {
                        warn!("TaskTestReceiver.run | Error receiving from queue: {:?}", err);
                        errorCount += 1;
                        if errorCount > 10 {
                            warn!("TaskTestReceiver.run | Error receiving count > 10, exit...");
                            break 'inner;
                        }        
                    },
                };
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
            };
            info!("TaskTestReceiver.run | received {} SQL's", count);
            info!("TaskTestReceiver.run | stopped");
            // thread::sleep(Duration::from_secs_f32(2.1));
        }).unwrap();
        self.handle.push(_h);
    }
    pub fn exit(&mut self) {
        self.exit.store(true, Ordering::Relaxed);
    }
    pub fn received(&self) -> usize {
        self.received.load(Ordering::Relaxed)
    }
    pub fn join(&mut self) {
        match self.handle.pop() {
            Some(handle) => {
            handle.join().unwrap()
            },
            None => {},
        };
    }
    pub fn getInputValues(&mut self) -> Receiver<PointType> {
        self.recv.pop().unwrap()
    }
}
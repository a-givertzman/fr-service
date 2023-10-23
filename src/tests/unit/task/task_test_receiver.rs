#![allow(non_snake_case)]

use std::{sync::{mpsc::Receiver, Arc, atomic::{AtomicBool, Ordering, AtomicUsize}}, thread::{self, JoinHandle}};

use log::{info, debug, warn, trace};


pub struct TaskTestReceiver {
    exit: Arc<AtomicBool>,
    received: Arc<AtomicUsize>,
    handle: Option<JoinHandle<()>>,
}

impl TaskTestReceiver {
    pub fn new() -> Self {
        Self {
            exit: Arc::new(AtomicBool::new(false)),
            received: Arc::new(AtomicUsize::new(0)),
            handle: None,
        }
    }
    pub fn run(&mut self, recvQueue: Receiver<String>, testValues: Vec<f64>) {
        info!("TaskTestReceiver.run | starting...");
        let exit = self.exit.clone();
        let received = self.received.clone();
        // let mut testValues = testValues.clone();
        let mut count = 0;
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
                        debug!("TaskTestReceiver.run | received SQL: {:?}", sql);
                        // debug!("TaskTestReceiver.run | value: {}\treceived SQL: {:?}", value, sql);
                        // assert!()
                    },
                    Err(err) => {
                        warn!("TaskTestReceiver.run | Error receiving from queue: {:?}", err);
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
        self.handle = Some(_h);
    }
    pub fn exit(&mut self) {
        self.exit.store(true, Ordering::Relaxed);
    }
    pub fn received(&self) -> usize {
        self.received.load(Ordering::Relaxed)
    }
    pub fn join(self) {
        match &self.handle {
            Some(_) => {
                self.handle.unwrap().join().unwrap()
            },
            None => {},
        };
    }
}
#![allow(non_snake_case)]

use std::{sync::{mpsc::Receiver, Arc, atomic::{AtomicBool, Ordering}}, thread};

use log::{info, debug, warn, trace};


pub struct TaskTestReceiver {
    exit: Arc<AtomicBool>,
}

impl TaskTestReceiver {
    pub fn new() -> Self {
        Self {
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn run(&mut self, recvQueue: Receiver<String>, testValues: Vec<f64>) {
        info!("TaskTestReceiver.run | starting...");
        let exit = self.exit.clone();
        let mut testValues = testValues.clone();
        let mut count = 0;
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            // info!("Task({}).run | prepared", name);
            'inner: loop {
                // TODO impl mathematics here...
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
                match testValues.pop() {
                    Some(value) => {
                        match recvQueue.recv() {
                            Ok(sql) => {
                                count += 1;
                                debug!("TaskTestReceiver.run | value: {}\treceived SQL: {:?}", value, sql);
                                // assert!()
                            },
                            Err(err) => {
                                warn!("TaskTestReceiver.run | Error receiving from queue: {:?}", err);
                            },
                        };
                    },
                    None => {
                        warn!("TaskTestReceiver.run | No more values");
                        break;
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
    }
    pub fn exit(&mut self) {
        self.exit.store(true, Ordering::Relaxed);
    }
}
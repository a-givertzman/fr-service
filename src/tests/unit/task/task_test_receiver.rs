#![allow(non_snake_case)]

use std::{sync::{mpsc::Receiver, Arc, atomic::{AtomicBool, Ordering}}, thread};

use log::{info, debug, warn};


pub struct TaskTestReceiver {
    exit: Arc<AtomicBool>,
}

impl TaskTestReceiver {
    pub fn run(&mut self, recvQueue: Receiver<String>, testValues: Vec<f64>) {
        info!("TaskTestReceiver.run | starting...");
        let exit = self.exit.clone();
        let mut testValues = testValues.clone();
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

                            },
                            Err(err) => {
                                warn!("TaskTestReceiver.run | Error receiving from queue: {:?}", err);
                            },
                        };
                    },
                    None => {
                        warn!("TaskTestReceiver.run | No more values");
                    },
                };
                if exit.load(Ordering::Relaxed) {
                    break 'inner;
                }
            };
            info!("TaskTestReceiver.run | stopped");
            // thread::sleep(Duration::from_secs_f32(2.1));
        }).unwrap();
    }
}
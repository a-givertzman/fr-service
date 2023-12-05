#![allow(non_snake_case)]

use std::{sync::{mpsc::{Sender, Receiver}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}};

use log::{warn, info};

use crate::{core_::point::point_type::PointType, services::service::Service};

pub struct MockMultiqueue {
    id: String,
    send: Sender<PointType>,
    recv: Vec<Receiver<PointType>>,
    received: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}
impl MockMultiqueue {
    pub fn new() -> Self {
        let (send, recv) = std::sync::mpsc::channel();
        Self {
            id: "MockMultiqueue".to_owned(),
            send,
            recv: vec![recv],
            received: Arc::new(Mutex::new(vec![])),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
    }
}
impl Service for MockMultiqueue {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn getLink(&self, name: &str) -> Sender<PointType> {
        assert!(name == "queue", "{}.run | link '{:?}' - not found", self.id, name);
        self.send.clone()
    }
    //
    // 
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let recv = self.recv.pop().unwrap();
        let received = self.received.clone();
        let handle = thread::spawn(move || {
            'main: loop {
                match recv.recv() {
                    Ok(point) => {
                        received.lock().unwrap().push(point);
                    },
                    Err(err) => {
                        warn!("{}.run | recv error: {:?}", selfId, err);
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }        
            }
        });
        info!("{}.run | Started", self.id);
        Ok(handle)
    }
    //
    // 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
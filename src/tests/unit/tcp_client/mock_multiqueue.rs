#![allow(non_snake_case)]

use std::sync::{mpsc::{Sender, Receiver}, Arc, atomic::{AtomicBool, Ordering}};

use log::warn;

use crate::{core_::point::point_type::PointType, services::service::Service};

pub struct MockMultiqueue {
    id: String,
    send: Sender<PointType>,
    recv: Receiver<PointType>,
    received: Vec<PointType>,
    exit: Arc<AtomicBool>,
}
impl MockMultiqueue {
    pub fn new() -> Self {
        let (send, recv) = std::sync::mpsc::channel();
        Self {
            id: "MockMultiqueue".to_owned(),
            send,
            recv,
            received: vec![],
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn received(&self) -> &Vec<PointType> {
        &self.received
    }
}
impl Service for MockMultiqueue {
    //
    //
    fn getLink(&self, name: &str) -> Sender<PointType> {
        assert!(name == "queue", "{}.run | link '{:?}' - not found", self.id, name);
        self.send.clone()
    }
    //
    // 
    fn run(&mut self) {
        let exit = self.exit.clone();
        'main: loop {
            match self.recv.recv() {
                Ok(point) => {
                    self.received.push(point);
                },
                Err(err) => {
                    warn!("{}.run | recv error: {:?}", self.id, err);
                },
            }
            if exit.load(Ordering::SeqCst) {
                break 'main;
            }        
        }
    }
    //
    // 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
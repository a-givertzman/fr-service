#![allow(non_snake_case)]

use std::sync::mpsc::{Sender, Receiver};

use crate::{core_::point::point_type::PointType, services::service::Service};

pub struct MockMultiqueue {
    id: String,
    send: Sender<PointType>,
    recv: Receiver<PointType>,
}
impl MockMultiqueue {
    pub fn new() -> Self {
        let (send, recv) = std::sync::mpsc::channel();
        Self {
            id: "MockMultiqueue".to_owned(),
            send,
            recv,
        }
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
        todo!()
    }
    //
    // 
    fn exit(&self) {
        todo!()
    }
}
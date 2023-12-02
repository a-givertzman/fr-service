#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, mpsc::{Sender, Receiver, self}, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread};

use log::{info, warn, error};

use crate::{services::{services::Services, service::Service}, conf::multi_queue_config::MultiQueueConfig, core_::point::point_type::PointType};

use super::subscriptions::Subscriptions;

///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
struct MultiQueue {
    id: String,
    subscriptions: Arc<Mutex<Subscriptions>>,
    inSend: HashMap<String, Sender<PointType>>,
    inRecv: Vec<Receiver<PointType>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: MultiQueueConfig, services: Arc<Mutex<Services>>) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/MultiQueue", parent.into()),
            subscriptions: Arc::new(Mutex::new(Subscriptions::new())),
            inSend: HashMap::from([(conf.recvQueue, send)]),
            inRecv: vec![recv],
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn subscribe() -> Result<(), String> {
        Err("not implemented".to_owned())

    }
    ///
    /// 
    pub fn unsubscribe(receiverId: &str) -> Result<(), String> {
        Err("not implemented".to_owned())
    }
}
///
/// 
impl Service for MultiQueue {
    //
    //
    fn getLink(&self, name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let recv = self.inRecv.pop().unwrap();
        let subscriptions = self.subscriptions.clone();
        let _handle = thread::Builder::new().name(format!("{} - MultiQueue.run", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            loop {
                let subscriptions = subscriptions.lock().unwrap();
                match recv.recv() {
                    Ok(point) => {
                        let pointId = point.name();
                        match subscriptions.get(&pointId) {
                            Some(senders) => {
                                for (receiverId, sender) in senders {
                                    match sender.send(point.clone()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", selfId, pointId, receiverId, err);
                                        },
                                    };
                                }
                            },
                            None => {
                                warn!("{}.run | subscriptions for point '{}' - not found", selfId, pointId);
                            },
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | recv error: {:?}", selfId, err);
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }                
            }
        });
        info!("{}.run | started", self.id);
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, mpsc::{Sender, Receiver, self}, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread::{self, JoinHandle}};

use log::{info, warn, error, debug, trace};

use crate::{services::{services::Services, service::Service}, conf::multi_queue_config::MultiQueueConfig, core_::point::{point_type::PointType, point_tx_id::PointTxId}};

use super::subscriptions::Subscriptions;

///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
pub struct MultiQueue {
    id: String,
    subscriptions: Arc<Mutex<Subscriptions>>,
    subscriptionsChanged: Arc<AtomicBool>,
    rxSend: HashMap<String, Sender<PointType>>,
    rxRecv: Vec<Receiver<PointType>>,
    sendQueues: Vec<String>,
    services: Arc<Mutex<Services>>,
    receiverDictionary: HashMap<usize, String>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MultiQueue {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: MultiQueueConfig, services: Arc<Mutex<Services>>) -> Self {
        let selfId = format!("{}/MultiQueue", parent.into());
        let (send, recv) = mpsc::channel();
        let sendQueues = conf.tx;
        Self {
            id: selfId.clone(),
            subscriptions: Arc::new(Mutex::new(Subscriptions::new(selfId))),
            subscriptionsChanged: Arc::new(AtomicBool::new(false)),
            rxSend: HashMap::from([(conf.rx, send)]),
            rxRecv: vec![recv],
            sendQueues,
            services,
            receiverDictionary: HashMap::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Service for MultiQueue {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn getLink(&mut self, name: &str) -> Sender<PointType> {
        match self.rxSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn subscribe(&mut self, receiverId: &str, points: &Vec<String>) -> Receiver<PointType> {
        let (send, recv) = mpsc::channel();
        let innerReceiverId = PointTxId::fromStr(receiverId);
        self.receiverDictionary.insert(innerReceiverId, receiverId.to_string());
        if points.is_empty() {
            self.subscriptions.lock().unwrap().addBroadcast(innerReceiverId, send.clone());
        } else {
            for pointId in points {
                self.subscriptions.lock().unwrap().addMulticast(innerReceiverId, pointId, send.clone());
            }
        }
        self.subscriptionsChanged.store(true, Ordering::SeqCst);
        recv
    }
    //
    //
    fn unsubscribe(&mut self, receiverId: &str, points: &Vec<String>) -> Result<(), String> {
        let receiverId = PointTxId::fromStr(receiverId);
        for pointId in points {
            match self.subscriptions.lock().unwrap().remove(&receiverId, pointId) {
                Ok(_) => {
                    self.receiverDictionary.remove(&receiverId);
                    self.subscriptionsChanged.store(true, Ordering::SeqCst);
                },
                Err(err) => {
                    return Err(err)
                },
            }
        }
        Ok(())
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let recv = self.rxRecv.pop().unwrap();
        let subscriptionsRef = self.subscriptions.clone();
        let subscriptionsChanged = self.subscriptionsChanged.clone();
        // let mut staticSubscriptions: HashMap<usize, Sender<PointType>> = HashMap::new();
        for receiverId in &self.sendQueues {
            debug!("{}.run | Lock services...", selfId);
            let send = self.services.lock().unwrap().getLink(receiverId);
            debug!("{}.run | Lock services - ok", selfId);
            let innerReceiverId = PointTxId::fromStr(receiverId);
            debug!("{}.run | Lock subscriptions...", selfId);
            self.subscriptions.lock().unwrap().addBroadcast(innerReceiverId, send.clone());
            debug!("{}.run | Lock subscriptions - ok", selfId);

            // staticSubscriptions.insert(PointTxId::fromStr(sendQueue), outSend);
        }
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            debug!("{}.run | Lock subscriptions...", selfId);
            let mut subscriptions = subscriptionsRef.lock().unwrap().clone();
            debug!("{}.run | Lock subscriptions - ok", selfId);
            loop {
                if subscriptionsChanged.load(Ordering::Relaxed) == true {
                    subscriptionsChanged.store(false, Ordering::SeqCst);
                    debug!("{}.run | Lock subscriptions...", selfId);
                    subscriptions = subscriptionsRef.lock().unwrap().clone();
                    debug!("{}.run | Lock subscriptions - ok", selfId);
                }
                match recv.recv() {
                    Ok(point) => {
                        let pointId = point.name();
                        trace!("{}.run | received: {:?}", selfId, point);
                        for (receiverId, sender) in subscriptions.iter(&pointId) {
                            // for (receiverId, sender) in subscriptions.iter(&pointId).chain(&staticSubscriptions) {
                            match receiverId != &point.txId() {
                                true => {
                                    match sender.send(point.clone()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", selfId, pointId, receiverId, err);
                                        },
                                    };
                                },
                                false => {},
                            }
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
        handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

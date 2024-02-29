#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, mpsc::{Sender, Receiver, self}, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread::{self, JoinHandle}};

use log::{debug, error, info, trace};

use crate::{
    core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, point::{point_tx_id::PointTxId, point_type::PointType}}, 
    conf::multi_queue_config::MultiQueueConfig, 
    services::{service::Service, services::Services, multi_queue::subscription_criteria::SubscriptionCriteria},
};

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
        let self_id = format!("{}/MultiQueue", parent.into());
        let (send, recv) = mpsc::channel();
        let sendQueues = conf.tx;
        Self {
            id: self_id.clone(),
            subscriptions: Arc::new(Mutex::new(Subscriptions::new(self_id))),
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
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.rxSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn subscribe(&mut self, receiverId: &str, points: &Vec<SubscriptionCriteria>) -> Receiver<PointType> {
        let (send, recv) = mpsc::channel();
        let innerReceiverId = PointTxId::fromStr(receiverId);
        self.receiverDictionary.insert(innerReceiverId, receiverId.to_string());
        if points.is_empty() {
            self.subscriptions.lock().unwrap().addBroadcast(innerReceiverId, send);
            debug!("{}.subscribe | Broadcast subscription registered, receiver: {} ({})", self.id, receiverId, innerReceiverId);
        } else {
            for subscription_criteria in points {
                self.subscriptions.lock().unwrap().addMulticast(innerReceiverId, &subscription_criteria.destination(), send.clone());
            }
            debug!("{}.subscribe | Multicast subscription registered, receiver: {} ({})", self.id, receiverId, innerReceiverId);
        }
        self.subscriptionsChanged.store(true, Ordering::SeqCst);
        recv
    }
    //
    //
    fn unsubscribe(&mut self, receiverId: &str, points: &Vec<SubscriptionCriteria>) -> Result<(), String> {
        let mut changed = false;
        let innerReceiverId = PointTxId::fromStr(receiverId);
        if points.is_empty() {
            match self.subscriptions.lock().unwrap().removeAll(&innerReceiverId) {
                Ok(_) => {
                    self.receiverDictionary.remove(&innerReceiverId);
                    changed = changed | true;
                    debug!("{}.unsubscribe | Broadcast subscription removed, receiver: {} ({})", self.id, receiverId, innerReceiverId);
                },
                Err(err) => {
                    return Err(err)
                },
            }
        } else {
            for subscription_criteria in points {
                match self.subscriptions.lock().unwrap().remove(&innerReceiverId, &subscription_criteria.destination()) {
                    Ok(_) => {
                        self.receiverDictionary.remove(&innerReceiverId);
                        changed = changed | true;
                        debug!("{}.unsubscribe | Multicat subscription '{}' removed, receiver: {} ({})", self.id, subscription_criteria.destination(), receiverId, innerReceiverId);
                    },
                    Err(err) => {
                        return Err(err)
                    },
                }
            }
        }
        if changed {
            self.subscriptionsChanged.store(true, Ordering::SeqCst);
        }
        Ok(())
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let recv = self.rxRecv.pop().unwrap();
        let subscriptionsRef = self.subscriptions.clone();
        let subscriptionsChanged = self.subscriptionsChanged.clone();
        // let mut staticSubscriptions: HashMap<usize, Sender<PointType>> = HashMap::new();
        for receiverId in &self.sendQueues {
            debug!("{}.run | Lock services...", self_id);
            let send = self.services.lock().unwrap().get_link(receiverId);
            debug!("{}.run | Lock services - ok", self_id);
            let innerReceiverId = PointTxId::fromStr(receiverId);
            debug!("{}.run | Lock subscriptions...", self_id);
            self.subscriptions.lock().unwrap().addBroadcast(innerReceiverId, send.clone());
            debug!("{}.run | Lock subscriptions - ok", self_id);

            // staticSubscriptions.insert(PointTxId::fromStr(sendQueue), txSend);
        }
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            debug!("{}.run | Lock subscriptions...", self_id);
            let mut subscriptions = subscriptionsRef.lock().unwrap().clone();
            debug!("{}.run | Lock subscriptions - ok", self_id);
            loop {
                if subscriptionsChanged.load(Ordering::Relaxed) == true {
                    subscriptionsChanged.store(false, Ordering::SeqCst);
                    debug!("{}.run | Subscriptions changes detected", self_id);
                    debug!("{}.run | Lock subscriptions...", self_id);
                    subscriptions = subscriptionsRef.lock().unwrap().clone();
                    debug!("{}.run | Lock subscriptions - ok", self_id);
                }
                match recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        let pointId = SubscriptionCriteria::new(&point.name(), point.cot()).destination();
                        trace!("{}.run | received: {:?}", self_id, point);
                        for (receiverId, sender) in subscriptions.iter(&pointId) {
                            // for (receiverId, sender) in subscriptions.iter(&pointId).chain(&staticSubscriptions) {
                            match receiverId != point.tx_id() {
                                true => {
                                    match sender.send(point.clone()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", self_id, pointId, receiverId, err);
                                        },
                                    };
                                },
                                false => {},
                            }
                        }
                    },
                    Err(err) => {
                        trace!("{}.run | recv timeout: {:?}", self_id, err);
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            info!("{}.run | Exit", self_id);
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

#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, mpsc::{Sender, Receiver, self}, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread::{self, JoinHandle}};

use log::{info, warn, error, debug, trace};

use crate::{
    core_::point::{point_tx_id::PointTxId, point_type::PointType}, 
    services::{
        multi_queue::{subscription_criteria::SubscriptionCriteria, subscriptions::Subscriptions}, 
        services::Services,
        service::service::Service, 
    },
};


///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
pub struct MockMultiQueueMatch {
    id: String,
    subscriptions: Arc<Mutex<Subscriptions>>,
    rxSend: HashMap<String, Sender<PointType>>,
    rxRecv: Vec<Receiver<PointType>>,
    sendQueues: Vec<String>,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockMultiQueueMatch {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, txQueues: Vec<String>, rxQueue: impl Into<String>, services: Arc<Mutex<Services>>) -> Self {
        let self_id = format!("{}/MockMultiQueueMatch", parent.into());
        let (send, recv) = mpsc::channel();
        Self {
            id: self_id.clone(),
            subscriptions: Arc::new(Mutex::new(Subscriptions::new(self_id))),
            rxSend: HashMap::from([(rxQueue.into(), send)]),
            rxRecv: vec![recv],
            sendQueues: txQueues,
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Service for MockMultiQueueMatch {
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
        let receiverId = PointTxId::fromStr(receiverId);
        if points.is_empty() {
            self.subscriptions.lock().unwrap().add_broadcast(receiverId, send.clone());
        } else {
            for subscription_criteria in points {
                self.subscriptions.lock().unwrap().add_multicast(receiverId, &subscription_criteria.destination(), send.clone());
            }
        }
        recv
    }
    //
    //
    fn unsubscribe(&mut self, receiverId: &str, points: &Vec<SubscriptionCriteria>) -> Result<(), String> {
        let receiverId = PointTxId::fromStr(receiverId);
        for subscription_criteria in points {
            match self.subscriptions.lock().unwrap().remove(&receiverId, &subscription_criteria.destination()) {
                Ok(_) => {},
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
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let recv = self.rxRecv.pop().unwrap();
        let subscriptions = self.subscriptions.clone();
        let mut staticSubscriptions: HashMap<usize, Sender<PointType>> = HashMap::new();
        for sendQueue in &self.sendQueues {
            debug!("{}.run | Lock services...", self_id);
            let txSend = self.services.lock().unwrap().get_link(sendQueue);
            debug!("{}.run | Lock services - ok", self_id);
            staticSubscriptions.insert(PointTxId::fromStr(sendQueue), txSend);
        }
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            loop {
                let subscriptions = subscriptions.lock().unwrap();
                match recv.recv() {
                    Ok(point) => {
                        let pointId = point.name();
                        trace!("{}.run | received: {:?}", self_id, point);
                        for (receiverId, sender) in subscriptions.iter(&pointId).chain(&staticSubscriptions) {
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
                        warn!("{}.run | recv error: {:?}", self_id, err);
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



















    // //
    // //
    // fn serveRx(&mut self, recv: Receiver<PointType>) -> Result<JoinHandle<()>, std::io::Error> {
    //     info!("{}.run | starting...", self.id);
    //     let self_id = self.id.clone();
    //     let exit = self.exit.clone();
    //     let subscriptions = self.subscriptions.clone();
    //     let mut staticSubscriptions: HashMap<String, Sender<PointType>> = HashMap::new();
    //     for sendQueue in &self.sendQueues {
    //         debug!("{}.run | Lock services...", self_id);
    //         let txSend = self.services.lock().unwrap().get_link(sendQueue);
    //         debug!("{}.run | Lock services - ok", self_id);
    //         staticSubscriptions.insert(sendQueue.to_string(), txSend);
    //     }
    //     let _handle = thread::Builder::new().name(format!("{} - MockMultiQueueMatch.run", self_id.clone())).spawn(move || {
    //         info!("{}.run | Preparing thread - ok", self_id);
    //         loop {
    //             let subscriptions = subscriptions.lock().unwrap();
    //             match recv.recv() {
    //                 Ok(point) => {
    //                     let pointId = point.name();
    //                     trace!("{}.run | received: {:?}", self_id, point);
    //                     for (receiverId, sender) in subscriptions.iter(&pointId).chain(&staticSubscriptions) {
    //                         match sender.send(point.clone()) {
    //                             Ok(_) => {},
    //                             Err(err) => {
    //                                 error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", self_id, pointId, receiverId, err);
    //                             },
    //                         };
    //                     }
    //                 },
    //                 Err(err) => {
    //                     warn!("{}.run | recv error: {:?}", self_id, err);
    //                 },
    //             }
    //             if exit.load(Ordering::SeqCst) {
    //                 break;
    //             }                
    //         }
    //     });
    //     info!("{}.run | started", self.id);
    //     _handle
    // }
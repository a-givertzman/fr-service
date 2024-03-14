use std::{sync::{Arc, Mutex, mpsc::{Sender, Receiver, self}, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread::{self, JoinHandle}};
use log::{debug, error, info, trace, warn};
use crate::{
    conf::multi_queue_config::MultiQueueConfig, core_::{constants::constants::RECV_TIMEOUT, object::object::Object, point::{point_tx_id::PointTxId, point_type::PointType}}, services::{multi_queue::subscription_criteria::SubscriptionCriteria, service::service::Service, services::Services}
};
use concat_string::concat_string;
use super::subscriptions::Subscriptions;

///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
pub struct MultiQueue {
    id: String,
    subscriptions: Arc<Mutex<Subscriptions>>,
    subscriptions_changed: Arc<AtomicBool>,
    rx_send: HashMap<String, Sender<PointType>>,
    rx_recv: Vec<Receiver<PointType>>,
    send_queues: Vec<String>,
    services: Arc<Mutex<Services>>,
    receiver_dictionary: HashMap<usize, String>,
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
        let send_queues = conf.tx;
        Self {
            id: self_id.clone(),
            subscriptions: Arc::new(Mutex::new(Subscriptions::new(self_id))),
            subscriptions_changed: Arc::new(AtomicBool::new(false)),
            rx_send: HashMap::from([(conf.rx, send)]),
            rx_recv: vec![recv],
            send_queues,
            services,
            receiver_dictionary: HashMap::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Object for MultiQueue {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for MultiQueue {
    //
    //
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.rx_send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn subscribe(&mut self, receiver_id: &str, points: &[SubscriptionCriteria]) -> (Sender<PointType>, Receiver<PointType>) {
        let (send, recv) = mpsc::channel();
        let inner_receiver_id = PointTxId::fromStr(receiver_id);
        self.receiver_dictionary.insert(inner_receiver_id, receiver_id.to_string());
        if points.is_empty() {
            self.subscriptions.lock().unwrap().add_broadcast(inner_receiver_id, send.clone());
            debug!("{}.subscribe | Broadcast subscription registered, receiver: \n\t{} ({})", self.id, receiver_id, inner_receiver_id);
        } else {
            for subscription_criteria in points {
                self.subscriptions.lock().unwrap().add_multicast(inner_receiver_id, &subscription_criteria.destination(), send.clone());
            }
            debug!("{}.subscribe | Multicast subscription registered, receiver: \n\t{} ({}) \n\tpoints: {:#?}", self.id, receiver_id, inner_receiver_id, points);
        }
        self.subscriptions_changed.store(true, Ordering::SeqCst);
        (send, recv)
    }
    //
    //
    fn extend_subscription(&mut self, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        let inner_receiver_id = PointTxId::fromStr(receiver_id);
        // self.receiver_dictionary.insert(inner_receiver_id, receiver_id.to_string());
        if points.is_empty() {
            let message = format!("{}.extend_subscription | Broadcast subscription can't be extended, receiver: {} ({})", self.id, receiver_id, inner_receiver_id);
            warn!("{}", message);
            Err(message)
        } else {
            let mut message = String::new();
            for subscription_criteria in points {
                if let Err(err) = self.subscriptions.lock().unwrap().extend_multicast(inner_receiver_id, &subscription_criteria.destination()) {
                    message = concat_string!(message, err, "\n");
                };
            }
            if message.is_empty() {
                debug!("{}.extend_subscription | Multicast subscription extended, receiver: {} ({})", self.id, receiver_id, inner_receiver_id);
                self.subscriptions_changed.store(true, Ordering::SeqCst);
                Ok(())
            } else {
                debug!("{}.extend_subscription | Multicast subscription extended, receiver: {} ({}) \n\t with errors: {:?}", self.id, receiver_id, inner_receiver_id, message);
                self.subscriptions_changed.store(true, Ordering::SeqCst);
                Err(message)
            }
        }
    }
    //
    //
    fn unsubscribe(&mut self, receiver_id: &str, points: &[SubscriptionCriteria]) -> Result<(), String> {
        let mut changed = false;
        let inner_receiver_id = PointTxId::fromStr(receiver_id);
        if points.is_empty() {
            match self.subscriptions.lock().unwrap().remove_all(&inner_receiver_id) {
                Ok(_) => {
                    self.receiver_dictionary.remove(&inner_receiver_id);
                    changed |= true;
                    debug!("{}.unsubscribe | Broadcast subscription removed, receiver: {} ({})", self.id, receiver_id, inner_receiver_id);
                },
                Err(err) => {
                    return Err(err)
                },
            }
        } else {
            for subscription_criteria in points {
                match self.subscriptions.lock().unwrap().remove(&inner_receiver_id, &subscription_criteria.destination()) {
                    Ok(_) => {
                        self.receiver_dictionary.remove(&inner_receiver_id);
                        changed |= true;
                        debug!("{}.unsubscribe | Multicat subscription '{}' removed, receiver: {} ({})", self.id, subscription_criteria.destination(), receiver_id, inner_receiver_id);
                    },
                    Err(err) => {
                        return Err(err)
                    },
                }
            }
        }
        if changed {
            self.subscriptions_changed.store(true, Ordering::SeqCst);
        }
        Ok(())
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let recv = self.rx_recv.pop().unwrap();
        let subscriptions_ref = self.subscriptions.clone();
        let subscriptions_changed = self.subscriptions_changed.clone();
        // let mut staticSubscriptions: HashMap<usize, Sender<PointType>> = HashMap::new();
        for receiver_id in &self.send_queues {
            debug!("{}.run | Lock services...", self_id);
            let send = self.services.lock().unwrap().get_link(receiver_id);
            debug!("{}.run | Lock services - ok", self_id);
            let inner_receiver_id = PointTxId::fromStr(receiver_id);
            debug!("{}.run | Lock subscriptions...", self_id);
            self.subscriptions.lock().unwrap().add_broadcast(inner_receiver_id, send.clone());
            debug!("{}.subscribe | Broadcast subscription registered, receiver: \n\t{} ({})", self.id, receiver_id, inner_receiver_id);
            debug!("{}.run | Lock subscriptions - ok", self_id);
        }
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            debug!("{}.run | Lock subscriptions...", self_id);
            let mut subscriptions = subscriptions_ref.lock().unwrap().clone();
            debug!("{}.run | Lock subscriptions - ok", self_id);
            loop {
                if subscriptions_changed.load(Ordering::Relaxed) {
                    subscriptions_changed.store(false, Ordering::SeqCst);
                    debug!("{}.run | Subscriptions changes detected", self_id);
                    debug!("{}.run | Lock subscriptions...", self_id);
                    subscriptions = subscriptions_ref.lock().unwrap().clone();
                    debug!("{}.run | Lock subscriptions - ok", self_id);
                }
                match recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        let point_id = SubscriptionCriteria::new(&point.name(), point.cot()).destination();
                        debug!("{}.run | received: {:?}", self_id, point);
                        for (receiver_id, sender) in subscriptions.iter(&point_id) {
                            // for (receiverId, sender) in subscriptions.iter(&pointId).chain(&staticSubscriptions) {
                            match receiver_id != point.tx_id() {
                                true => {
                                    match sender.send(point.clone()) {
                                        Ok(_) => {},
                                        Err(err) => {
                                            error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", self_id, point_id, receiver_id, err);
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

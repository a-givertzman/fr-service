use std::{collections::HashMap, fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread};
use log::{debug, error, info, trace, warn};
use crate::{
    conf::multi_queue_config::MultiQueueConfig, 
    core_::{constants::constants::RECV_TIMEOUT, object::object::Object, point::{point_tx_id::PointTxId, point_type::PointType}}, 
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services
    },
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
impl Debug for MultiQueue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MultiQueue")
            .field("id", &self.id)
            .finish()
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
            self.subscriptions.slock().add_broadcast(inner_receiver_id, send.clone());
            debug!("{}.subscribe | Broadcast subscription registered, receiver: \n\t{} ({})", self.id, receiver_id, inner_receiver_id);
        } else {
            for subscription_criteria in points {
                self.subscriptions.slock().add_multicast(inner_receiver_id, &subscription_criteria.destination(), send.clone());
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
                if let Err(err) = self.subscriptions.slock().extend_multicast(inner_receiver_id, &subscription_criteria.destination()) {
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
            match self.subscriptions.slock().remove_all(&inner_receiver_id) {
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
                match self.subscriptions.slock().remove(&inner_receiver_id, &subscription_criteria.destination()) {
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
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let recv = self.rx_recv.pop().unwrap();
        let subscriptions_ref = self.subscriptions.clone();
        let subscriptions_changed = self.subscriptions_changed.clone();
        for receiver_id in &self.send_queues {
            let send = self.services.slock().get_link(receiver_id).unwrap_or_else(|err| {
                panic!("{}.run | services.get_link error: {:#?}", self_id, err);
            });
            let inner_receiver_id = PointTxId::fromStr(receiver_id);
            self.subscriptions.slock().add_broadcast(inner_receiver_id, send.clone());
            debug!("{}.subscribe | Broadcast subscription registered, receiver: \n\t{} ({})", self.id, receiver_id, inner_receiver_id);
        }
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            // debug!("{}.run | Lock subscriptions...", self_id);
            let mut subscriptions = subscriptions_ref.slock().clone();
            // debug!("{}.run | Lock subscriptions - ok", self_id);
            loop {
                if subscriptions_changed.load(Ordering::Relaxed) {
                    subscriptions_changed.store(false, Ordering::SeqCst);
                    debug!("{}.run | Subscriptions changes detected", self_id);
                    // debug!("{}.run | Lock subscriptions...", self_id);
                    subscriptions = subscriptions_ref.slock().clone();
                    // debug!("{}.run | Lock subscriptions - ok", self_id);
                }
                match recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        let point_id = SubscriptionCriteria::new(&point.name(), point.cot()).destination();
                        trace!("{}.run | received: \n\t{:?}", self_id, point);
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
        match handle {
            Ok(handle) => {
                info!("{}.run | Started", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            },
            Err(err) => {
                let message = format!("{}.run | Start faled: {:#?}", self.id, err);
                warn!("{}", message);
                Err(message)
            },
        }
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

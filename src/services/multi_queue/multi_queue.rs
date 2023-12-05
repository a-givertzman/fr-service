#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, mpsc::{Sender, Receiver, self}, atomic::{Ordering, AtomicBool}}, collections::HashMap, thread::{self, JoinHandle}, error};

use log::{info, warn, error, debug, trace};

use crate::{services::{services::Services, service::Service}, conf::multi_queue_config::MultiQueueConfig, core_::point::point_type::PointType};

use super::subscriptions::Subscriptions;

///
/// - Receives points into the MPSC queue in the blocking mode
/// - If new point received, immediately sends it to the all subscribed consumers
/// - Keeps all consumers subscriptions in the single map:
pub struct MultiQueue {
    id: String,
    subscriptions: Arc<Mutex<HashMap<String, Subscriptions>>>,
    rxName: String,
    // rxSend: HashMap<String, Sender<PointType>>,
    // inRecv: Vec<Receiver<PointType>>,
    sendQueues: Vec<String>,
    services: Arc<Mutex<Services>>,
    serveRx: Vec<JoinHandle<()>>,
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
        let sendQueues = conf.tx;
        Self {
            id: selfId.clone(),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            rxName: conf.rx,
            // rxSend: HashMap::new(),     //HashMap::from([(conf.rx, send)]),
            // inRecv: vec![],
            sendQueues,
            services,
            serveRx: vec![],
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    //
    //
    fn serveRx(&mut self, recv: Receiver<PointType>) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let subscriptions = self.subscriptions.clone();
        let mut staticSubscriptions: HashMap<String, Sender<PointType>> = HashMap::new();
        for sendQueue in &self.sendQueues {
            debug!("{}.run | Getting services...", selfId);
            let outSend = self.services.lock().unwrap().getLink(sendQueue);
            debug!("{}.run | Getting services - ok", selfId);
            staticSubscriptions.insert(sendQueue.to_string(), outSend);
        }
        let _handle = thread::Builder::new().name(format!("{} - MultiQueue.run", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            loop {
                let subscriptions = subscriptions.lock().unwrap();
                match recv.recv() {
                    Ok(point) => {
                        let pointId = point.name();
                        trace!("{}.run | received: {:?}", selfId, point);
                        for (receiverId, sender) in subscriptions.iter(&pointId).chain(&staticSubscriptions) {
                            match sender.send(point.clone()) {
                                Ok(_) => {},
                                Err(err) => {
                                    error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", selfId, pointId, receiverId, err);
                                },
                            };
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
        _handle
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
        let (send, recv) = mpsc::channel();
        if name == self.rxName {
            let handle = self.serveRx(recv).unwrap();
            self.serveRx.push(handle);
            return send
        }
        panic!("{}.run | link '{:?}' - not found", self.id, name);
    }
    ///
    /// 
    fn subscribe(&mut self, receiverId: &str, points: &Vec<String>) -> Receiver<PointType> {
        let (send, recv) = mpsc::channel();
        if points.is_empty() {
            self.subscriptions.lock().unwrap().addBroadcast(receiverId, send.clone());
        } else {
            for pointId in points {
                self.subscriptions.lock().unwrap().addMulticast(receiverId, pointId, send.clone());
            }
        }
        recv
    }
    ///
    /// 
    fn unsubscribe(&mut self, receiverId: &str, points: &Vec<String>) -> Result<(), String> {
        for pointId in points {
            match self.subscriptions.lock().unwrap().remove(receiverId, pointId) {
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
        // let selfId = self.id.clone();
        // let exit = self.exit.clone();
        // let recv = self.inRecv.pop().unwrap();
        // let subscriptions = self.subscriptions.clone();
        // let mut staticSubscriptions: HashMap<String, Sender<PointType>> = HashMap::new();
        // for sendQueue in &self.sendQueues {
        //     debug!("{}.run | Getting services...", selfId);
        //     let outSend = self.services.lock().unwrap().getLink(sendQueue);
        //     debug!("{}.run | Getting services - ok", selfId);
        //     staticSubscriptions.insert(sendQueue.to_string(), outSend);
        // }
        // let _handle = thread::Builder::new().name(format!("{} - MultiQueue.run", selfId.clone())).spawn(move || {
        //     info!("{}.run | Preparing thread - ok", selfId);
        //     loop {
        //         let subscriptions = subscriptions.lock().unwrap();
        //         match recv.recv() {
        //             Ok(point) => {
        //                 let pointId = point.name();
        //                 trace!("{}.run | received: {:?}", selfId, point);
        //                 for (receiverId, sender) in subscriptions.iter(&pointId).chain(&staticSubscriptions) {
        //                     match sender.send(point.clone()) {
        //                         Ok(_) => {},
        //                         Err(err) => {
        //                             error!("{}.run | subscriptions '{}', receiver '{}' - send error: {:?}", selfId, pointId, receiverId, err);
        //                         },
        //                     };
        //                 }
        //             },
        //             Err(err) => {
        //                 warn!("{}.run | recv error: {:?}", selfId, err);
        //             },
        //         }
        //         if exit.load(Ordering::SeqCst) {
        //             break;
        //         }                
        //     }
        // });
        info!("{}.run | started", self.id);
        // _handle
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{}.run | in service", self.id)))
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

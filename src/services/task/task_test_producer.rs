#![allow(non_snake_case)]

use std::{sync::{mpsc::Sender, Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}, collections::HashMap};
use rand::Rng;

use log::{debug, warn, info};

use crate::{core_::point::point_type::{PointType, ToPoint}, services::{service::Service, services::Services}};


///
/// 
pub struct TaskTestProducer {
    id: String,
    iterations: usize,
    link: String, 
    inSend: HashMap<String, Sender<PointType>>,
    services: Arc<Mutex<Services>>,
    sent: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TaskTestProducer {
    pub fn new(iterations: usize, link: &str, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: String::from("TaskTestProducer"),
            iterations,
            link: link.to_string(),
            inSend: HashMap::new(),
            services,
            sent: Arc::new(Mutex::new(vec![])),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn sent(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.sent.clone()
    }
}
///
/// 
impl Service for TaskTestProducer {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn getLink(&mut self, name: &str) -> Sender<PointType> {
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }        
    }
    //
    // 
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        let selfId = self.id.clone();
        let iterations = self.iterations;
        let outSend = self.services.lock().unwrap().getLink(&self.link);
        let sent = self.sent.clone();
        thread::Builder::new().name("name".to_owned()).spawn(move || {
            debug!("{}.run | calculating step...", selfId);
            let mut random = rand::thread_rng();
            let max = 1.0;
            // let mut sent = 0;
            for _ in 0..iterations {
                let value = random.gen_range(0.0..max);
                let point = value.toPoint("/path/Point.Name");
                match outSend.send(point.clone()) {
                    Ok(_) => {
                        sent.lock().unwrap().push(point);
                    },
                    Err(err) => {
                        warn!("{}.run | Error write to queue: {:?}", selfId, err);
                    },
                }
                // thread::sleep(Duration::from_micros(10));
            }
            info!("{}.run | Sent points: {}", selfId, sent.lock().unwrap().len());
            // thread::sleep(Duration::from_secs_f32(0.1));
            // debug!("TaskTestProducer({}).run | calculating step - done ({:?})", name, cycle.elapsed());
        })
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::Relaxed);
    }
}

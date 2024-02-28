#![allow(non_snake_case)]

use std::{sync::{mpsc::Sender, Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}, time::Duration};

use log::{debug, warn, info, trace};
use testing::entities::test_value::Value;

use crate::{core_::point::{point_tx_id::PointTxId, point_type::{PointType, ToPoint}}, services::{service::Service, services::Services}};


///
/// 
pub struct TaskTestProducer {
    id: String,
    link: String, 
    cycle: Duration,
    // rxSend: HashMap<String, Sender<PointType>>,
    services: Arc<Mutex<Services>>,
    test_data: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TaskTestProducer {
    pub fn new(parent: &str, link: &str, cycle: Duration, services: Arc<Mutex<Services>>, test_data: Vec<Value>) -> Self {
        Self {
            id: format!("{}/TaskTestProducer", parent),
            link: link.to_string(),
            cycle,
            // rxSend: HashMap::new(),
            services,
            test_data,
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
    fn get_link(&mut self, _name: &str) -> Sender<PointType> {
        panic!("{}.get_link | Does not support get_link", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }        
    }
    //
    // 
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        let self_id = self.id.clone();
        let txId = PointTxId::fromStr(&self_id);
        let cycle = self.cycle.clone();
        let delayed = !cycle.is_zero();
        let txSend = self.services.lock().unwrap().get_link(&self.link);
        let sent = self.sent.clone();
        let test_data = self.test_data.clone();
        thread::Builder::new().name(self_id.clone()).spawn(move || {
            debug!("{}.run | calculating step...", self_id);
            for value in test_data {
                let point = value.to_point(txId, "/path/Point.Name");
                match txSend.send(point.clone()) {
                    Ok(_) => {
                        sent.lock().unwrap().push(point.clone());
                        trace!("{}.run | sent points: {:?}", self_id, sent.lock().unwrap().len());
                    },
                    Err(err) => {
                        warn!("{}.run | Error write to queue: {:?}", self_id, err);
                    },
                }
                if delayed {
                    thread::sleep(cycle);
                }
            }
            info!("{}.run | All sent: {}", self_id, sent.lock().unwrap().len());
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

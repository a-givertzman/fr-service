#![allow(non_snake_case)]

use std::{sync::{mpsc::Sender, Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread::{self, JoinHandle}};

use log::{debug, warn, info};

use crate::{core_::{point::{point_type::PointType, point_tx_id::PointTxId}, testing::test_stuff::test_value::Value}, services::{service::Service, services::Services}};


///
/// 
pub struct TaskTestProducer {
    id: String,
    link: String, 
    // rxSend: HashMap<String, Sender<PointType>>,
    services: Arc<Mutex<Services>>,
    testData: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TaskTestProducer {
    pub fn new(parent: &str, link: &str, services: Arc<Mutex<Services>>, testData: Vec<Value>) -> Self {
        Self {
            id: format!("{}/TaskTestProducer", parent),
            link: link.to_string(),
            // rxSend: HashMap::new(),
            services,
            testData,
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
        panic!("{}.getLink | Does not support getLink", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }        
    }
    //
    // 
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        let selfId = self.id.clone();
        let txId = PointTxId::fromStr(&selfId);
        let txSend = self.services.lock().unwrap().getLink(&self.link);
        let sent = self.sent.clone();
        let testData = self.testData.clone();
        thread::Builder::new().name(selfId.clone()).spawn(move || {
            debug!("{}.run | calculating step...", selfId);
            for value in testData {
                let point = value.toPoint(txId, "/path/Point.Name");
                match txSend.send(point.clone()) {
                    Ok(_) => {
                        sent.lock().unwrap().push(point);
                    },
                    Err(err) => {
                        warn!("{}.run | Error write to queue: {:?}", selfId, err);
                    },
                }
                // thread::sleep(Duration::from_micros(10));
            }
            info!("{}.run | All sent: {}", selfId, sent.lock().unwrap().len());
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

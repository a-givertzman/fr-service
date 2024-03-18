use std::{sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex}, thread, time::Duration};
use log::{debug, warn, info, trace};
use testing::entities::test_value::Value;
use crate::{core_::{object::object::Object, point::{point_tx_id::PointTxId, point_type::{PointType, ToPoint}}}, services::{service::{service::Service, service_handles::ServiceHandles}, services::Services}};

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
impl Object for TaskTestProducer {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for TaskTestProducer {
    //
    // 
    fn run(&mut self) -> Result<ServiceHandles, String> {
        let self_id = self.id.clone();
        let tx_id = PointTxId::fromStr(&self_id);
        let cycle = self.cycle;
        let delayed = !cycle.is_zero();
        let tx_send = self.services.lock().unwrap().get_link(&self.link);
        let sent = self.sent.clone();
        let test_data = self.test_data.clone();
        match thread::Builder::new().name(self_id.clone()).spawn(move || {
            debug!("{}.run | calculating step...", self_id);
            for value in test_data {
                let point = value.to_point(tx_id, "/path/Point.Name");
                match tx_send.send(point.clone()) {
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
        }) {
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
        self.exit.store(true, Ordering::Relaxed);
    }
}

use std::{collections::HashMap, fmt::Debug, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Mutex, RwLock}, thread, time::Duration};
use log::{debug, warn, info, trace};
use testing::entities::test_value::Value;
use crate::{conf::point_config::{name::Name, point_config::PointConfig}, core_::{object::object::Object, point::{point_tx_id::PointTxId, point_type::{PointType, ToPoint}}}, services::{queue_name::QueueName, safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services}};

///
/// 
pub struct TaskTestProducer {
    id: String,
    name: Name,
    send_to: QueueName, 
    cycle: Duration,
    // rxSend: HashMap<String, Sender<PointType>>,
    services: Arc<RwLock<Services>>,
    test_data: Vec<(String, Value)>,
    sent: Arc<Mutex<Vec<PointType>>>,
    exit: Arc<AtomicBool>,
}
//
// 
impl TaskTestProducer {
    pub fn new(parent: &str, send_to: &str, cycle: Duration, services: Arc<RwLock<Services>>, test_data: &[(String, Value)]) -> Self {
        let name = Name::new(parent, format!("TaskTestProducer{}", COUNT.fetch_add(1, Ordering::Relaxed)));
        Self {
            id: name.join(),
            name,
            send_to: QueueName::new(send_to),
            cycle,
            // rxSend: HashMap::new(),
            services,
            test_data: test_data.to_vec(),
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
//
// 
impl Object for TaskTestProducer {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> crate::conf::point_config::name::Name {
        self.name.clone()
    }
}
//
// 
impl Debug for TaskTestProducer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TaskTestProducer")
            .field("id", &self.id)
            .finish()
    }
}
//
// 
impl Service for TaskTestProducer {
    //
    // 
    fn run(&mut self) -> Result<ServiceHandles, String> {
        let self_id = self.id.clone();
        let tx_id = PointTxId::from_str(&self_id);
        let cycle = self.cycle;
        let delayed = !cycle.is_zero();
        let tx_send = self.services.rlock(&self_id).get_link(&self.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.id, err);
        });
        let sent = self.sent.clone();
        let test_data = self.test_data.clone();
        let handle = thread::Builder::new().name(self_id.clone()).spawn(move || {
            debug!("{}.run | calculating step...", self_id);
            for (name, value) in test_data {
                let point = value.to_point(tx_id, &name);
                match tx_send.send(point.clone()) {
                    Ok(_) => {
                        sent.lock().unwrap().push(point.clone());
                        trace!("{}.run | sent points: {:?}", self_id, sent.lock().unwrap().len());
                    }
                    Err(err) => {
                        warn!("{}.run | Error write to queue: {:?}", self_id, err);
                    }
                }
                if delayed {
                    thread::sleep(cycle);
                }
            }
            info!("{}.run | All sent: {}", self_id, sent.lock().unwrap().len());
            // thread::sleep(Duration::from_secs_f32(0.1));
            // debug!("TaskTestProducer({}).run | calculating step - done ({:?})", name, cycle.elapsed());
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Started", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.id, err);
                warn!("{}", message);
                Err(message)
            }
        }
    }
    //
    // Returns Vec<PointConfig> of points found in the test_data
    fn points(&self) -> Vec<crate::conf::point_config::point_config::PointConfig> {
        self.test_data
            .iter()
            .map(|(name, value)| {
                (name.clone(), value.clone())
            })
            .collect::<HashMap<String, Value>>()
            .iter()
            .map(|(name, value)| {
                let type_ = match value {
                    Value::Bool(_) => "Bool",
                    Value::Int(_) => "Int",
                    Value::Real(_) => "Real",
                    Value::Double(_) => "Double",
                    Value::String(_) => "String",
                };
                PointConfig::from_yaml(
                    &Name::new("", ""),
                    &serde_yaml::from_str(&format!(r#"{}:
                        type: {}"#, name, type_)).unwrap()
                )
            })
            .collect()
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::Relaxed);
    }
}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(0);

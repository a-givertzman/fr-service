use std::{fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread, time::Duration};
use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use log::{debug, warn, info};
use rand::Rng;
use serde_json::json;
use testing::entities::test_value::Value;
use crate::{
    conf::point_config::{name::Name, point_config::PointConfig, point_config_history::PointConfigHistory, point_config_type::PointConfigType}, 
    core_::{cot::cot::Cot, object::object::Object, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status, types::bool::Bool}, 
    services::{safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services, task::service_cycle::ServiceCycle},
};

use super::producer_service_config::ProducerServiceConfig;

///
/// 
pub struct ProducerService {
    id: String,
    name: Name,
    conf: ProducerServiceConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl ProducerService {
    pub fn new(conf: ProducerServiceConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: format!("{}(ProducerService)", conf.name),
            name: conf.name.clone(),
            conf,
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn build_gen_points(parent_id: &str, tx_id: usize, points: Vec<PointConfig>) -> IndexMap<String, Box<impl ParsePoint<Value>>> {
        let mut gen_points = IndexMap::new();
        for point_conf in points {
            match point_conf._type {
                crate::conf::point_config::point_config_type::PointConfigType::Bool => {
                    gen_points.insert(point_conf.name.clone(), Box::new(PointGen::new(parent_id, tx_id, point_conf.name.clone(), &point_conf)));
                },
                crate::conf::point_config::point_config_type::PointConfigType::Int => {
                    gen_points.insert(point_conf.name.clone(), Box::new(PointGen::new(parent_id, tx_id, point_conf.name.clone(), &point_conf)));
                },
                crate::conf::point_config::point_config_type::PointConfigType::Real => {
                    gen_points.insert(point_conf.name.clone(), Box::new(PointGen::new(parent_id, tx_id, point_conf.name.clone(), &point_conf)));
                },
                crate::conf::point_config::point_config_type::PointConfigType::Double => {
                    gen_points.insert(point_conf.name.clone(), Box::new(PointGen::new(parent_id, tx_id, point_conf.name.clone(), &point_conf)));
                },
                crate::conf::point_config::point_config_type::PointConfigType::String => {
                    gen_points.insert(point_conf.name.clone(), Box::new(PointGen::new(parent_id, tx_id, point_conf.name.clone(), &point_conf)));
                },
                crate::conf::point_config::point_config_type::PointConfigType::Json => {
                    gen_points.insert(point_conf.name.clone(), Box::new(PointGen::new(parent_id, tx_id, point_conf.name.clone(), &point_conf)));
                },
            }
        }
        gen_points
    }
}
///
/// 
impl Object for ProducerService {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> crate::conf::point_config::name::Name {
        self.name.clone()
    }
}
///
/// 
impl Debug for ProducerService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProducerService")
            .field("id", &self.id)
            .finish()
    }
}
///
/// 
impl Service for ProducerService {
    //
    // 
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let tx_id = PointTxId::fromStr(&self_id);
        let exit = self.exit.clone();
        let interval = self.conf.cycle.clone().unwrap_or(Duration::ZERO);
        let delayed = !interval.is_zero();
        let mut cycle = ServiceCycle::new(&self.id, interval);
        let send = self.services.slock().get_link(&self.conf.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.id, err);
        });
        let mut gen_points = Self::build_gen_points(&self.id, tx_id, self.conf.points());
        match thread::Builder::new().name(self_id.clone()).spawn(move || {
            // let mut test_data = Self::build_test_data(&self_id);
            'main: loop {
                debug!("{}.run | Step...", self_id);
                for (_, gen_point) in &mut gen_points {
                    cycle.start();
                    match gen_point.next(&Value::Bool(false), Utc::now()) {
                        Some(point) => {
                            match send.send(point.clone()) {
                                Ok(_) => {
                                    debug!("{}.run | sent point: {:?}", self_id, point);
                                },
                                Err(err) => {
                                    warn!("{}.run | Send error: {:?}", self_id, err);
                                },
                            }
                        },
                        None => {},
                    };
                    if delayed {
                        cycle.wait();
                    }
                    if exit.load(Ordering::SeqCst) {
                        break 'main;
                    }
                }
            }
            info!("{}.run | Stopped", self_id);
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
    fn points(&self) -> Vec<PointConfig> {
        self.conf.points()
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::Relaxed);
    }
}

///
///
#[derive(Debug, Clone)]
pub struct PointGen {
    id: String,
    pub tx_id: usize,
    _type: PointConfigType,
    pub name: String,
    pub value: Value,
    pub status: Status,
    pub history: PointConfigHistory,
    pub alarm: Option<u8>,
    pub timestamp: DateTime<Utc>,
    is_changed: bool,
}
impl PointGen {
    ///
    /// 
    pub fn new(
        parent_id: &str,
        tx_id: usize,
        name: String,
        config: &PointConfig,
        // filter: Filter<T>,
    ) -> PointGen {
        PointGen {
            id: format!("{}/PointGen({})", parent_id, name),
            tx_id,
            _type: config._type.clone(),
            name,
            value: Value::Bool(false),
            status: Status::Invalid,
            is_changed: false,
            history: config.history.clone(),
            alarm: config.alarm,
            timestamp: Utc::now(),
        }
    }
    ///
    /// 
    fn to_point(&self) -> Option<PointType> {
        if self.is_changed {
            match &self._type {
                PointConfigType::Bool => {
                    Some(PointType::Bool(Point::new(
                        self.tx_id, 
                        &self.name, 
                        Bool(test_data_bool().as_bool()), 
                        self.status, 
                        Cot::Inf,
                        self.timestamp,
                    )))
                },
                PointConfigType::Int => {
                    Some(PointType::Int(Point::new(
                        self.tx_id, 
                        &self.name, 
                        test_data_int().as_int(), 
                        self.status, 
                        Cot::Inf,
                        self.timestamp,
                    )))
                },
                PointConfigType::Real => {
                    Some(PointType::Real(Point::new(
                        self.tx_id, 
                        &self.name, 
                        test_data_real().as_real(), 
                        self.status, 
                        Cot::Inf,
                        self.timestamp,
                    )))
                },
                PointConfigType::Double => {
                    Some(PointType::Double(Point::new(
                        self.tx_id, 
                        &self.name, 
                        test_data_double().as_double(), 
                        self.status, 
                        Cot::Inf,
                        self.timestamp,
                    )))
                },
                PointConfigType::String => {
                    Some(PointType::String(Point::new(
                        self.tx_id, 
                        &self.name, 
                        test_data_double().as_double().to_string(), 
                        self.status, 
                        Cot::Inf,
                        self.timestamp,
                    )))
                },
                PointConfigType::Json => {
                    Some(PointType::String(Point::new(
                        self.tx_id, 
                        &self.name, 
                        json!(test_data_double().as_double()).to_string(), 
                        self.status, 
                        Cot::Inf,
                        self.timestamp,
                    )))
                },
            }
        } else {
            None
        }
    }
    //
    //
    fn add_value(&mut self, input: &Value, timestamp: DateTime<Utc>) {
        // if input != &self.value {
        // }
        self.value = input.clone();
        self.status = Status::Ok;
        self.timestamp = timestamp;
        self.is_changed = true;
    }    
}
///
impl ParsePoint<Value> for PointGen {
    //
    //
    fn next(&mut self, value: &Value, timestamp: DateTime<Utc>) -> Option<PointType> {
        self.add_value(value, timestamp);
        match self.to_point() {
            Some(point) => {
                self.is_changed = false;
                Some(point)
            },
            None => None,
        }
    }
    //
    //
    fn next_status(&mut self, status: Status) -> Option<PointType> {
        self.status = status;
        self.timestamp = Utc::now();
        self.to_point()
    }
    //
    //
    fn is_changed(&self) -> bool {
        self.is_changed
    }
}



pub trait ParsePoint<T> {
    ///
    /// Returns new point parsed from the data slice [bytes] with the given [timestamp] and Status::Ok
    fn next(&mut self, input: &T, timestamp: DateTime<Utc>) -> Option<PointType>;
    ///
    /// Returns new point (prevously parsed) with the given [status]
    fn next_status(&mut self, status: Status) -> Option<PointType>;
    ///
    /// Returns true if value or status was updated since last call [addRaw()]
    fn is_changed(&self) -> bool;
}


fn get_random_index(len: usize) -> usize {
    let mut rnd = rand::thread_rng();
    rnd.gen_range(0..len)
}


fn test_data_bool() -> Value {
    let data = vec![
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(false),
        Value::Bool(true),
        Value::Bool(false),
    ];
    let index = get_random_index(data.len());
    data[index].clone()
}
fn test_data_int() -> Value {
    let data = vec![
        Value::Int(0),
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
        Value::Int(5),
        Value::Int(6),
        Value::Int(7),
        Value::Int(8),
        Value::Int(9),
    ];
    let index = get_random_index(data.len());
    data[index].clone()
}
fn test_data_real() -> Value {
    let data = vec![
        Value::Real(0.0),
        Value::Real(1.0),
        Value::Real(2.0),
        Value::Real(3.0),
        Value::Real(4.0),
        Value::Real(5.0),

    ];
    let index = get_random_index(data.len());
    data[index].clone()
}
fn test_data_double() -> Value {
    let data = vec![
        Value::Double(0.0),
        Value::Double(1.0),
        Value::Double(2.0),
        Value::Double(3.0),
        Value::Double(4.0),
        Value::Double(5.0),
    ];
    let index = get_random_index(data.len());
    data[index].clone()
}
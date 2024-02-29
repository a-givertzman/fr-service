//!
//! JdsService implements behavior on the JDS communication protocol for the following kinds of requests:
//! Basic configuration parameters
//! service JdsService Id:
//!     parameter: value    # meaning
//!     parameter: value    # meaning
//! ```
use std::{collections::HashMap, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use log::{debug, info};
use crate::{
    conf::{jds_service_config::jds_service_config::JdsServiceConfig, point_config::point_config::PointConfig}, core_::{cot::cot::Cot, point::point_type::PointType}, services::{multi_queue::subscription_criteria::SubscriptionCriteria, service::Service, services::Services}
};


///
/// Supported kinds of requests
/// - "Auth" request - authentication requested
/// - "Points" - all points configurations requested
/// - "Subscribe" - after this request points transmission begins
pub struct JdsService {
    id: String,
    rxSend: HashMap<String, Sender<PointType>>,
    rxRecv: Vec<Receiver<PointType>>,
    conf: JdsServiceConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl JdsService {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: JdsServiceConfig, services: Arc<Mutex<Services>>) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/JdsService({})", parent.into(), conf.name),
            rxSend: HashMap::from([(conf.rx.clone(), send)]),
            rxRecv: vec![recv],
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Service for JdsService {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let self_conf_name = self.conf.name.clone();
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            let points = CONFIGS.iter().map(|conf| {
                let conf = serde_yaml::from_str(conf).unwrap();
                PointConfig::from_yaml(&self_conf_name, &conf)
            });
            let points = points.map(|point_conf| {
                SubscriptionCriteria::new(&point_conf.name, Cot::Act)
            }).collect::<Vec<SubscriptionCriteria>>();
            debug!("{}.write | Points subscribed on: ({})", self_id, points.len());
            for name in &points {
                println!("\t{:?}", name);
            }

            loop {
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        info!("{}.run | started", self.id);
        handle
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}

const CONFIGS: &[&str] = &[
    r#"Auth.Secret:
        type: String      # Bool / Int / Float / String / Json
        comment: Auth request, contains token / pass string"#,
    r#"Auth.Ssh:
        type: String      # Bool / Int / Float / String / Json
        comment: Auth request, contains SSH key"#,
    r#"Points.All:
        type: String      # Bool / Int / Float / String / Json
        comment: Request on all Ponts configurations"#,
    r#"Subscribe.All:
        type: String      # Bool / Int / Float / String / Json
        comment: Request to begin transmossion of all configured Points"#,
];
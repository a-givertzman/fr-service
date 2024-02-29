//!
//! JdsService implements behavior on the JDS communication protocol for the following kinds of requests:
//! Basic configuration parameters
//! service JdsService Id:
//!     parameter: value    # meaning
//!     parameter: value    # meaning
//! ```
use std::{collections::HashMap, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use concat_string::concat_string;
use log::{debug, info};
use crate::{
    conf::{jds_service_config::jds_service_config::JdsServiceConfig, point_config::{point_config::PointConfig, point_name::PointName}}, core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, point::point_type::PointType}, services::{multi_queue::subscription_criteria::SubscriptionCriteria, service::Service, services::Services}
};


///
/// Supported kinds of requests
/// - "Auth" request - authentication requested
/// - "Points" - all points configurations requested
/// - "Subscribe" - after this request points transmission begins
pub struct JdsService {
    id: String,
    parent: String,
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
        let parent = parent.into();
        Self {
            id: format!("{}/JdsService({})", parent, conf.name),
            parent: parent,
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
        let parent = self.parent.clone();
        let self_conf = self.conf.clone();
        let services = self.services.clone();
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            let points = CONFIGS.iter().map(|conf| {
                let conf = serde_yaml::from_str(conf).unwrap();
                PointConfig::from_yaml(&self_conf.name, &conf)
            });
            let points = points.map(|point_conf| {
                SubscriptionCriteria::new(&PointName::new(&parent, &point_conf.name).full(), Cot::Req)
            }).collect::<Vec<SubscriptionCriteria>>();
            debug!("{}.run | Points subscribed on: ({})", self_id, points.len());
            for name in &points {
                println!("\t{:?}", name);
            }
            // let rx_recv = services.lock().unwrap().subscribe(&self_conf.rx, &self_id, &points);
            let rx_recv = services.lock().unwrap().subscribe("MultiQueue", &self_id, &points);
            'main: while !exit.load(Ordering::SeqCst) {
                match rx_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(point) => {
                        debug!("{}.run | request: {:?}", self_id, point);
                    },
                    Err(err) => {
                        match err {
                            mpsc::RecvTimeoutError::Timeout => {},
                            mpsc::RecvTimeoutError::Disconnected => {
                                panic!("{}.run | Send error: {:?}", self_id, err);
                            },
                        }
                    },
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
//!
//! JdsService implements behavior on the JDS communication protocol for the following kinds of requests:
//! Basic configuration parameters
//! service JdsService Id:
//!     parameter: value    # meaning
//!     parameter: value    # meaning
//! ```
use std::{collections::HashMap, hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use concat_string::concat_string;
use hashers::fx_hash::FxHasher;
use log::{debug, info, warn};
use crate::{
    conf::{jds_service_config::jds_service_config::JdsServiceConfig, point_config::{point_config::PointConfig, point_name::PointName}}, core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status}, services::{multi_queue::subscription_criteria::SubscriptionCriteria, service::Service, services::Services}
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
    requests: HashMap<String, String, BuildHasherDefault<FxHasher>>,
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
        let mut requests = HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
        requests.insert(PointName::new(&parent, "JdsService/Auth.Secret").full(), 0);
        requests.insert(PointName::new(&parent, "JdsService/Auth.Secret").full(), 0);
        Self {
            id: format!("{}/JdsService({})", parent, conf.name),
            parent: parent,
            rxSend: HashMap::from([(conf.rx.clone(), send)]),
            rxRecv: vec![recv],
            conf: conf.clone(),
            services,
            requests: HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    fn match_request<'a>(self_id: &'a str, request: &'a PointType) -> Option<&'a str> {
        match request.name() {
            name if name == point_name_auth_secret => {
                Some(r#"{
                        \"reply\": \"Auth.Secret Reply\"
                    }"#)
            },
            name if name == point_name_auth_ssh => {
                Some(r#"{
                    \"reply\": \"Auth.Ssh Reply\"
                }"#)
            },
            _ => {
                warn!("{}.run | Unknown request name: {:?}", self_id, request.name());
                None
            }
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
        let tx_id = PointTxId::fromStr(&self_id);
        let exit = self.exit.clone();
        let parent = self.parent.clone();
        let self_conf = self.conf.clone();
        let tx_send = self.services.lock().unwrap().get_link(&self.conf.tx);
        let services = self.services.clone();
        let point_name_auth_secret = PointName::new(&parent, "JdsService/Auth.Secret").full();
        let point_name_auth_ssh = PointName::new(&parent, "JdsService/Auth.Ssh").full();
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
                        debug!("{}.run | request: \n\t{:?}", self_id, point);
                        match Self::match_request(&self_id, &point) {
                            Some(point) => {
                                let point = PointType::String(Point::new(
                                    tx_id, 
                                    &PointName::new(&parent, "JdsService/Auth.Secret").full(),
                                    r#"{
                                        \"reply\": \"Auth.Secret Reply\"
                                    }"#.to_string(), 
                                    Status::Ok, 
                                    Cot::ReqCon, 
                                    chrono::offset::Utc::now(),
                                ));                                
                                match tx_send.send(point) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        panic!("{}.run | Send error: {:?}", self_id, err);
                                    },
                                };
                            },
                            None => {},
                        }
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
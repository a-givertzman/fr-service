//!
//! JdsService implements behavior on the JDS communication protocol for the following kinds of requests:
//! Basic configuration parameters
//! service JdsService Id:
//!     parameter: value    # meaning
//!     parameter: value    # meaning
//! ```
use std::{collections::HashMap, hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use concat_string::concat_string;
use const_format::formatcp;
use hashers::fx_hash::FxHasher;
use log::{debug, info, warn};
use regex::RegexBuilder;
use serde_json::json;
use crate::{
    conf::{jds_service_config::jds_service_config::JdsServiceConfig, point_config::{point_config::PointConfig, point_name::PointName}}, core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status}, services::{multi_queue::subscription_criteria::SubscriptionCriteria, service::Service, services::Services}
};

use super::request_kind::RequestKind;


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
    fn send_reply(self_id: &str, tx_send: &Sender<PointType>, reply: PointType) {
        match tx_send.send(reply) {
            Ok(_) => {},
            Err(err) => {
                panic!("{}.run | Send error: {:?}", self_id, err);
            },
        };
    }
    ///
    /// Detecting kind of the request stored as json string in the incoming point.
    /// Performs the action depending on the Request kind.
    fn match_request<'a>(parent: &str, self_id: &'a str, tx_id: usize, request: &'a PointType, tx_send: &Sender<PointType>, services: &Arc<Mutex<Services>>) {
        let point_name_auth_secret = PointName::new(&parent, "JdsService/Auth.Secret").full();
        let point_name_auth_ssh = PointName::new(&parent, "JdsService/Auth.Ssh").full();
        match RequestKind::from(request.name()) {
            RequestKind::AuthSecret => {
                Self::send_reply(
                    self_id,
                    tx_send,
                    PointType::String(Point::new(
                        tx_id, 
                        &PointName::new(&parent, "JdsService/Auth.Secret").full(),
                        r#"{
                            \"reply\": \"Auth.Secret Reply\"
                        }"#.to_string(), 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    )),
                );
            },
            RequestKind::AuthSsh => {
                Self::send_reply(
                    self_id,
                    tx_send,
                    PointType::String(Point::new(
                        tx_id, 
                        &PointName::new(&parent, "JdsService/Auth.Secret").full(),
                        r#"{
                            \"reply\": \"Auth.Ssh Reply\"
                        }"#.to_string(), 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    )),
                );
            },
            RequestKind::Points => {
                let points = services.lock().unwrap().points();
                let points = json!(points).to_string();
                Self::send_reply(
                    self_id,
                    tx_send,
                    PointType::String(Point::new(
                        tx_id, 
                        &PointName::new(&parent, "JdsService/Points").full(),
                        points, 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    )),
                );
            },
            RequestKind::Subscribe => {
                Self::send_reply(
                    self_id,
                    tx_send,
                    PointType::String(Point::new(
                        tx_id, 
                        &PointName::new(&parent, "JdsService/Subscribe").full(),
                        r#"{
                            \"reply\": \"Subscribe\"
                        }"#.to_string(), 
                        Status::Ok, 
                        Cot::ReqCon, 
                        chrono::offset::Utc::now(),
                    )),
                );
            },
            RequestKind::Unknown => {
                warn!("{}.run | Unknown request name: {:?}", self_id, request.name());
            },
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
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            let points = REQUEST_YAML_CONFIGS.iter().map(|conf| {
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
                        Self::match_request(&parent, &self_id, tx_id,&point, &tx_send, &services);
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
///
/// Used to create Point configurations to sibscribe on all kind of requests
const REQUEST_YAML_CONFIGS: &[&str] = &[
    formatcp!(
        r#"{}:
            type: String      # Bool / Int / Float / String / Json
            comment: Auth request, contains token / pass string"#, 
        RequestKind::AUTH_SECRET
    ),
    formatcp!(
        r#"{}:
            type: String      # Bool / Int / Float / String / Json
            comment: Auth request, contains SSH key"#, 
        RequestKind::AUTH_SSH,
    ),
    formatcp!(
        r#"{}:
            type: String      # Bool / Int / Float / String / Json
            comment: Request all Ponts configurations"#, 
        RequestKind::POINTS,
    ),
    formatcp!(
        r#"{}:
            type: String      # Bool / Int / Float / String / Json
            comment: Request to begin transmossion of all configured Points"#, 
        RequestKind::SUBSCRIBE,
    ),
];

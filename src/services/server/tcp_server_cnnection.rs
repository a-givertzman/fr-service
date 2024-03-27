use std::{
    thread, 
    time::{Duration, Instant},
    collections::HashMap, hash::BuildHasherDefault, 
    sync::{atomic::{AtomicBool, Ordering}, 
    mpsc::{Receiver, RecvTimeoutError}, Arc, Mutex, RwLock}, 
};
use hashers::fx_hash::FxHasher;
use log::{debug, info, warn};
use serde_json::json;
use crate::{
    conf::tcp_server_config::TcpServerConfig, 
    core_::{
        cot::cot::Cot, 
        constants::constants::RECV_TIMEOUT, 
        point::point_type::PointType,
        net::protocols::jds::{
            jds_decode_message::JdsDecodeMessage, 
            jds_deserialize::JdsDeserialize, 
            jds_encode_message::JdsEncodeMessage, 
            jds_routes::{JdsRoutes, RouterReply}, jds_serialize::JdsSerialize,
        }, 
    }, 
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, 
        queue_name::QueueName, 
        server::jds_connection::JdsConnection, 
        service::service_handles::ServiceHandles, 
        services::Services,
        server::{connections::Action, tcp_server_auth::TcpServerAuth}
    }, 
    tcp::{tcp_read_alive::TcpReadAlive, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive},
};

///
/// 
pub enum JdsState {
    Unknown,
    Authenticated,
}
///
/// 
impl From<usize> for JdsState {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Authenticated,
            _ => Self::Unknown,
        }
    }
}
///
/// 
pub struct Shared {
    pub tx_queue_name: String,
    pub jds_state: JdsState,
    pub auth: TcpServerAuth,
}

///
/// Single Jds over TCP connection
pub struct TcpServerConnection {
    id: String,
    path: String,
    action_recv: Vec<Receiver<Action>>, 
    services: Arc<Mutex<Services>>, 
    conf: TcpServerConfig, 
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpServerConnection {
    ///
    /// Creates new instance of the [TcpServerConnection]
    /// - parent - id of the parent
    /// - path - path of the parent
    pub fn new(parent: impl Into<String>, path: impl Into<String>, action_recv: Receiver<Action>, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Self {
        Self {
            id: format!("{}/TcpServerConnection", parent.into()),
            path: format!("{}/Jds", path.into()),
            action_recv: vec![action_recv],
            services,
            conf,
            exit,
        }
    }
    ///
    /// Main loop of the connection 
    pub fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let self_path = self.path.clone();
        let conf = self.conf.clone();
        let shared_options: Arc<RwLock<Shared>> = Arc::new(RwLock::new(Shared {
                tx_queue_name: String::new(), 
                jds_state: match conf.auth {
                    TcpServerAuth::None => JdsState::Authenticated,
                    _                   => JdsState::Unknown,
                }, 
                auth: conf.auth.clone(), 
        }));
        let self_conf_tx = conf.tx.clone();
        let rx_max_length = conf.rx_max_len;
        let exit = self.exit.clone();
        let exit_pair = Arc::new(AtomicBool::new(false));
        let action_recv = self.action_recv.pop().unwrap();
        let services = self.services.clone();
        let tx_queue_name = QueueName::new(&self_conf_tx);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let receivers = Arc::new(RwLock::new(
                HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            ));
            receivers.write().unwrap().insert(Cot::Req, services.lock().unwrap().get_link(&self_conf_tx));
            // let recv = services.lock().unwrap().get_link(&self_conf_tx);
            let points = services.lock().unwrap().points().iter().fold(vec![], |mut points, point_conf| {
                // points.push(SubscriptionCriteria::new(&point_conf.name, Cot::Inf));
                // points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActCon));
                // points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActErr));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqCon));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqErr));
                points
            });
            let send = services.lock().unwrap().get_link(&self_conf_tx);
            println!("{}.run | tx_queue_name: {:?}", self_id, tx_queue_name);
            let (req_reply_send, recv) = services.lock().unwrap().subscribe(tx_queue_name.service(), &self_id, &points);
            shared_options.write().unwrap().tx_queue_name = tx_queue_name.service().to_owned();
            let buffered = rx_max_length > 0;
            let mut tcp_read_alive = TcpReadAlive::new(
                &self_id,
                Arc::new(Mutex::new(JdsRoutes::new(
                    &self_id,
                    &self_path,
                    services.clone(),
                    JdsDeserialize::new(
                        self_id.clone(),
                        JdsDecodeMessage::new(
                            &self_id,
                        ),
                    ),
                    req_reply_send,
                    |parent, path, point, services, shared| {
                        let parent: String = parent;
                        let path: String = path;
                        let point: PointType = point;
                        println!("{}.run | point from socket: \n\t{:?}", path, point);
                        match point.cot() {
                            Cot::Req => JdsConnection::handle_request(&parent, &path, 0, point, services, shared),
                            _        => {
                                match shared.read().unwrap().jds_state {
                                    JdsState::Unknown => {
                                        warn!("{}.run | Rejected point from socket: \n\t{:?}", parent, json!(&point).to_string());
                                        RouterReply::new(None, None)
                                    },
                                    JdsState::Authenticated => {
                                        debug!("{}.run | Passed point from socket: \n\t{:?}", path, json!(&point).to_string());
                                        RouterReply::new(Some(point), None)
                                    },
                                }
                                // RouterReply::new(Some(point), None)
                            },
                        }
                    },
                    shared_options,
                ))),
                send,
                Duration::from_millis(10),
                Some(exit.clone()),
                Some(exit_pair.clone()),
            );
            let tcp_write_alive = TcpWriteAlive::new(
                &self_id,
                Duration::from_millis(10),
                Arc::new(Mutex::new(TcpStreamWrite::new(
                    &self_id,
                    buffered,
                    Some(rx_max_length as usize),
                    Box::new(JdsEncodeMessage::new(
                        &self_id,
                        JdsSerialize::new(
                            &self_id,
                            recv,
                        ),
                    )),
                ))),
                Some(exit.clone()),
                Some(exit_pair.clone()),
            );
            let keep_timeout = conf.keep_timeout.unwrap_or(Duration::from_secs(3));
            let mut duration = Instant::now();
            loop {
                exit_pair.store(false, Ordering::SeqCst);
                match action_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(action) => {
                        match action {
                            Action::Continue(tcp_stream) => {
                                info!("{}.run | Action - Continue received", self_id);
                                let h_read = tcp_read_alive.run(tcp_stream.try_clone().unwrap());
                                let h_write = tcp_write_alive.run(tcp_stream);
                                h_read.join().unwrap_or_else(|_| panic!("{}.run | Error joining TcpReadAlive thread, probable exit with errors", self_id));
                                h_write.join().unwrap_or_else(|_| panic!("{}.run | Error joining TcpWriteAlive thread, probable exit with errors", self_id));
                                info!("{}.run | Finished", self_id);
                                duration = Instant::now();
                            },
                            Action::Exit => {
                                info!("{}.run | Action - Exit received", self_id);
                                break;
                            },
                        }
                    },
                    Err(err) => {
                        match err {
                            RecvTimeoutError::Timeout => {},
                            RecvTimeoutError::Disconnected => {
                                break;
                            },
                        }
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    info!("{}.run | Detected exit", self_id);
                    break;
                }
                if keep_timeout.checked_sub(duration.elapsed()).is_none() {
                    info!("{}.run | Keeped lost connection timeout({:?}) exceeded", self_id, keep_timeout);
                    break;
                }
            }
            info!("{}.run | Exit", self_id);
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            },
            Err(err) => {
                let message = format!("{}.run | Start faled: {:#?}", self.id, err);
                warn!("{}", message);
                Err(message)
            },
        }
    }
}
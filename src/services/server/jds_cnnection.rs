use std::{
    thread, 
    time::{Duration, Instant},
    collections::HashMap, hash::BuildHasherDefault, 
    sync::{atomic::{AtomicBool, Ordering}, 
    mpsc::{Receiver, RecvTimeoutError}, Arc, Mutex, RwLock}, 
};
use hashers::fx_hash::FxHasher;
use log::{debug, error, info, trace, warn};
use serde_json::json;
use crate::{
    conf::{point_config::name::Name, tcp_server_config::TcpServerConfig}, 
    core_::{
        constants::constants::RECV_TIMEOUT, cot::cot::Cot, net::protocols::jds::{
            jds_decode_message::JdsDecodeMessage, 
            jds_deserialize::JdsDeserialize, 
            jds_encode_message::JdsEncodeMessage, 
            jds_serialize::JdsSerialize,
        }, point::point_type::PointType 
    }, 
    services::{
        multi_queue::subscription_criteria::SubscriptionCriteria, 
        queue_name::QueueName, 
        safe_lock::SafeLock, 
        server::{
            connections::Action, 
            jds_request::JdsRequest, 
            jds_routes::{JdsRoutes, RouterReply}, 
            jds_auth::TcpServerAuth,
        }, 
        service::service_handles::ServiceHandles, 
        services::Services,
    }, 
    tcp::{tcp_read_alive::TcpReadAlive, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive},
};

///
/// 
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Shared {
    pub subscribe: String,
    pub subscribe_receiver: String,
    pub jds_state: JdsState,
    pub auth: TcpServerAuth,
    pub connection_id: String,
}

///
/// Single Jds over TCP connection
pub struct JdsConnection {
    id: String,
    name: Name,
    connection_id: String,
    action_recv: Vec<Receiver<Action>>, 
    services: Arc<Mutex<Services>>, 
    conf: TcpServerConfig, 
    exit: Arc<AtomicBool>,
}
///
/// 
impl JdsConnection {
    ///
    /// Creates new instance of the [JdsConnection]
    /// - parent - id of the parent
    /// - path - path of the parent
    pub fn new(parent_id: &str, parent: &Name, connection_id: &str, action_recv: Receiver<Action>, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Self {
        let id = format!("{}/JdsConnection/{}", parent_id, connection_id);
        let name = Name::new(parent, "Jds");
        error!("{}.new | NAME: {:#?}",id, name);
        Self {
            id, //: format!("{}/JdsConnection/{}", parent_id, connection_id),
            name,   //: Name::new(parent, "Jds"),
            connection_id: connection_id.into(),
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
        let self_name = self.name.clone();
        let conf = self.conf.clone();
        let self_conf_send_to = conf.tx.clone();
        let receiver_name = Name::new(&self_name, &self.connection_id).join();
        let subscribe = QueueName::new(&self_conf_send_to).service().to_owned();
        let shared_options: Arc<RwLock<Shared>> = Arc::new(RwLock::new(Shared {
                subscribe: subscribe.clone(), 
                subscribe_receiver: receiver_name.clone(), 
                jds_state: match conf.auth {
                    TcpServerAuth::None => JdsState::Authenticated,
                    _                   => JdsState::Unknown,
                }, 
                auth: conf.auth.clone(),
                connection_id: self.connection_id.clone(),
        }));
        let rx_max_length = conf.rx_max_len;
        let exit = self.exit.clone();
        let exit_pair = Arc::new(AtomicBool::new(false));
        let action_recv = self.action_recv.pop().unwrap();
        let services = self.services.clone();
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let receivers = Arc::new(RwLock::new(
                HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            ));
            receivers.write().unwrap().insert(Cot::Req, services.slock().get_link(&self_conf_send_to));
            // let recv = services.slock().get_link(&self_conf_tx);
            let points = services.slock().points(&self_id).iter().fold(vec![], |mut points, point_conf| {
                // points.push(SubscriptionCriteria::new(&point_conf.name, Cot::Inf));
                // points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActCon));
                // points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActErr));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqCon));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqErr));
                points
            });
            let send = services.slock().get_link(&self_conf_send_to).unwrap_or_else(|err| {
                panic!("{}.run | services.get_link error: {:#?}", self_id, err);
            });
            println!("{}.run | subscribe: {:?}", self_id, subscribe);
            let (req_reply_send, recv) = services.slock().subscribe(&subscribe, &receiver_name, &points);
            let buffered = rx_max_length > 0;
            let mut tcp_read_alive = TcpReadAlive::new(
                &self_id,
                Arc::new(Mutex::new(JdsRoutes::new(
                    &self_id,
                    &self_name,
                    services.clone(),
                    JdsDeserialize::new(
                        format!("{}/TcpReadAlive/JdsRoutes", self_id),
                        JdsDecodeMessage::new(
                            format!("{}/TcpReadAlive/JdsRoutes/JdsDeserialize", self_id),
                        ),
                    ),
                    req_reply_send,
                    |parent_id, parent_name, point, services, shared| {
                        let parent_id: String = parent_id;
                        let parent: Name = parent_name;
                        let point: PointType = point;
                        debug!("{}.run | point from socket: Point( name: {:?}, status: {:?}, cot: {:?}, timestamp: {:?})", parent, point.name(), point.status(), point.cot(), point.timestamp());
                        trace!("{}.run | point from socket: \n\t{:?}", parent, point);
                        match point.cot() {
                            Cot::Req => JdsRequest::handle(&parent_id, &parent, 0, point, services, shared),
                            _        => {
                                match shared.read().unwrap().jds_state {
                                    JdsState::Unknown => {
                                        warn!("{}.run | Rejected point from socket: \n\t{:?}", parent_id, json!(&point).to_string());
                                        RouterReply::new(None, None)
                                    },
                                    JdsState::Authenticated => {
                                        debug!("{}.run | Passed point from socket: \n\t{:?}", parent, json!(&point).to_string());
                                        RouterReply::new(Some(point), None)
                                    },
                                }
                            },
                        }
                    },
                    shared_options,
                ))),
                send,
                conf.cycle.unwrap_or(Duration::from_millis(50)),
                Some(exit.clone()),
                Some(exit_pair.clone()),
            );
            let tcp_write_alive = TcpWriteAlive::new(
                &self_id,
                conf.cycle.unwrap_or(Duration::from_millis(50)),
                Arc::new(Mutex::new(TcpStreamWrite::new(
                    format!("{}/TcpWriteAlive", self_id),
                    buffered,
                    Some(rx_max_length as usize),
                    Box::new(JdsEncodeMessage::new(
                        format!("{}/TcpWriteAlive/TcpStreamWrite", self_id),
                        JdsSerialize::new(
                            format!("{}/TcpWriteAlive/TcpStreamWrite/JdsEncodeMessage", self_id),
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
use std::{collections::HashMap, hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, mpsc::{Receiver, RecvTimeoutError}, Arc, Mutex, RwLock}, thread::{self, JoinHandle}, time::{Duration, Instant}};
use hashers::fx_hash::FxHasher;
use log::{error, info, trace};
use crate::{
    conf::tcp_server_config::TcpServerConfig, 
    core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, net::protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize, jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}}, 
    services::{multi_queue::subscription_criteria::SubscriptionCriteria, queue_name::QueueName, services::Services}, 
    tcp::{steam_read::StreamFilter, tcp_read_alive::{Router, TcpReadAlive}, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive},
};
use super::connections::Action;
use concat_string::concat_string;

///
/// Single Jds over TCP connection
pub struct TcpServerConnection {
    id: String,
    action_recv: Vec<Receiver<Action>>, 
    services: Arc<Mutex<Services>>, 
    filter: Option<StreamFilter>,
    conf: TcpServerConfig, 
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpServerConnection {
    ///
    /// - filter - all trafic from server to client will be filtered by some criterias, until Subscribe request confirmed:
    ///    - cot - [Cot] - bit mask wich will be passed
    ///    - name - exact name wich passed
    pub fn new(parent: impl Into<String>, action_recv: Receiver<Action>, services: Arc<Mutex<Services>>, filter: Option<StreamFilter>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Self {
        Self {
            id: format!("{}/TcpServerConnection", parent.into()),
            action_recv: vec![action_recv],
            services,
            filter,
            conf,
            exit,
        }
    }
    ///
    /// Waiting for Subscribe request confirmation from JdsService for the Connection
    fn await_subscribe_rec_con(self_id: String, services: Arc<Mutex<Services>>, filter: Arc<Mutex<Option<StreamFilter>>>, restart: Arc<AtomicBool>) {
        match *filter.clone().lock().unwrap() {
            Some(_) => {
                let _ = thread::Builder::new().name(format!("{}.await_subscribe_rec_con", self_id.clone())).spawn(move || {
                    let jds_service = services.lock().unwrap().get("JdsService");
                    let jds_points = jds_service.lock().unwrap().points().iter().fold(vec![], |mut points, point_conf| {
                        let point_name = &concat_string!(self_id, point_conf.name);
                        points.push(SubscriptionCriteria::new(point_name, Cot::ReqCon));
                        points.push(SubscriptionCriteria::new(point_name, Cot::ReqErr));
                        points
                    });
                    let (_, jds_recv) = jds_service.lock().unwrap().subscribe(&self_id, &jds_points);
                    loop {
                        match jds_recv.recv() {
                            Ok(point) => {
                                trace!("{}.await_subscribe_rec_con | Point: {:?}", self_id, point);
                                let mut filter_lock = filter.lock().unwrap();
                                *filter_lock = None;
                                restart.store(true, Ordering::SeqCst);
                                break;
                            },
                            Err(err) => {
                                error!("{}.await_subscribe_rec_con | Receive error: {:?}", self_id, err);
                            },
                        }
                    }
                    match jds_service.lock().unwrap().unsubscribe(&self_id, &jds_points) {
                        Ok(_) => {},
                        Err(err) => {
                            error!("{}.await_subscribe_rec_con | Unsubscribe error: {:?}", self_id, err);
                        },
                    };
                });
            },
            None => {},
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let self_id_clone = self.id.clone();
        let conf = self.conf.clone();
        let self_conf_tx = conf.tx.clone();
        let rx_max_length = conf.rxMaxLength;
        let exit = self.exit.clone();
        let exit_pair = Arc::new(AtomicBool::new(false));
        let action_recv = self.action_recv.pop().unwrap();
        let services = self.services.clone();
        let filter = Arc::new(Mutex::new(self.filter.clone()));
        let tx_queue_name = QueueName::new(&self_conf_tx);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let receivers = Arc::new(RwLock::new(
                HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
            ));
            receivers.write().unwrap().insert(Cot::Req, services.lock().unwrap().get_link(&self_conf_tx));
            let recv = services.lock().unwrap().get_link(&self_conf_tx);
            let points = services.lock().unwrap().points().iter().fold(vec![], |mut points, point_conf| {
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::Inf));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActCon));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActErr));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqCon));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqErr));
                points
            });
            let send = services.lock().unwrap().get_link(&self_conf_tx);
            let (send, recv) = services.lock().unwrap().subscribe(tx_queue_name.service(), &self_id, &points);
            let buffered = rx_max_length > 0;
            let mut tcp_read_alive = TcpReadAlive::new(
                &self_id,
                Arc::new(Mutex::new(Router::new(
                    &self_id,
                    JdsDeserialize::new(
                        self_id.clone(),
                        JdsDecodeMessage::new(
                            &self_id,
                        ),
                    ),
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
            let keep_timeout = conf.keepTimeout.unwrap_or(Duration::from_secs(3));
            let mut duration = Instant::now();
            Self::await_subscribe_rec_con(self_id.clone(), services.clone(), filter.clone(), exit_pair.clone());
            loop {
                exit_pair.store(false, Ordering::SeqCst);
                match action_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(action) => {
                        match action {
                            Action::Continue(tcp_stream) => {
                                info!("{}.run | Action - Continue received", self_id);
                                let hr = tcp_read_alive.run(tcp_stream.try_clone().unwrap());
                                let hw = tcp_write_alive.run(tcp_stream);
                                hr.join().unwrap();
                                hw.join().unwrap();
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
        info!("{}.run | Started", self_id_clone);
        handle
    }    
}

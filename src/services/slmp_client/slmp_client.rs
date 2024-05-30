use std::{collections::HashMap, fmt::Debug, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread, time::Duration};
use log::{debug, info, warn};
use testing::stuff::wait::WaitTread;
use crate::{
    conf::{diag_keywd::DiagKeywd, point_config::name::Name, slmp_client_config::slmp_client_config::SlmpClientConfig, tcp_client_config::TcpClientConfig}, core_::{net::protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize, jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, object::object::Object, point::{point_tx_id::PointTxId, point_type::PointType}, types::map::IndexMapFxHasher}, services::{diagnosis::diag_point::DiagPoint, safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services}, tcp::{
        tcp_client_connect::TcpClientConnect, tcp_read_alive::TcpReadAlive, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive
    } 
};
///
/// - Connects to the SLMP device (FX5 Eth module)
/// - Cyclically reads adressess from the SLMP device and yields changed to the MultiQueue
/// - Writes Point to the protocol (SLMP device) specific address
pub struct SlmpClient {
    tx_id: usize,
    id: String,
    name: Name,
    conf: SlmpClientConfig,
    services: Arc<Mutex<Services>>,
    tcp_recv_alive: Option<Arc<Mutex<TcpReadAlive>>>,
    tcp_send_alive: Option<Arc<Mutex<TcpWriteAlive>>>,
    diagnosis: Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
    exit: Arc<AtomicBool>,
}
//
// 
impl SlmpClient {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(conf: SlmpClientConfig, services: Arc<Mutex<Services>>) -> Self {
        let tx_id = PointTxId::fromStr(&conf.name.join());
        let diagnosis = Arc::new(Mutex::new(conf.diagnosis.iter().map(|(keywd, conf)| {
            (keywd.to_owned(), DiagPoint::new(tx_id, conf.clone()))
        }).collect()));
        Self {
            tx_id,
            id: conf.name.join(),
            name: conf.name.clone(),
            conf: conf.clone(),
            services,
            tcp_recv_alive: None,
            tcp_send_alive: None,
            diagnosis,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
//
//
impl Object for SlmpClient {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl Debug for SlmpClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SlmpClient")
            .field("id", &self.id)
            .finish()
    }
}
//
// 
impl Service for SlmpClient {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let tx_send = self.services.slock().get_link(&self.conf.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self_id, err);
        });
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        let exit_pair = Arc::new(AtomicBool::new(false));
        let tx_send = self.services.slock().get_link(&conf.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.id, err);
        });
        // let buffered = conf.rx_buffered; // TODO Read this from config
        // let in_recv = self.in_recv.pop().unwrap();
        // let (cyclic, cycleInterval) = match conf.cycle {
        //     Some(interval) => (interval > Duration::ZERO, interval),
        //     None => (false, Duration::ZERO),
        // };
        let reconnect = conf.reconnect_cycle.clone();
        let mut tcp_client_connect = TcpClientConnect::new(
            self_id.clone(), 
            format!("{}:{}", conf.ip, conf.port),
            reconnect,
        );
        let mut tcp_read_alive = TcpReadAlive::new(
            &self_id,
            Arc::new(Mutex::new(
                JdsDeserialize::new(
                    self_id.clone(),
                    JdsDecodeMessage::new(
                        &self_id,
                    ),
                ),
            )),
            tx_send,
            Some(Duration::from_millis(10)),
            Some(exit.clone()),
            Some(exit_pair.clone()),
        );
        // let tcp_write_alive = TcpWriteAlive::new(
        //     &self_id,
        //     None,
        //     Arc::new(Mutex::new(TcpStreamWrite::new(
        //         &self_id,
        //         buffered,
        //         Some(conf.rx_max_len as usize),
        //         Box::new(JdsEncodeMessage::new(
        //             &self_id,
        //             JdsSerialize::new(
        //                 &self_id,
        //                 in_recv,
        //             ),
        //         )),
        //     ))),
        //     Some(exit.clone()),
        //     Some(exit_pair.clone()),
        // );
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            loop {
                exit_pair.store(false, Ordering::SeqCst);
                if let Some(tcp_stream) = tcp_client_connect.connect() {
                    let h_r = tcp_read_alive.run(tcp_stream.try_clone().unwrap());
                    // let h_w = tcp_write_alive.run(tcp_stream);
                    h_r.wait().unwrap();
                    // h_w.wait().unwrap();
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
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
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        match &self.tcp_recv_alive {
            Some(tcp_recv_alive) => {
                tcp_recv_alive.slock().exit()
            }
            None => {}
        }
        match &self.tcp_send_alive {
            Some(tcp_send_alive) => {
                tcp_send_alive.slock().exit()
            }
            None => {}
        }
    }
}

use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, time::Duration, collections::HashMap, thread};

use log::{debug, info, warn};
use testing::stuff::wait::WaitTread;

use crate::{
    conf::tcp_client_config::TcpClientConfig, core_::{net::protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize, jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, object::object::Object, point::point_type::PointType}, services::{service::{service::Service, service_handles::ServiceHandles}, services::Services}, tcp::{
        tcp_client_connect::TcpClientConnect, tcp_read_alive::TcpReadAlive, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive
    } 
};
///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
pub struct TcpClient {
    id: String,
    in_send: HashMap<String, Sender<PointType>>,
    in_recv: Vec<Receiver<PointType>>,
    conf: TcpClientConfig,
    services: Arc<Mutex<Services>>,
    tcp_recv_alive: Option<Arc<Mutex<TcpReadAlive>>>,
    tcp_send_alive: Option<Arc<Mutex<TcpWriteAlive>>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpClient {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: TcpClientConfig, services: Arc<Mutex<Services>>) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/TcpClient({})", parent.into(), conf.name),
            in_recv: vec![recv],
            in_send: HashMap::from([(conf.rx.clone(), send)]),
            conf: conf.clone(),
            services,
            tcp_recv_alive: None,
            tcp_send_alive: None,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
impl Object for TcpClient {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for TcpClient {
    //
    // 
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.in_send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        let exit_pair = Arc::new(AtomicBool::new(false));
        info!("{}.run | rx queue name: {:?}", self.id, conf.rx);
        info!("{}.run | tx queue name: {:?}", self.id, conf.tx);
        debug!("{}.run | Lock services...", self_id);
        let tx_send = self.services.lock().unwrap().get_link(&conf.tx);
        debug!("{}.run | Lock services - ok", self_id);
        let buffered = conf.rx_buffered; // TODO Read this from config
        let in_recv = self.in_recv.pop().unwrap();
        // let (cyclic, cycleInterval) = match conf.cycle {
        //     Some(interval) => (interval > Duration::ZERO, interval),
        //     None => (false, Duration::ZERO),
        // };
        let reconnect = conf.reconnect_cycle.unwrap_or(Duration::from_secs(3));
        let mut tcp_client_connect = TcpClientConnect::new(
            self_id.clone(), 
            conf.address, 
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
                Some(conf.rx_max_len as usize),
                Box::new(JdsEncodeMessage::new(
                    &self_id,
                    JdsSerialize::new(
                        &self_id,
                        in_recv,
                    ),
                )),
            ))),
            Some(exit.clone()),
            Some(exit_pair.clone()),
        );
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            loop {
                exit_pair.store(false, Ordering::SeqCst);
                if let Some(tcpStream) = tcp_client_connect.connect() {
                    let h_r = tcp_read_alive.run(tcpStream.try_clone().unwrap());
                    let h_w = tcp_write_alive.run(tcpStream);
                    h_r.wait().unwrap();
                    h_w.wait().unwrap();
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
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        match &self.tcp_recv_alive {
            Some(tcpRecvAlive) => {
                tcpRecvAlive.lock().unwrap().exit()
            },
            None => {},
        }
        match &self.tcp_send_alive {
            Some(tcpSendAlive) => {
                tcpSendAlive.lock().unwrap().exit()
            },
            None => {},
        }
    }
}

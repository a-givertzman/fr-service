#![allow(non_snake_case)]

use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, time::Duration, collections::HashMap, thread::{self, JoinHandle}};

use log::{info, debug};

use crate::{
    conf::tcp_client_config::TcpClientConfig, core_::{net::protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize, jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, object::object::Object, point::point_type::PointType}, services::{service::service::Service, services::Services}, tcp::{
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
    inSend: HashMap<String, Sender<PointType>>,
    inRecv: Vec<Receiver<PointType>>,
    conf: TcpClientConfig,
    services: Arc<Mutex<Services>>,
    tcpRecvAlive: Option<Arc<Mutex<TcpReadAlive>>>,
    tcpSendAlive: Option<Arc<Mutex<TcpWriteAlive>>>,
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
            inRecv: vec![recv],
            inSend: HashMap::from([(conf.rx.clone(), send)]),
            conf: conf.clone(),
            services,
            tcpRecvAlive: None,
            tcpSendAlive: None,
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
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        let exitPair = Arc::new(AtomicBool::new(false));
        info!("{}.run | rx queue name: {:?}", self.id, conf.rx);
        info!("{}.run | tx queue name: {:?}", self.id, conf.tx);
        debug!("{}.run | Lock services...", self_id);
        let txSend = self.services.lock().unwrap().get_link(&conf.tx);
        debug!("{}.run | Lock services - ok", self_id);
        let buffered = conf.rx_buffered; // TODO Read this from config
        let inRecv = self.inRecv.pop().unwrap();
        // let (cyclic, cycleInterval) = match conf.cycle {
        //     Some(interval) => (interval > Duration::ZERO, interval),
        //     None => (false, Duration::ZERO),
        // };
        let reconnect = conf.reconnect_cycle.unwrap_or(Duration::from_secs(3));
        let mut tcpClientConnect = TcpClientConnect::new(
            self_id.clone(), 
            conf.address, 
            reconnect,
        );
        let mut tcpReadAlive = TcpReadAlive::new(
            &self_id,
            Arc::new(Mutex::new(
                JdsDeserialize::new(
                    self_id.clone(),
                    JdsDecodeMessage::new(
                        &self_id,
                    ),
                ),
            )),
            txSend,
            Duration::from_millis(10),
            Some(exit.clone()),
            Some(exitPair.clone()),
        );
        let tcpWriteAlive = TcpWriteAlive::new(
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
                        inRecv,
                    ),
                )),
            ))),
            Some(exit.clone()),
            Some(exitPair.clone()),
        );
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            loop {
                exitPair.store(false, Ordering::SeqCst);
                match tcpClientConnect.connect() {
                    Some(tcpStream) => {
                        let hR = tcpReadAlive.run(tcpStream.try_clone().unwrap());
                        let hW = tcpWriteAlive.run(tcpStream);
                        hR.join().unwrap();
                        hW.join().unwrap();
                    },
                    None => {},
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        info!("{}.run | started", self.id);
        handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        match &self.tcpRecvAlive {
            Some(tcpRecvAlive) => {
                tcpRecvAlive.lock().unwrap().exit()
            },
            None => {},
        }
        match &self.tcpSendAlive {
            Some(tcpSendAlive) => {
                tcpSendAlive.lock().unwrap().exit()
            },
            None => {},
        }
    }
}

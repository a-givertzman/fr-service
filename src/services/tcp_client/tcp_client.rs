#![allow(non_snake_case)]

use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, time::Duration, collections::HashMap, thread::{self, JoinHandle}};

use log::{info, debug};

use crate::{
    core_::{point::point_type::PointType, net::protocols::jds::{jds_serialize::JdsSerialize, jds_encode_message::JdsEncodeMessage}},
    conf::tcp_client_config::TcpClientConfig,
    services::{service::Service, services::Services}, 
    tcp::{
        tcp_client_connect::TcpClientConnect, 
        tcp_stream_write::TcpStreamWrite, 
        tcp_write_alive::TcpWriteAlive, 
        tcp_read_alive::TcpReadAlive
    }, 
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
///
/// 
impl Service for TcpClient {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    // 
    fn getLink(&self, name: &str) -> Sender<PointType> {
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        info!("{}.run | rx queue name: {:?}", self.id, conf.rx);
        info!("{}.run | tx queue name: {:?}", self.id, conf.tx);
        debug!("{}.run | Getting services...", selfId);
        let outSend = self.services.lock().unwrap().getLink(&conf.tx);
        debug!("{}.run | Getting services - ok", selfId);
        let outSend = Arc::new(Mutex::new(outSend));
        let buffered = true; // TODO Read this from config
        let inRecv = self.inRecv.pop().unwrap();
        // let (cyclic, cycleInterval) = match conf.cycle {
        //     Some(interval) => (interval > Duration::ZERO, interval),
        //     None => (false, Duration::ZERO),
        // };
        let reconnect = if conf.reconnectCycle.is_some() {conf.reconnectCycle.unwrap()} else {Duration::from_secs(3)};
        let _queueMaxLength = conf.rxMaxLength;
        let mut tcpClientConnect = TcpClientConnect::new(
            selfId.clone(), 
            conf.address, 
            reconnect,
        );
        let tcpReadAlive = TcpReadAlive::new(
            &selfId,
            outSend,
            Duration::from_millis(10),
        );
        let tcpWriteAlive = TcpWriteAlive::new(
            &selfId,
            Duration::from_millis(10),
            Arc::new(Mutex::new(TcpStreamWrite::new(
                &selfId,
                buffered,
                Some(conf.rxMaxLength as usize),
                Box::new(JdsEncodeMessage::new(
                    &selfId,
                    JdsSerialize::new(
                        &selfId,
                        inRecv,
                    ),
                )),
            ))),
        );
        info!("{}.run | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{} - Read", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            loop {
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

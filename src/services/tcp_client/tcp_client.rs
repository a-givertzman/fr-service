#![allow(non_snake_case)]

use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, time::Duration, collections::HashMap, thread::{self, JoinHandle}, net::TcpStream};

use log::{info, debug, warn};

use crate::{
    core_::{point::point_type::PointType, net::{connection_status::ConnectionStatus, protocols::jds::{jds_serialize::JdsSerialize, jds_encode_message::JdsEncodeMessage}}},
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
    inRecv: Vec<Receiver<PointType>>,
    inSend: HashMap<String, Sender<PointType>>,
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
            inSend: HashMap::from([(conf.recvQueue.clone(), send)]),
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
    ///
    /// returns sender of the TcpClient input queue by name
    fn getLink(&self, name: &str) -> Sender<PointType> {
        match self.inSend.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// The TcpClient main loop
    fn run(&mut self) {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        info!("{}.run | in queue name: {:?}", self.id, conf.recvQueue);
        info!("{}.run | out queue name: {:?}", self.id, conf.sendQueue);
        let recvQueueParts: Vec<&str> = conf.sendQueue.split(".").collect();
        let receiverServiceName = recvQueueParts[0];
        let receiverQueueName = recvQueueParts[1];
        debug!("{}.run | Getting services...", selfId);
        let services = self.services.lock().unwrap();
        debug!("{}.run | Getting services - ok", selfId);

        let outSend = services.get(&receiverServiceName).lock().unwrap().getLink(receiverQueueName);
        let outSend = Arc::new(Mutex::new(outSend));
        let buffered = true; // TODO Read this from config
        let inRecv = self.inRecv.pop().unwrap();
        // let (cyclic, cycleInterval) = match conf.cycle {
        //     Some(interval) => (interval > Duration::ZERO, interval),
        //     None => (false, Duration::ZERO),
        // };
        let reconnect = if conf.reconnectCycle.is_some() {conf.reconnectCycle.unwrap()} else {Duration::from_secs(3)};
        let _queueMaxLength = conf.recvQueueMaxLength;
        let mut tcpClientConnect = TcpClientConnect::new(
            selfId.clone(), 
            conf.address, 
            reconnect,
        );
        let tcpRecvAlive = TcpReadAlive::new(
            &selfId,
            outSend,
            Duration::from_millis(10),
        );
        let tcpSendAlive = TcpWriteAlive::new(
            &selfId,
            Arc::new(Mutex::new(TcpStreamWrite::new(
                &selfId,
                buffered,
                Some(conf.recvQueueMaxLength as usize),
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
        let _handle = thread::Builder::new().name(format!("{} - Read", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            loop {
                match tcpClientConnect.connect() {
                    Some(tcpStream) => {
                        let hR = tcpRecvAlive.run(tcpStream.try_clone().unwrap());
                        let hW = tcpSendAlive.run(tcpStream);
                        hR.join().unwrap();
                        hW.join().unwrap();
                    },
                    None => {},
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        }).unwrap();
        info!("{}.run | started", self.id);
    }
    ///
    /// 
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

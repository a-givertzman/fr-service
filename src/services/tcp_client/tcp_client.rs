#![allow(non_snake_case)]

use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, time::Duration, collections::HashMap, thread::{self, JoinHandle}, net::TcpStream, io::{Read, Write}, rc::Rc};

use log::{info, debug, warn};

use crate::{
    core_::{point::point_type::PointType, net::{connection_status::ConnectionStatus, protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize, jds_serialize::JdsSerialize, jds_encode_message::JdsEncodeMessage}}, retain_buffer::retain_buffer::RetainBuffer},
    conf::tcp_client_config::TcpClientConfig,
    services::{service::Service, task::task_cycle::ServiceCycle, services::Services}, tcp::{tcp_socket_client_connect::TcpSocketClientConnect, tcp_stream_write::TcpStreamWrite}, 
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
    outRecv: Vec<Receiver<PointType>>,
    outSend: HashMap<String, Sender<PointType>>,
    conf: TcpClientConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
    exitRW: Arc<AtomicBool>,
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
            outRecv: vec![],
            outSend: HashMap::new(),
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
            exitRW: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    ///
    fn readSocket(selfId: String, mut tcpStream: JdsDeserialize, send: Arc<Mutex<Sender<PointType>>>, exit: Arc<AtomicBool>, isConnected: Arc<AtomicBool>) -> JoinHandle<()> {
        let handle = thread::Builder::new().name(format!("{} - Read", selfId.clone())).spawn(move || {
            let send = send.lock().unwrap();
            loop {
                match tcpStream.read() {
                    ConnectionStatus::Active(point) => {
                        match point {
                            Ok(point) => {
                                match send.send(point) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        warn!("{}.readSocket | write to queue error: {:?}", selfId, err);
                                    },
                                };
                            },
                            Err(err) => {
                                warn!("{}.readSocket | error: {:?}", selfId, err);
                            },
                        }
                    },
                    ConnectionStatus::Closed(err) => {
                        isConnected.store(false, Ordering::SeqCst);
                        exit.store(true, Ordering::SeqCst);
                        warn!("{}.readSocket | error: {:?}", selfId, err);
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        }).unwrap();
        handle
    }
    ///
    /// 
    fn writeSocket(selfId: String, streamWrite: Arc<Mutex<TcpStreamWrite>>, mut tcpStream: TcpStream, exit: Arc<AtomicBool>, isConnected: Arc<AtomicBool>) -> JoinHandle<()> {
        let _hW = thread::Builder::new().name(format!("{} - Write", selfId.clone())).spawn(move || {
            let mut streamWrite = streamWrite.lock().unwrap();
            loop {
                match streamWrite.write(&mut tcpStream) {
                    Ok(_) => {},
                    Err(err) => {
                        warn!("{}.writeSocket | error: {:?}", selfId, err);
                        isConnected.store(false, Ordering::SeqCst);
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        }).unwrap();
        _hW
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
        let exit = self.exit.clone();
        let exitRW = self.exitRW.clone();
        let conf = self.conf.clone();
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
        let _handle = thread::Builder::new().name(format!("{} - main", selfId)).spawn(move || {
            info!("{}.run | Tread main starting...", selfId);
            let isConnected = Arc::new(AtomicBool::new(false));
            let tcpStreamWrite = Arc::new(Mutex::new(TcpStreamWrite::new(
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
            )));
            // let mut cycle = ServiceCycle::new(cycleInterval);
            let mut connect = TcpSocketClientConnect::new(selfId.clone() + "/TcpSocketClientConnect", conf.address);
            let mut tcpStream = None;
            'main: loop {
                if !isConnected.load(Ordering::SeqCst) {
                    tcpStream = connect.connect(reconnect);
                    match tcpStream {
                        Some(stream) => {
                            if let Err(err) = stream.set_read_timeout(Some(Duration::from_secs(10))) {
                                debug!("{}.run | TcpStream.set_timeout error: {:?}", selfId, err);
                            };
                            isConnected.store(true, Ordering::SeqCst);
                            let outSend = outSend.clone();
                            let streamRead = JdsDeserialize::new(
                                selfId.clone(),
                                JdsDecodeMessage::new(
                                    selfId.clone(),
                                    stream.try_clone().unwrap(),
                                ),
                            );
                            let handleR = Self::readSocket(
                                selfId.clone(),
                                streamRead,
                                outSend,
                                exitRW.clone(),
                                isConnected.clone()
                            );
                            let handleW = Self::writeSocket(
                                selfId.clone(),
                                tcpStreamWrite.clone(),
                                stream,
                                exitRW.clone(),
                                isConnected.clone()
                            );
                            handleR.join().unwrap();
                            handleW.join().unwrap();
                        },
                        None => {
                            isConnected.store(false, Ordering::SeqCst);
                        },
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            };
            info!("{}.run | stopped", selfId);
        }).unwrap();
        info!("{}.run | started", self.id);
    }
    ///
    /// 
    fn exit(&self) {
        self.exitRW.store(true, Ordering::SeqCst);
        self.exit.store(true, Ordering::SeqCst);
    }
}

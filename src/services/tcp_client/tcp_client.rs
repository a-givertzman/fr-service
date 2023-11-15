#![allow(non_snake_case)]

use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, collections::HashMap, thread};

use log::{info, debug, trace};

use crate::{
    core_::point::point_type::PointType,
    conf::tcp_client_config::TcpClientConfig,
    services::{service::Service, task::task_cycle::ServiceCycle}, tcp::tcp_socket_client_connect::TcpSocketClientConnect, 
};



///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
pub struct TcpClient {
    id: String,
    recv: Vec<Receiver<PointType>>,
    send: HashMap<String, Sender<PointType>>,
    conf: TcpClientConfig,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpClient {
    ///
    /// 
    pub fn new(id: impl Into<String>, conf: TcpClientConfig) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: id.into(),
            recv: vec![recv],
            send: HashMap::from([(conf.recvQueue.clone(), send)]),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Service for TcpClient {
    ///
    /// returns sender of the ApiClient queue by name
    fn getLink(&self, name: impl Into<String>) -> Sender<PointType> {
        let name = name.into();
        match self.send.get(&name) {
            Some(send) => send.clone(),
            None => panic!("ApiClient({}).run | link '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// 
    fn run(&mut self) {
        info!("ApiClient({}).run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let recv = self.recv.pop().unwrap();
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let reconnect = if conf.reconnectCycle.is_some() {conf.reconnectCycle.unwrap()} else {Duration::from_secs(3)};
        let _queueMaxLength = conf.recvQueueMaxLength;
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut isConnected = false;
            // let mut buffer = Vec::new();
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut connect = TcpSocketClientConnect::new(selfId.clone() + "/TcpSocketClientConnect", conf.address);
            let mut stream = None;
            'main: loop {
                if !isConnected {
                    stream = connect.connect(reconnect);
                    match &stream {
                        Some(stream) => {
                            match stream.set_read_timeout(Some(Duration::from_secs(10))) {
                                Ok(_) => {},
                                Err(err) => {
                                    debug!("ApiClient({}).run | TcpStream.set_timeout error: {:?}", selfId, err);
                                },
                            };
                        },
                        None => {},
                    }
                }
                match &mut stream {
                    Some(stream) => {
                        isConnected = true;
                        // cycle.start();
                        // trace!("ApiClient({}).run | step...", selfId);
                        // Self::readQueue(&selfId, &recv, &mut buffer);
                        // let mut count = buffer.len();
                        // while count > 0 {
                        //     match buffer.first() {
                        //         Some(point) => {
                        //             let sql = point.asString().value;
                        //             match Self::send(&selfId, sql, stream) {
                        //                 Ok(_) => {
                        //                     match Self::readAll(&selfId, stream) {
                        //                         ConnectionStatus::Active(bytes) => {
                        //                             let reply = String::from_utf8(bytes).unwrap();
                        //                             debug!("ApiClient({}).run | API reply: {:?}", selfId, reply);
                        //                             let reply: SqlReply = serde_json::from_str(&reply).unwrap();
                        //                             if reply.hasError() {
                        //                                 warn!("ApiClient({}).run | API reply has error: {:?}", selfId, reply.error);
                        //                                 // break;
                        //                             } else {
                        //                                 buffer.remove(0);
                        //                             }
                        //                         },
                        //                         ConnectionStatus::Closed => {
                        //                             isConnected = false;
                        //                             break;
                        //                         },
                        //                     };
                        //                 },
                        //                 Err(err) => {
                        //                     isConnected = false;
                        //                     warn!("ApiClient({}).run | error sending API: {:?}", selfId, err);
                        //                     break;
                        //                 },
                        //             }
                        //         },
                        //         None => {break;},
                        //     };
                        //     count -=1;
                        // }
                        // if exit.load(Ordering::SeqCst) {
                        //     break 'main;
                        // }
                        // trace!("ApiClient({}).run | step - done ({:?})", selfId, cycle.elapsed());
                        // if cyclic {
                        //     cycle.wait();
                        // }
                    },
                    None => {
                        isConnected = false;
                    },
                }
            };
            info!("ApiClient({}).run | stopped", selfId);
        }).unwrap();
        info!("ApiClient({}).run | started", self.id);
        // h.join().unwrap();
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
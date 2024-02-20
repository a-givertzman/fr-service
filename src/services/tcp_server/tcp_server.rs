#![allow(non_snake_case)]

use log::{info, warn, debug};
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{Sender, Receiver, self, SendError}}, 
    thread::{self, JoinHandle}, 
    net::{TcpListener, TcpStream, Shutdown}, time::{Duration, Instant}, any::Any, collections::HashMap,
};
use testing::stuff::wait::WaitTread;
use crate::{
    services::{services::Services, service::Service, task::service_cycle::ServiceCycle, queue_name::QueueName}, 
    conf::tcp_server_config::TcpServerConfig, 
    core_::{point::point_type::PointType, net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, constants::constants::RECV_TIMEOUT}, 
    tcp::{tcp_read_alive::TcpReadAlive, tcp_write_alive::TcpWriteAlive, tcp_stream_write::TcpStreamWrite},
};


///
/// Bounds TCP socket server
/// Listening socket for incoming connections
/// Verified incoming connections handles in the separate thread
pub struct TcpServer {
    id: String,
    conf: TcpServerConfig,
    connections: Arc<Mutex<TcpServerConnections>>,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpServer {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: TcpServerConfig, services: Arc<Mutex<Services>>) -> Self {
        let self_id = format!("{}/TcpServer({})", parent.into(), conf.name);
        Self {
            id: self_id.clone(),
            conf: conf.clone(),
            connections: Arc::new(Mutex::new(TcpServerConnections::new(self_id))),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    fn setupConnection(self_id: String, connectionId: &String, stream: TcpStream, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>, connections: Arc<Mutex<TcpServerConnections>>) {
        info!("{}.setupConnection | Trying to repair Connection '{}'...", self_id, connectionId);
        // let connectionsLock = connections.lock().unwrap();
        let repairResult = connections.lock().unwrap().repair(&connectionId, stream.try_clone().unwrap());
        match repairResult {
            Ok(_) => {
                info!("{}.setupConnection | Connection '{}' - reparied", self_id, connectionId);
            },
            Err(err) => {
                info!("{}.run | {}", self_id, err);

                info!("{}.setupConnection | New connection: '{}'", self_id, connectionId);
                let (send, recv) = mpsc::channel();
                let mut connection = TcpServerConnection::new(connectionId.clone(), recv, services.clone(), conf.clone(), exit.clone());
                match connection.run() {
                    Ok(handle) => {
                        match send.send(Action::Continue(stream)) {
                            Ok(_) => {},
                            Err(err) => {
                                warn!("{}.setupConnection | Send tcpStream error {:?}", self_id, err);
                            },
                        }
                        info!("{}.setupConnection | connections.lock...", self_id);
                        connections.lock().unwrap().insert(
                            connectionId,
                            handle,
                            send,
                        );
                        info!("{}.setupConnection | connections.lock - ok", self_id);
                    },
                    Err(err) => {
                        warn!("{}.setupConnection | error: {:?}", self_id, err);
                    },
                };
                info!("{}.setupConnection | Connection '{}' - created new", self_id, connectionId);
            },
        }
    }
    ///
    /// 
    fn setStreamTimout(self_id: &String, stream: &TcpStream, raadTimeout: Duration, writeTimeout: Option<Duration>) {
        match stream.set_read_timeout(Some(raadTimeout)) {
            Ok(_) => {
                info!("{}.setStreamTimout | Socket set read timeout {:?} - ok", self_id, raadTimeout);
            },
            Err(err) => {
                warn!("{}.setStreamTimout | Socket set read timeout error {:?}", self_id, err);
            },
        }
        if let Some(timeout) = writeTimeout {
            match stream.set_write_timeout(Some(timeout)) {
                Ok(_) => {
                    info!("{}.setStreamTimout | Socket set write timeout {:?} - ok", self_id, timeout);
                },
                Err(err) => {
                    warn!("{}.setStreamTimout | Socket set write timeout error {:?}", self_id, err);
                },
            }
        }
    }
}
///
/// 
impl Service for TcpServer {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    // 
    fn get_link(&mut self, _name: &str) -> Sender<PointType> {
        panic!("{}.getLink | Does not support getLink", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        let connections = self.connections.clone();
        let services = self.services.clone();
        let reconnectCycle = conf.reconnectCycle.unwrap_or(Duration::ZERO);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let mut cycle = ServiceCycle::new(reconnectCycle);
            'main: loop {
                cycle.start();
                info!("{}.run | Open socket {}...", self_id, conf.address);
                match TcpListener::bind(conf.address) {
                    Ok(listener) => {
                        info!("{}.run | Open socket {} - ok", self_id, conf.address);
                        for stream in listener.incoming() {
                            if exit.load(Ordering::SeqCst) {
                                debug!("{}.run | Detected exit", self_id);
                                break;
                            }
                            match stream {
                                Ok(stream) => {
                                    let remIp = stream.peer_addr().map_or("Unknown remote IP".to_string(), |a| {a.ip().to_string()});
                                    let connectionId = format!("{}-{}", self_id, remIp);
                                    Self::setStreamTimout(&self_id, &stream, RECV_TIMEOUT, None);
                                    info!("{}.run | Setting up Connection '{}'...", self_id, connectionId);
                                    Self::setupConnection(self_id.clone(), &connectionId, stream, services.clone(), conf.clone(), exit.clone(), connections.clone());
                                },
                                Err(err) => {
                                    warn!("{}.run | error: {:?}", self_id, err);
                                },
                            }
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | error: {:?}", self_id, err);
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                cycle.wait();
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
            info!("{}.run | Exit...", self_id);
            // Self::waitConnections(&self_id, connections);
            connections.lock().unwrap().wait();
            info!("{}.run | Exit", self_id);
        });
        info!("{}.run | Started", self.id);
        handle
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(10));
        info!("{}.exit | Final connection...", self.id);
        match TcpStream::connect_timeout(&self.conf.address, Duration::from_millis(100)) {
            Ok(stream) => {
                info!("{}.exit | Final connection - ok", self.id);
                stream.shutdown(Shutdown::Both).unwrap();
            },
            Err(err) => {
                info!("{}.exit | Final connection error: {:?}", self.id, err);
            },
        };
    }    
}

///
/// Keep TCP Server's connection's:
/// - thread JoinHandle
/// - Sender<Action>
#[derive(Debug)]
struct Connection {
    handle: JoinHandle<()>,
    send: Sender<Action>,
}
///
/// 
impl Connection {
    pub fn new(handle: JoinHandle<()>, send: Sender<Action>,) -> Self {
        Self {
            handle: handle,
            send: send,
        }
    }
    ///
    /// 
    pub fn send(&self, action: Action) -> Result<(), SendError<Action>> {
        self.send.send(action)
    }
    ///
    /// 
    pub fn wait(self) -> Result<(), Box<dyn Any + Send>> {
        self.handle.wait()
    }
    ///
    /// 
    pub fn isActive(&self) -> bool {
        !self.handle.is_finished()
    }
}

///
/// 
enum Action {
    Continue(TcpStream),
    Exit,
}


///
/// 
#[derive(Debug)]
struct TcpServerConnections {
    id: String,
    connections: HashMap<String, Connection>,
}
///
/// 
impl TcpServerConnections {
    ///
    /// 
    pub fn new(parent: impl Into<String>) -> Self {
        Self { 
            id: format!("{}/TcpServerConnections", parent.into()),
            connections: HashMap::new(),
        }
    }
    ///
    /// 
    fn insert(&mut self, connectionId: &String, handle: JoinHandle<()>, send: Sender<Action>) {
        info!("{}.insert | connection: '{}'", self.id, connectionId);
        self.connections.insert(
            connectionId.to_string(),
            Connection::new(
                handle,
                send,
            )
        );
    }
    ///
    /// 
    fn repair(&self, connectionId: &String, stream: TcpStream) -> Result<(), String> {
        match self.connections.get(connectionId) {
            Some(conn) => {
                if conn.isActive() {
                    match conn.send(Action::Continue(stream)) {
                        Ok(_) => {
                            // info!("{}.run | Keeped connection '{}' repaired", self_id, connectionId);
                            Ok(())
                        },
                        Err(err) => {
                            Err(format!("{}.repair | Keeped connection repair error {:?}", self.id, err))
                        },
                    }
                } else {
                    Err(format!("{}.repair | Keeped connection '{}' - exceeded", self.id, connectionId))
                }
            },
            None => {
                Err(format!("{}.repair | Keeped connection '{}' - not found", self.id, connectionId))
            },
        }
    }    
    ///
    /// Wait for all connetions handles beeng finished
    fn wait(&mut self) {
        info!("{}.run | connections.lock...", self.id);
        while self.connections.len() > 0 {
            info!("{}.run | connections.lock - ok", self.id);
            // let mut connectionsLock = connections.lock().unwrap();
            let keys: Vec<String> = self.connections.keys().map(|k| {k.to_string()}).collect();
            info!("{}.run | Wait for connections:", self.id);
            for key in &keys {
                info!("{}.run | \tconnection: {:?}\t isActive: {}", self.id, key, self.connections.get(key).unwrap().isActive());
            }
            match keys.first() {
                Some(key) => {
                    let connection = self.connections.remove(key).unwrap();
                    connection.send(Action::Exit).unwrap_or_else(|_| {info!("{}.run | Connection '{}' - already finished", self.id, key)});
                    connection.wait().unwrap();
                },
                None => {
                    break;
                },
            };
        }
    }
}


///
/// 
struct TcpServerConnection {
    id: String,
    actionRecv: Vec<Receiver<Action>>, 
    services: Arc<Mutex<Services>>, 
    conf: TcpServerConfig, 
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpServerConnection {
    ///
    /// 
    pub fn new(parent: impl Into<String>, actionRecv: Receiver<Action>, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Self {
        Self {
            id: format!("{}/TcpServerConnection", parent.into()),
            actionRecv: vec![actionRecv],
            services,
            conf,
            exit,
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let self_idClone = self.id.clone();
        let conf = self.conf.clone();
        let selfConfTx = conf.tx.clone();
        let rxMaxLength = conf.rxMaxLength;
        let exit = self.exit.clone();
        let exitPair = Arc::new(AtomicBool::new(false));
        let actionRecv = self.actionRecv.pop().unwrap();
        let services = self.services.clone();
        let txQueueName = QueueName::new(&selfConfTx);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let send = services.lock().unwrap().getLink(&selfConfTx);
            let recv = services.lock().unwrap().subscribe(txQueueName.service(), &self_id, &vec![]);
            let buffered = rxMaxLength > 0;
            let mut tcpReadAlive = TcpReadAlive::new(
                &self_id,
                send,
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
                    Some(rxMaxLength as usize),
                    Box::new(JdsEncodeMessage::new(
                        &self_id,
                        JdsSerialize::new(
                            &self_id,
                            recv,
                        ),
                    )),
                ))),
                Some(exit.clone()),
                Some(exitPair.clone()),
            );
            let keepTimeout = conf.keepTimeout.unwrap_or(Duration::from_secs(3));
            let mut duration = Instant::now();
            loop {
                exitPair.store(false, Ordering::SeqCst);
                match actionRecv.recv_timeout(RECV_TIMEOUT) {
                    Ok(action) => {
                        match action {
                            Action::Continue(tcpStream) => {
                                info!("{}.run | Action - Continue received", self_id);
                                let hR = tcpReadAlive.run(tcpStream.try_clone().unwrap());
                                let hW = tcpWriteAlive.run(tcpStream);
                                hR.join().unwrap();
                                hW.join().unwrap();
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
                            mpsc::RecvTimeoutError::Timeout => {},
                            mpsc::RecvTimeoutError::Disconnected => {
                                break;
                            },
                        }
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    info!("{}.run | Detected exit", self_id);
                    break;
                }
                if keepTimeout.checked_sub(duration.elapsed()).is_none() {
                    info!("{}.run | Keeped lost connection timeout({:?}) exceeded", self_id, keepTimeout);
                    break;
                }
            }
            info!("{}.run | Exit", self_id);
        });
        info!("{}.run | Started", self_idClone);
        handle
    }    
}











    // ///
    // /// 
    // fn repairConnection(self_id: &String, connectionId: &String, connections: Arc<Mutex<HashMap<String, Connection>>>, stream: TcpStream) -> Result<(), String> {
    //     match connections.lock().unwrap().get(connectionId) {
    //         Some(conn) => {
    //             if conn.isActive() {
    //                 match conn.send(Action::Continue(stream)) {
    //                     Ok(_) => {
    //                         // info!("{}.run | Keeped connection '{}' repaired", self_id, connectionId);
    //                         Ok(())
    //                     },
    //                     Err(err) => {
    //                         Err(format!("{}.run | Keeped connection repair error {:?}", self_id, err))
    //                     },
    //                 }
    //             } else {
    //                 Err(format!("{}.run | Keeped connection '{}' - exceeded", self_id, connectionId))
    //             }
    //         },
    //         None => {
    //             Err(format!("{}.run | Keeped connection '{}' - not found", self_id, connectionId))
    //         },
    //     }
    // }



    //     ///
    // /// 
    // fn waitConnections(self_id: &String, connections: Arc<Mutex<HashMap<String, Connection>>>) {
    //     info!("{}.run | connections.lock...", self_id);
    //     while connections.lock().unwrap().len() > 0 {
    //         info!("{}.run | connections.lock - ok", self_id);
    //         let mut connectionsLock = connections.lock().unwrap();
    //         let keys: Vec<String> = connectionsLock.keys().map(|k| {k.to_string()}).collect();
    //         info!("{}.run | Wait for connections:", self_id);
    //         for key in &keys {
    //             info!("{}.run | \tconnection: {:?}\t isActive: {}", self_id, key, connectionsLock.get(key).unwrap().isActive());
    //         }
    //         match keys.first() {
    //             Some(key) => {
    //                 let connection = connectionsLock.remove(key).unwrap();
    //                 connection.send(Action::Exit).unwrap_or_else(|_| {info!("{}.run | Connection '{}' - already finished", self_id, key)});
    //                 connection.wait().unwrap();
    //             },
    //             None => {
    //                 break;
    //             },
    //         };
    //     }
    // }




    //     ///
    // /// Setup thread for incomming connection
    // fn connection(self_id: String, actionRecv: Receiver<Action>, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Result<JoinHandle<()>, std::io::Error> {
    //     info!("{}.connection | starting...", self_id);
    //     let self_idClone = self_id.clone();
    //     let selfConfTx = conf.tx.clone();
    //     let txQueueName = QueueName::new(&selfConfTx);
    //     info!("{}.connection | Preparing thread...", self_id);
    //     let handle = thread::Builder::new().name(format!("{}.connection", self_id.clone())).spawn(move || {
    //         info!("{}.connection | Preparing thread - ok", self_id);
    //         let send = services.lock().unwrap().getLink(&selfConfTx);
    //         let recv = services.lock().unwrap().subscribe(txQueueName.service(), &self_id, &vec![]);
    //         let buffered = conf.rxMaxLength > 0;
    //         let mut tcpReadAlive = TcpReadAlive::new(
    //             &self_id,
    //             send,
    //             Duration::from_millis(10),
    //             Some(exit.clone()),
    //         );
    //         let tcpWriteAlive = TcpWriteAlive::new(
    //             &self_id,
    //             Duration::from_millis(10),
    //             Arc::new(Mutex::new(TcpStreamWrite::new(
    //                 &self_id,
    //                 buffered,
    //                 Some(conf.rxMaxLength as usize),
    //                 Box::new(JdsEncodeMessage::new(
    //                     &self_id,
    //                     JdsSerialize::new(
    //                         &self_id,
    //                         recv,
    //                     ),
    //                 )),
    //             ))),
    //             Some(exit.clone()),
    //         );
    //         let keepTimeout = conf.keepTimeout.unwrap_or(Duration::from_secs(3));
    //         let mut duration = Instant::now();
    //         loop {
    //             match actionRecv.recv_timeout(RECV_TIMEOUT) {
    //                 Ok(action) => {
    //                     match action {
    //                         Action::Continue(tcpStream) => {
    //                             info!("{}.connection | Action - Continue received", self_id);
    //                             let hR = tcpReadAlive.run(tcpStream.try_clone().unwrap());
    //                             let hW = tcpWriteAlive.run(tcpStream);
    //                             hR.join().unwrap();
    //                             hW.join().unwrap();
    //                             info!("{}.connection | Finished", self_id);
    //                             duration = Instant::now();
    //                         },
    //                         Action::Exit => {
    //                             info!("{}.connection | Action - Exit received", self_id);
    //                             break;
    //                         },
    //                     }
    //                 },
    //                 Err(err) => {
    //                     match err {
    //                         mpsc::RecvTimeoutError::Timeout => {},
    //                         mpsc::RecvTimeoutError::Disconnected => {
    //                             break;
    //                         },
    //                     }
    //                 },
    //             }
    //             if exit.load(Ordering::SeqCst) {
    //                 info!("{}.connection | Detected exit", self_id);
    //                 break;
    //             }
    //             if keepTimeout.checked_sub(duration.elapsed()).is_none() {
    //                 info!("{}.connection | Keeped lost connection timeout({:?}) exceeded", self_id, keepTimeout);
    //                 break;
    //             }
    //         }
    //         info!("{}.connection | Exit", self_id);
    //     });
    //     info!("{}.connection | Started", self_idClone);
    //     handle
    // }
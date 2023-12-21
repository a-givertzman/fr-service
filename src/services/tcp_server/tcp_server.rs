#![allow(non_snake_case)]

use log::{info, warn};
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{Sender, Receiver, self, SendError}}, 
    thread::{self, JoinHandle}, 
    net::{TcpListener, TcpStream, Shutdown}, time::{Duration, Instant}, any::Any, collections::HashMap,
};
use crate::{
    services::{services::Services, service::Service, task::service_cycle::ServiceCycle, queue_name::QueueName}, 
    conf::tcp_server_config::TcpServerConfig, 
    core_::{point::point_type::PointType, net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, testing::test_stuff::wait::WaitTread, constants::constants::RECV_TIMEOUT}, 
    tcp::{tcp_read_alive::TcpReadAlive, tcp_write_alive::TcpWriteAlive, tcp_stream_write::TcpStreamWrite},
};


///
/// Bounds TCP socket server
/// Listening socket for incoming connections
/// Verified incoming connections handles in the separate thread
pub struct TcpServer {
    id: String,
    conf: TcpServerConfig,
    connections: Arc<Mutex<HashMap<String, Connection>>>,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpServer {
    ///
    /// 
    pub fn new(parent: impl Into<String>, conf: TcpServerConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: format!("{}/TcpServer({})", parent.into(), conf.name),
            conf: conf.clone(),
            connections: Arc::new(Mutex::new(HashMap::new())),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Setup thread for incomming connection
    fn connection(selfId: String, actionRecv: Receiver<Action>, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.connection | starting...", selfId);
        let selfIdClone = selfId.clone();
        let selfConfTx = conf.tx.clone();
        let txQueueName = QueueName::new(&selfConfTx);
        info!("{}.connection | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.connection", selfId.clone())).spawn(move || {
            info!("{}.connection | Preparing thread - ok", selfId);
            let send = services.lock().unwrap().getLink(&selfConfTx);
            let recv = services.lock().unwrap().subscribe(txQueueName.service(), &selfId, &vec![]);
            let buffered = conf.rxMaxLength > 0;
            let mut tcpReadAlive = TcpReadAlive::new(
                &selfId,
                send,
                Duration::from_millis(10),
                Some(exit.clone()),
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
                            recv,
                        ),
                    )),
                ))),
                Some(exit.clone()),
            );
            let keepTimeout = conf.keepTimeout.unwrap_or(Duration::from_secs(3));
            let mut duration = Instant::now();
            loop {
                match actionRecv.recv_timeout(RECV_TIMEOUT) {
                    Ok(action) => {
                        match action {
                            Action::Continue(tcpStream) => {
                                let hR = tcpReadAlive.run(tcpStream.try_clone().unwrap());
                                let hW = tcpWriteAlive.run(tcpStream);
                                hR.join().unwrap();
                                hW.join().unwrap();
                                duration = Instant::now();
                            },
                            Action::Exit => {
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
                    info!("{}.connection | Detected exit", selfId);
                    break;
                }
                if keepTimeout.checked_sub(duration.elapsed()).is_none() {
                    info!("{}.connection | Keeped lost connection timeout({:?}) exceeded", selfId, keepTimeout);
                    break;
                }
            }
            info!("{}.connection | Exit", selfId);
        });
        info!("{}.connection | Started", selfIdClone);
        handle
    }
    ///
    /// 
    fn newConnection(selfId: String, connectionId: &String, stream: TcpStream, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>, connections: Arc<Mutex<HashMap<String, Connection>>>) {
        info!("{}.newConnection | New connection: '{}'", selfId, connectionId);
        let (send, recv) = mpsc::channel();
        match Self::connection(connectionId.clone(), recv, services.clone(), conf.clone(), exit.clone()) {
            Ok(handle) => {
                match send.send(Action::Continue(stream)) {
                    Ok(_) => {},
                    Err(err) => {
                        warn!("{}.run | Send tcpStream error {:?}", selfId, err);
                    },
                }
                info!("{}.newConnection | connections.lock...", selfId);
                connections.lock().unwrap().insert(
                    connectionId.to_string(),
                    Connection::new(
                        handle,
                        send,
                    )
                );
                info!("{}.newConnection | connections.lock - ok", selfId);
            },
            Err(err) => {
                warn!("{}.run | error: {:?}", selfId, err);
            },
        };
    }
    ///
    /// 
    fn repairConnection(selfId: &String, connectionId: &String, connections: Arc<Mutex<HashMap<String, Connection>>>, stream: TcpStream) -> Result<(), String> {
        match connections.lock().unwrap().get(connectionId) {
            Some(conn) => {
                if conn.isActive() {
                    match conn.send(Action::Continue(stream)) {
                        Ok(_) => {
                            // info!("{}.run | Keeped connection '{}' repaired", selfId, connectionId);
                            Ok(())
                        },
                        Err(err) => {
                            Err(format!("{}.run | Keeped connection repair error {:?}", selfId, err))
                        },
                    }
                } else {
                    Err(format!("{}.run | Keeped connection '{}' - exceeded", selfId, connectionId))
                }
            },
            None => {
                Err(format!("{}.run | Keeped connection '{}' - not found", selfId, connectionId))
            },
        }
    }
    ///
    /// 
    fn setStreamTimout(selfId: &String, stream: &TcpStream, raadTimeout: Duration, writeTimeout: Option<Duration>) {
        match stream.set_read_timeout(Some(raadTimeout)) {
            Ok(_) => {
                info!("{}.run | Socket set read timeout {:?} - ok", selfId, raadTimeout);
            },
            Err(err) => {
                warn!("{}.run | Socket set read timeout error {:?}", selfId, err);
            },
        }
        if let Some(timeout) = writeTimeout {
            match stream.set_write_timeout(Some(timeout)) {
                Ok(_) => {
                    info!("{}.run | Socket set write timeout {:?} - ok", selfId, timeout);
                },
                Err(err) => {
                    warn!("{}.run | Socket set write timeout error {:?}", selfId, err);
                },
            }
        }
    }
    ///
    /// 
    fn waitConnections(selfId: &String, connections: Arc<Mutex<HashMap<String, Connection>>>) {
        info!("{}.run | connections.lock...", selfId);
        while connections.lock().unwrap().len() > 0 {
            info!("{}.run | connections.lock - ok", selfId);
            let mut connectionsLock = connections.lock().unwrap();
            let keys: Vec<String> = connectionsLock.keys().map(|k| {k.to_string()}).collect();
            info!("{}.run | Wait for connections:", selfId);
            for key in &keys {
                info!("{}.run | \tconnection: {:?}\t isActive: {}", selfId, key, connectionsLock.get(key).unwrap().isActive());
            }
            match keys.first() {
                Some(key) => {
                    let connection = connectionsLock.remove(key).unwrap();
                    connection.send(Action::Exit).unwrap_or_else(|_| {info!("{}.run | Connection '{}' - already finished", selfId, key)});
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
impl Service for TcpServer {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    // 
    fn getLink(&mut self, _name: &str) -> Sender<PointType> {
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
        let selfId = self.id.clone();
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        let connections = self.connections.clone();
        let services = self.services.clone();
        let reconnectCycle = conf.reconnectCycle.unwrap_or(Duration::ZERO);
        info!("{}.run | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let mut cycle = ServiceCycle::new(reconnectCycle);
            'main: loop {
                cycle.start();
                info!("{}.run | Open socket {}...", selfId, conf.address);
                match TcpListener::bind(conf.address) {
                    Ok(listener) => {
                        info!("{}.run | Open socket {} - ok", selfId, conf.address);
                        for stream in listener.incoming() {
                            if exit.load(Ordering::SeqCst) {
                                info!("{}.run | Detected exit", selfId);
                                break;
                            }
                            match stream {
                                Ok(stream) => {
                                    let remIp = stream.peer_addr().map_or("Uncnown remote IP".to_string(), |a| {a.ip().to_string()});
                                    let connectionId = format!("{}-{}", selfId, remIp);
                                    Self::setStreamTimout(&selfId, &stream, RECV_TIMEOUT, None);
                                    match Self::repairConnection(&selfId, &connectionId, connections.clone(), stream.try_clone().unwrap()) {
                                        Ok(_) => {
                                            info!("{}.run | Connection '{}' - reparied", selfId, connectionId);
                                        },
                                        Err(err) => {
                                            info!("{}.run | {}", selfId, err);
                                            Self::newConnection(selfId.clone(), &connectionId, stream, services.clone(), conf.clone(), exit.clone(), connections.clone());
                                            info!("{}.run | Connection '{}' - new created", selfId, connectionId);
                                        },
                                    }
                                },
                                Err(err) => {
                                    warn!("{}.run | error: {:?}", selfId, err);
                                },
                            }
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | error: {:?}", selfId, err);
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
            info!("{}.run | Exit...", selfId);
            Self::waitConnections(&selfId, connections);
            info!("{}.run | Exit", selfId);
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
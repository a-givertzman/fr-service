#![allow(non_snake_case)]

use log::{info, warn, debug, error};
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{Sender, Receiver, self, SendError}}, 
    thread::{self, JoinHandle}, 
    net::{TcpListener, TcpStream, Shutdown}, time::Duration, any::Any, collections::HashMap,
};
use crate::{
    services::{services::Services, service::Service, task::service_cycle::ServiceCycle, queue_name::QueueName}, 
    conf::tcp_server_config::TcpServerConfig, 
    core_::{point::point_type::PointType, net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, testing::test_stuff::wait::WaitTread}, 
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
    fn setupConnection(selfId: String, actionRecv: Receiver<Option<TcpStream>>, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.setupConnection | starting...", selfId);
        let selfIdClone = selfId.clone();
        let selfConfTx = conf.tx.clone();
        let txQueueName = QueueName::new(&selfConfTx);
        info!("{}.setupConnection | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.setupConnection", selfId.clone())).spawn(move || {
            info!("{}.setupConnection | Preparing thread - ok", selfId);
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
            for action in actionRecv {
                match action {
                    Some(tcpStream) => {
                        let hR = tcpReadAlive.run(tcpStream.try_clone().unwrap());
                        let hW = tcpWriteAlive.run(tcpStream);
                        hR.join().unwrap();
                        hW.join().unwrap();
                        
                    },
                    None => {
                        break;
                    },
                }
            }
            info!("{}.setupConnection | Exit", selfId);
        });
        info!("{}.setupConnection | started", selfIdClone);
        handle
    }
    ///
    /// 
    fn waitConnections(selfId: String, connections: Arc<Mutex<HashMap<String, Connection>>>) {
        while connections.lock().unwrap().len() > 0 {
            let mut connectionsLock = connections.lock().unwrap();
            let keys: Vec<String> = connectionsLock.keys().map(|k| {k.to_string()}).collect();
            info!("{}.run | Wait for connections: {:?}", selfId, keys);
            match keys.first() {
                Some(key) => {
                    let connection = connectionsLock.remove(key).unwrap();
                    connection.send(None).unwrap();
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
            let mut cycle = ServiceCycle::new(reconnectCycle);
            'main: loop {
                cycle.start();
                info!("{}.run | Open socket {}...", selfId, conf.address);
                match TcpListener::bind(conf.address) {
                    Ok(listener) => {
                        info!("{}.run | Done socket {} - ok", selfId, conf.address);
                        for stream in listener.incoming() {
                            if exit.load(Ordering::SeqCst) {
                                break;
                            }
                            match stream {
                                Ok(stream) => {
                                    let remIp = stream.peer_addr().map_or("Uncnown remote IP".to_string(), |a| {a.to_string()});
                                    let readTimeout = Duration::from_millis(100);
                                    match stream.set_read_timeout(Some(readTimeout)) {
                                        Ok(_) => {
                                            info!("{}.run | Socket set read timeout {:?} - ok", selfId, readTimeout);

                                        },
                                        Err(err) => {
                                            warn!("{}.run | Socket set read timeout error {:?}", selfId, err);
                                            
                                        },
                                    }
                                    let (send, recv) = mpsc::channel();
                                    let connectionId = format!("{}({})", selfId, remIp);
                                    match Self::setupConnection(selfId.clone(), recv, services.clone(), conf.clone(), exit.clone()) {
                                        Ok(handle) => {
                                            match send.send(Some(stream)) {
                                                Ok(_) => {},
                                                Err(err) => {
                                                    warn!("{}.run | Send tcpStream error {:?}", selfId, err);
                                                },
                                            }
                                            connections.lock().unwrap().insert(
                                                connectionId,
                                                Connection::new(
                                                    handle,
                                                    send,
                                                )
                                            );
                                        },
                                        Err(err) => {
                                            warn!("{}.run | error: {:?}", selfId, err);
                                        },
                                    };
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
            Self::waitConnections(selfId, connections);
        });
        info!("{}.run | started", self.id);
        handle
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(10));
        let s = TcpStream::connect_timeout(&self.conf.address, Duration::from_millis(100)).unwrap();
        s.shutdown(Shutdown::Both).unwrap();
    }    
}

///
/// Keep TCP Server's connection's:
/// - thread JoinHandle
/// - Sender<Option<TcpStream>>
struct Connection {
    handle: JoinHandle<()>,
    send: Sender<Option<TcpStream>>,
}
///
/// 
impl Connection {
    pub fn new(handle: JoinHandle<()>, send: Sender<Option<TcpStream>>,) -> Self {
        Self {
            handle: handle,
            send: send,
        }
    }
    ///
    /// 
    pub fn send(&self, action: Option<TcpStream>) -> Result<(), SendError<Option<TcpStream>>> {
        self.send.send(action)
    }
    ///
    /// 
    pub fn wait(self) -> Result<(), Box<dyn Any + Send>> {
        self.handle.wait()
    }
}
use log::{info, warn, debug};
use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::{Sender, Receiver, self, SendError}}, 
    thread::{self, JoinHandle}, 
    net::{TcpListener, TcpStream, Shutdown}, time::{Duration, Instant}, any::Any, collections::HashMap,
};
use testing::stuff::wait::WaitTread;
use crate::{
    conf::tcp_server_config::TcpServerConfig, 
    core_::{constants::constants::RECV_TIMEOUT, cot::cot::Cot, net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, point::point_type::PointType}, 
    services::{multi_queue::subscription_criteria::SubscriptionCriteria, queue_name::QueueName, service::service::Service, services::Services, task::service_cycle::ServiceCycle}, 
    tcp::{tcp_read_alive::TcpReadAlive, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive},
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
    fn setup_connection(self_id: String, connection_id: &String, stream: TcpStream, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>, connections: Arc<Mutex<TcpServerConnections>>) {
        info!("{}.setup_connection | Trying to repair Connection '{}'...", self_id, connection_id);
        // let connectionsLock = connections.lock().unwrap();
        let repair_result = connections.lock().unwrap().repair(&connection_id, stream.try_clone().unwrap());
        match repair_result {
            Ok(_) => {
                info!("{}.setup_connection | Connection '{}' - reparied", self_id, connection_id);
            },
            Err(err) => {
                info!("{}.run | {}", self_id, err);

                info!("{}.setup_connection | New connection: '{}'", self_id, connection_id);
                let (send, recv) = mpsc::channel();
                let mut connection = TcpServerConnection::new(connection_id.clone(), recv, services.clone(), conf.clone(), exit.clone());
                match connection.run() {
                    Ok(handle) => {
                        match send.send(Action::Continue(stream)) {
                            Ok(_) => {},
                            Err(err) => {
                                warn!("{}.setup_connection | Send tcpStream error {:?}", self_id, err);
                            },
                        }
                        info!("{}.setup_connection | connections.lock...", self_id);
                        connections.lock().unwrap().insert(
                            connection_id,
                            handle,
                            send,
                        );
                        info!("{}.setup_connection | connections.lock - ok", self_id);
                    },
                    Err(err) => {
                        warn!("{}.setup_connection | error: {:?}", self_id, err);
                    },
                };
                info!("{}.setup_connection | Connection '{}' - created new", self_id, connection_id);
            },
        }
    }
    ///
    /// 
    fn set_stream_timout(self_id: &String, stream: &TcpStream, raad_timeout: Duration, write_timeout: Option<Duration>) {
        match stream.set_read_timeout(Some(raad_timeout)) {
            Ok(_) => {
                info!("{}.set_stream_timout | Socket set read timeout {:?} - ok", self_id, raad_timeout);
            },
            Err(err) => {
                warn!("{}.set_stream_timout | Socket set read timeout error {:?}", self_id, err);
            },
        }
        if let Some(timeout) = write_timeout {
            match stream.set_write_timeout(Some(timeout)) {
                Ok(_) => {
                    info!("{}.set_stream_timout | Socket set write timeout {:?} - ok", self_id, timeout);
                },
                Err(err) => {
                    warn!("{}.set_stream_timout | Socket set write timeout error {:?}", self_id, err);
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
        panic!("{}.get_link | Does not support get_link", self.id())
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
        let reconnect_cycle = conf.reconnectCycle.unwrap_or(Duration::ZERO);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let mut cycle = ServiceCycle::new(reconnect_cycle);
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
                                    let rem_ip = stream.peer_addr().map_or("Unknown remote IP".to_string(), |a| {a.ip().to_string()});
                                    let connection_id = format!("{}-{}", self_id, rem_ip);
                                    Self::set_stream_timout(&self_id, &stream, RECV_TIMEOUT, None);
                                    info!("{}.run | Setting up Connection '{}'...", self_id, connection_id);
                                    Self::setup_connection(self_id.clone(), &connection_id, stream, services.clone(), conf.clone(), exit.clone(), connections.clone());
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
    pub fn is_active(&self) -> bool {
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
    fn insert(&mut self, connection_id: &String, handle: JoinHandle<()>, send: Sender<Action>) {
        info!("{}.insert | connection: '{}'", self.id, connection_id);
        self.connections.insert(
            connection_id.to_string(),
            Connection::new(
                handle,
                send,
            )
        );
    }
    ///
    /// 
    fn repair(&self, connection_id: &String, stream: TcpStream) -> Result<(), String> {
        match self.connections.get(connection_id) {
            Some(conn) => {
                if conn.is_active() {
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
                    Err(format!("{}.repair | Keeped connection '{}' - exceeded", self.id, connection_id))
                }
            },
            None => {
                Err(format!("{}.repair | Keeped connection '{}' - not found", self.id, connection_id))
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
                info!("{}.run | \tconnection: {:?}\t isActive: {}", self.id, key, self.connections.get(key).unwrap().is_active());
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
    action_recv: Vec<Receiver<Action>>, 
    services: Arc<Mutex<Services>>, 
    conf: TcpServerConfig, 
    exit: Arc<AtomicBool>,
}
///
/// 
impl TcpServerConnection {
    ///
    /// 
    pub fn new(parent: impl Into<String>, action_recv: Receiver<Action>, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Self {
        Self {
            id: format!("{}/TcpServerConnection", parent.into()),
            action_recv: vec![action_recv],
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
        let self_id_clone = self.id.clone();
        let conf = self.conf.clone();
        let self_conf_tx = conf.tx.clone();
        let rx_max_length = conf.rxMaxLength;
        let exit = self.exit.clone();
        let exit_pair = Arc::new(AtomicBool::new(false));
        let action_recv = self.action_recv.pop().unwrap();
        let services = self.services.clone();
        let tx_queue_name = QueueName::new(&self_conf_tx);
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let send = services.lock().unwrap().get_link(&self_conf_tx);
            let points = services.lock().unwrap().points().iter().fold(vec![], |mut points, point_conf| {
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::Inf));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActCon));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ActErr));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqCon));
                points.push(SubscriptionCriteria::new(&point_conf.name, Cot::ReqErr));
                points
            });
            let recv = services.lock().unwrap().subscribe(tx_queue_name.service(), &self_id, &points);
            let buffered = rx_max_length > 0;
            let mut tcp_read_alive = TcpReadAlive::new(
                &self_id,
                send,
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
                    Some(rx_max_length as usize),
                    Box::new(JdsEncodeMessage::new(
                        &self_id,
                        JdsSerialize::new(
                            &self_id,
                            recv,
                        ),
                    )),
                ))),
                Some(exit.clone()),
                Some(exit_pair.clone()),
            );
            let keep_timeout = conf.keepTimeout.unwrap_or(Duration::from_secs(3));
            let mut duration = Instant::now();
            loop {
                exit_pair.store(false, Ordering::SeqCst);
                match action_recv.recv_timeout(RECV_TIMEOUT) {
                    Ok(action) => {
                        match action {
                            Action::Continue(tcp_stream) => {
                                info!("{}.run | Action - Continue received", self_id);
                                let hR = tcp_read_alive.run(tcp_stream.try_clone().unwrap());
                                let hW = tcp_write_alive.run(tcp_stream);
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
                if keep_timeout.checked_sub(duration.elapsed()).is_none() {
                    info!("{}.run | Keeped lost connection timeout({:?}) exceeded", self_id, keep_timeout);
                    break;
                }
            }
            info!("{}.run | Exit", self_id);
        });
        info!("{}.run | Started", self_id_clone);
        handle
    }    
}

use log::{debug, info, warn};
use std::{
    net::{Shutdown, TcpListener, TcpStream}, sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc, Mutex}, thread, time::Duration
};
use crate::{
    conf::tcp_server_config::TcpServerConfig, 
    core_::{constants::constants::RECV_TIMEOUT, object::object::Object}, 
    services::{
        server::{
            connections::{Action, TcpServerConnections}, tcp_server_cnnection::TcpServerConnection
        }, service::{service::Service, service_handles::ServiceHandles}, services::Services, task::service_cycle::ServiceCycle
    }, 
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
    /// Creates new instance of the TcpServer:
    /// - parent - name of the parent entity, used to create self name: "/parent/self_id/"
    /// - filter - all trafic from server to client will be filtered by some criterias, until Subscribe request confirmed:
    ///    - cot - [Cot] - bit mask wich will be passed
    ///    - name - exact name wich passed
    pub fn new(parent: impl Into<String>, conf: TcpServerConfig, services: Arc<Mutex<Services>>, ) -> Self {
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
        let repair_result = connections.lock().unwrap().repair(connection_id, stream.try_clone().unwrap());
        match repair_result {
            Ok(_) => {
                info!("{}.setup_connection | Connection '{}' - reparied", self_id, connection_id);
            },
            Err(err) => {
                info!("{}.run | {}", self_id, err);

                info!("{}.setup_connection | New connection: '{}'", self_id, connection_id);
                let (send, recv) = mpsc::channel();
                let mut connection = TcpServerConnection::new(
                    connection_id.clone(),
                    recv, services.clone(),
                    conf.clone(),
                    exit.clone()
                );
                match connection.run() {
                    Ok(mut handles) => {
                        if handles.len() != 1 {
                            panic!("{}.setup_connection | TcpServerConnection.run must return single handle, but returns {}", self_id, handles.len())
                        }
                        let (_, handle) = handles.into_iter().next().unwrap();
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
impl Object for TcpServer {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for TcpServer {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        let connections = self.connections.clone();
        let services = self.services.clone();
        let reconnect_cycle = conf.reconnect_cycle.unwrap_or(Duration::ZERO);
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

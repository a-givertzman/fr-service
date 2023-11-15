#![allow(non_snake_case)]

use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, thread, collections::HashMap, net::TcpStream, io::{Write, Read}};

use log::{info, debug, trace, warn};

use crate::{
    core_::{point::point_type::PointType, net::connection_status::ConnectionStatus}, 
    conf::api_client_config::ApiClientConfig,
    services::{task::task_cycle::ServiceCycle, api_cient::api_reply::SqlReply, service::Service}, 
    tcp::tcp_socket_client_connect::TcpSocketClientConnect, 
};

use super::api_query::ApiQuery;

///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
pub struct ApiClient {
    id: String,
    recv: Vec<Receiver<PointType>>,
    send: HashMap<String, Sender<PointType>>,
    conf: ApiClientConfig,
    exit: Arc<AtomicBool>,
}
///
/// 
impl ApiClient {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: ApiClientConfig) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/ApiClient({})", parent.into(), conf.name),
            recv: vec![recv],
            send: HashMap::from([(conf.recvQueue.clone(), send)]),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Reads all avalible at the moment items from the in-queue
    fn readQueue(selfId: &str, recv: &Receiver<PointType>, buffer: &mut Vec<PointType>) {
        let maxReadAtOnce = 1000;
        for (index, point) in recv.try_iter().enumerate() {   
            debug!("{}.readQueue | point: {:?}", selfId, &point);
            buffer.push(point);
            if index > maxReadAtOnce {
                break;
            }                 
        }
    }
    ///
    /// Writing sql string to the TcpStream
    fn send(selfId: &str, sql: String, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>>{
        let query = ApiQuery::new("authToken", "id", "database", sql, true, true);
        match stream.write(query.toJson().as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("{}.send | write to tcp stream error: {:?}", selfId, err);
                Err(Box::new(err))
            },
        }
    }
    ///
    /// bytes to be read from socket at once
    const BUF_LEN: usize = 1024 * 4;
    ///
    /// reads all avalible data from the TspStream
    /// - returns Active: if read bytes non zero length without errors
    /// - returns Closed:
    ///    - if read 0 bytes
    ///    - if on error
    fn readAll(selfId: &str, stream: &mut TcpStream) -> ConnectionStatus {
        let mut buf = [0; Self::BUF_LEN];
        let mut result = vec![];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    debug!("{}.readAll |     read len: {:?}", selfId, len);
                    result.append(& mut buf[..len].into());
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return ConnectionStatus::Closed;
                        } else {
                            return ConnectionStatus::Active(result)
                        }
                    }
                },
                Err(err) => {
                    warn!("{}.readAll | error reading from socket: {:?}", selfId, err);
                    warn!("{}.readAll | error kind: {:?}", selfId, err.kind());
                    return match err.kind() {
                        std::io::ErrorKind::NotFound => todo!(),
                        std::io::ErrorKind::PermissionDenied => ConnectionStatus::Closed,
                        std::io::ErrorKind::ConnectionRefused => ConnectionStatus::Closed,
                        std::io::ErrorKind::ConnectionReset => ConnectionStatus::Closed,
                        // std::io::ErrorKind::HostUnreachable => ConnectionStatus::Closed,
                        // std::io::ErrorKind::NetworkUnreachable => ConnectionStatus::Closed,
                        std::io::ErrorKind::ConnectionAborted => ConnectionStatus::Closed,
                        std::io::ErrorKind::NotConnected => ConnectionStatus::Closed,
                        std::io::ErrorKind::AddrInUse => ConnectionStatus::Closed,
                        std::io::ErrorKind::AddrNotAvailable => ConnectionStatus::Closed,
                        // std::io::ErrorKind::NetworkDown => ConnectionStatus::Closed,
                        std::io::ErrorKind::BrokenPipe => ConnectionStatus::Closed,
                        std::io::ErrorKind::AlreadyExists => todo!(),
                        std::io::ErrorKind::WouldBlock => ConnectionStatus::Closed,
                        // std::io::ErrorKind::NotADirectory => todo!(),
                        // std::io::ErrorKind::IsADirectory => todo!(),
                        // std::io::ErrorKind::DirectoryNotEmpty => todo!(),
                        // std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
                        // std::io::ErrorKind::FilesystemLoop => todo!(),
                        // std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
                        std::io::ErrorKind::InvalidInput => todo!(),
                        std::io::ErrorKind::InvalidData => todo!(),
                        std::io::ErrorKind::TimedOut => todo!(),
                        std::io::ErrorKind::WriteZero => todo!(),
                        // std::io::ErrorKind::StorageFull => todo!(),
                        // std::io::ErrorKind::NotSeekable => todo!(),
                        // std::io::ErrorKind::FilesystemQuotaExceeded => todo!(),
                        // std::io::ErrorKind::FileTooLarge => todo!(),
                        // std::io::ErrorKind::ResourceBusy => todo!(),
                        // std::io::ErrorKind::ExecutableFileBusy => todo!(),
                        // std::io::ErrorKind::Deadlock => todo!(),
                        // std::io::ErrorKind::CrossesDevices => todo!(),
                        // std::io::ErrorKind::TooManyLinks => todo!(),
                        // std::io::ErrorKind::InvalidFilename => todo!(),
                        // std::io::ErrorKind::ArgumentListTooLong => todo!(),
                        std::io::ErrorKind::Interrupted => todo!(),
                        std::io::ErrorKind::Unsupported => todo!(),
                        std::io::ErrorKind::UnexpectedEof => todo!(),
                        std::io::ErrorKind::OutOfMemory => todo!(),
                        std::io::ErrorKind::Other => todo!(),
                        _ => ConnectionStatus::Closed,
                    }
                    // return ConnectionStatus::Closed;
                },
            };
        }
    }    
}
///
/// 
impl Service for ApiClient {
    ///
    /// returns sender of the ApiClient queue by name
    fn getLink(&self, name: impl Into<String>) -> Sender<PointType> {
        let name = name.into();
        match self.send.get(&name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// 
    fn run(&mut self) {
        info!("{}.run | starting...", self.id);
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
        let _h = thread::Builder::new().name(format!("{} - main", selfId)).spawn(move || {
            let mut isConnected = false;
            let mut buffer = Vec::new();
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
                                    debug!("{}.run | TcpStream.set_timeout error: {:?}", selfId, err);
                                },
                            };
                        },
                        None => {},
                    }
                }
                match &mut stream {
                    Some(stream) => {
                        isConnected = true;
                        cycle.start();
                        trace!("{}.run | step...", selfId);
                        Self::readQueue(&selfId, &recv, &mut buffer);
                        let mut count = buffer.len();
                        while count > 0 {
                            match buffer.first() {
                                Some(point) => {
                                    let sql = point.asString().value;
                                    match Self::send(&selfId, sql, stream) {
                                        Ok(_) => {
                                            match Self::readAll(&selfId, stream) {
                                                ConnectionStatus::Active(bytes) => {
                                                    let reply = String::from_utf8(bytes).unwrap();
                                                    debug!("{}.run | API reply: {:?}", selfId, reply);
                                                    let reply: SqlReply = serde_json::from_str(&reply).unwrap();
                                                    if reply.hasError() {
                                                        warn!("{}.run | API reply has error: {:?}", selfId, reply.error);
                                                        // break;
                                                    } else {
                                                        buffer.remove(0);
                                                    }
                                                },
                                                ConnectionStatus::Closed => {
                                                    isConnected = false;
                                                    break;
                                                },
                                            };
                                        },
                                        Err(err) => {
                                            isConnected = false;
                                            warn!("{}.run | error sending API: {:?}", selfId, err);
                                            break;
                                        },
                                    }
                                },
                                None => {break;},
                            };
                            count -=1;
                        }
                        if exit.load(Ordering::SeqCst) {
                            break 'main;
                        }
                        trace!("{}.run | step - done ({:?})", selfId, cycle.elapsed());
                        if cyclic {
                            cycle.wait();
                        }
                    },
                    None => {
                        isConnected = false;
                    },
                }
            };
            info!("{}.run | stopped", selfId);
        }).unwrap();
        info!("{}.run | started", self.id);
        // h.join().unwrap();
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
#![allow(non_snake_case)]

use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, thread::{self, JoinHandle}, collections::HashMap, net::TcpStream, io::{Write, Read}};

use log::{info, debug, trace, warn};

use crate::{
    core_::{point::point_type::PointType, net::connection_status::ConnectionStatus, retain_buffer::retain_buffer::RetainBuffer}, 
    conf::api_client_config::ApiClientConfig,
    services::{task::task_cycle::ServiceCycle, api_cient::api_reply::SqlReply, service::Service}, 
    tcp::tcp_client_connect::TcpClientConnect, 
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
    fn readQueue(selfId: &str, recv: &Receiver<PointType>, buffer: &mut RetainBuffer<PointType>) {
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
    fn readAll(selfId: &str, stream: &mut TcpStream) -> ConnectionStatus<Vec<u8>, String> {
        let mut buf = [0; Self::BUF_LEN];
        let mut result = vec![];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    debug!("{}.readAll |     read len: {:?}", selfId, len);
                    result.append(& mut buf[..len].into());
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return ConnectionStatus::Closed(format!("{}.readAll | tcp stream closed", selfId));
                        } else {
                            return ConnectionStatus::Active(result)
                        }
                    }
                },
                Err(err) => {
                    warn!("{}.readAll | error reading from socket: {:?}", selfId, err);
                    warn!("{}.readAll | error kind: {:?}", selfId, err.kind());
                    let status = ConnectionStatus::Closed(format!("{}.readAll | tcp stream error: {:?}", selfId, err));
                    return match err.kind() {
                        std::io::ErrorKind::NotFound => status,
                        std::io::ErrorKind::PermissionDenied => status,
                        std::io::ErrorKind::ConnectionRefused => status,
                        std::io::ErrorKind::ConnectionReset => status,
                        // std::io::ErrorKind::HostUnreachable => status,
                        // std::io::ErrorKind::NetworkUnreachable => status,
                        std::io::ErrorKind::ConnectionAborted => status,
                        std::io::ErrorKind::NotConnected => status,
                        std::io::ErrorKind::AddrInUse => status,
                        std::io::ErrorKind::AddrNotAvailable => status,
                        // std::io::ErrorKind::NetworkDown => status,
                        std::io::ErrorKind::BrokenPipe => status,
                        std::io::ErrorKind::AlreadyExists => status,
                        std::io::ErrorKind::WouldBlock => status,
                        // std::io::ErrorKind::NotADirectory => todo!(),
                        // std::io::ErrorKind::IsADirectory => todo!(),
                        // std::io::ErrorKind::DirectoryNotEmpty => todo!(),
                        // std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
                        // std::io::ErrorKind::FilesystemLoop => todo!(),
                        // std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
                        std::io::ErrorKind::InvalidInput => status,
                        std::io::ErrorKind::InvalidData => status,
                        std::io::ErrorKind::TimedOut => status,
                        std::io::ErrorKind::WriteZero => status,
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
                        std::io::ErrorKind::Interrupted => status,
                        std::io::ErrorKind::Unsupported => status,
                        std::io::ErrorKind::UnexpectedEof => status,
                        std::io::ErrorKind::OutOfMemory => status,
                        std::io::ErrorKind::Other => status,
                        _ => status,
                    }
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
    fn getLink(&self, name: &str) -> Sender<PointType> {
        match self.send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// 
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
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
        let _handle = thread::Builder::new().name(format!("{} - main", selfId)).spawn(move || {
            let mut buffer = RetainBuffer::new(&selfId, "", Some(conf.recvQueueMaxLength as usize));
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut connect = TcpClientConnect::new(selfId.clone() + "/TcpSocketClientConnect", conf.address, reconnect);
            let mut connectionClosed = false;
            'main: loop {
                match connect.connect() {
                    Some(mut stream) => {
                        connectionClosed = false;
                        match stream.set_read_timeout(Some(Duration::from_secs(10))) {
                            Ok(_) => {},
                            Err(err) => {
                                debug!("{}.run | TcpStream.set_timeout error: {:?}", selfId, err);
                            },
                        };
                        'send: loop {
                            cycle.start();
                            trace!("{}.run | step...", selfId);
                            Self::readQueue(&selfId, &recv, &mut buffer);
                            let mut count = buffer.len();
                            while count > 0 {
                                match buffer.first() {
                                    Some(point) => {
                                        let sql = point.asString().value;
                                        match Self::send(&selfId, sql, &mut stream) {
                                            Ok(_) => {
                                                match Self::readAll(&selfId, &mut stream) {
                                                    ConnectionStatus::Active(bytes) => {
                                                        let reply = String::from_utf8(bytes).unwrap();
                                                        debug!("{}.run | API reply: {:?}", selfId, reply);
                                                        let reply: SqlReply = serde_json::from_str(&reply).unwrap();
                                                        if reply.hasError() {
                                                            warn!("{}.run | API reply has error: {:?}", selfId, reply.error);
                                                        } else {
                                                            buffer.popFirst();
                                                        }
                                                    },
                                                    ConnectionStatus::Closed(err) => {
                                                        connectionClosed = true;
                                                        warn!("{}.run | API read error: {:?}", selfId, err);
                                                        break 'send;
                                                    },
                                                };
                                            },
                                            Err(err) => {
                                                connectionClosed = true;
                                                warn!("{}.run | API sending error: {:?}", selfId, err);
                                                break 'send;
                                            },
                                        }
                                    },
                                    None => {break;},
                                };
                                count -=1;
                            }
                            if exit.load(Ordering::SeqCst) | connectionClosed {
                                break 'main;
                            }
                            trace!("{}.run | step - done ({:?})", selfId, cycle.elapsed());
                            if cyclic {
                                cycle.wait();
                            }
                        };
                    },
                    None => {
                        debug!("{}.run | Not connection", selfId);
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                thread::sleep(Duration::from_millis(100));
            };
            info!("{}.run | stopped", selfId);
        });
        info!("{}.run | started", self.id);
        _handle
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
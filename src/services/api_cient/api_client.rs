use log::{info, debug, trace, warn};
use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, thread::{self, JoinHandle}, collections::HashMap, net::TcpStream, io::{Write, Read}};
use api_tools::{client::api_query::ApiQuery, reply::api_reply::SqlReply};
use crate::{
    core_::{point::point_type::PointType, net::connection_status::ConnectionStatus, retain_buffer::retain_buffer::RetainBuffer}, 
    conf::api_client_config::ApiClientConfig,
    services::{task::service_cycle::ServiceCycle, service::service::Service}, 
    tcp::tcp_client_connect::TcpClientConnect, 
};

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
            send: HashMap::from([(conf.rx.clone(), send)]),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Reads all avalible at the moment items from the in-queue
    fn read_queue(self_id: &str, recv: &Receiver<PointType>, buffer: &mut RetainBuffer<PointType>) {
        let max_read_at_once = 1000;
        for (index, point) in recv.try_iter().enumerate() {   
            debug!("{}.readQueue | point: {:?}", self_id, &point);
            buffer.push(point);
            if index > max_read_at_once {
                break;
            }                 
        }
    }
    ///
    /// Writing sql string to the TcpStream
    fn send(self_id: &str, sql: String, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>>{
        let query = ApiQuery::new("authToken", "id", "database", sql, true, true);
        match stream.write(query.toJson().as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("{}.send | write to tcp stream error: {:?}", self_id, err);
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
    fn read_all(self_id: &str, stream: &mut TcpStream) -> ConnectionStatus<Vec<u8>, String> {
        let mut buf = [0; Self::BUF_LEN];
        let mut result = vec![];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    debug!("{}.readAll |     read len: {:?}", self_id, len);
                    result.append(& mut buf[..len].into());
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return ConnectionStatus::Closed(format!("{}.readAll | tcp stream closed", self_id));
                        } else {
                            return ConnectionStatus::Active(result)
                        }
                    }
                },
                Err(err) => {
                    warn!("{}.readAll | error reading from socket: {:?}", self_id, err);
                    warn!("{}.readAll | error kind: {:?}", self_id, err.kind());
                    let status = ConnectionStatus::Closed(format!("{}.readAll | tcp stream error: {:?}", self_id, err));
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
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    // 
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let recv = self.recv.pop().unwrap();
        let (cyclic, cycle_interval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let reconnect = if conf.reconnectCycle.is_some() {conf.reconnectCycle.unwrap()} else {Duration::from_secs(3)};
        let _queue_max_length = conf.rxMaxLength;
        let _handle = thread::Builder::new().name(format!("{} - main", self_id)).spawn(move || {
            let mut buffer = RetainBuffer::new(&self_id, "", Some(conf.rxMaxLength as usize));
            let mut cycle = ServiceCycle::new(cycle_interval);
            let mut connect = TcpClientConnect::new(self_id.clone() + "/TcpSocketClientConnect", conf.address, reconnect);
            'main: loop {
                match connect.connect() {
                    Some(mut stream) => {
                        match stream.set_read_timeout(Some(Duration::from_secs(10))) {
                            Ok(_) => {},
                            Err(err) => {
                                debug!("{}.run | TcpStream.set_timeout error: {:?}", self_id, err);
                            },
                        };
                        'send: loop {
                            cycle.start();
                            trace!("{}.run | step...", self_id);
                            Self::read_queue(&self_id, &recv, &mut buffer);
                            let mut count = buffer.len();
                            while count > 0 {
                                match buffer.first() {
                                    Some(point) => {
                                        let sql = point.as_string().value;
                                        match Self::send(&self_id, sql, &mut stream) {
                                            Ok(_) => {
                                                match Self::read_all(&self_id, &mut stream) {
                                                    ConnectionStatus::Active(bytes) => {
                                                        let reply = String::from_utf8(bytes).unwrap();
                                                        debug!("{}.run | API reply: {:?}", self_id, reply);
                                                        let reply: SqlReply = serde_json::from_str(&reply).unwrap();
                                                        if reply.hasError() {
                                                            warn!("{}.run | API reply has error: {:?}", self_id, reply.error);
                                                        } else {
                                                            buffer.popFirst();
                                                        }
                                                    },
                                                    ConnectionStatus::Closed(err) => {
                                                        warn!("{}.run | API read error: {:?}", self_id, err);
                                                        break 'send;
                                                    },
                                                };
                                            },
                                            Err(err) => {
                                                warn!("{}.run | API sending error: {:?}", self_id, err);
                                                break 'send;
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
                            trace!("{}.run | step - done ({:?})", self_id, cycle.elapsed());
                            if cyclic {
                                cycle.wait();
                            }
                        };
                    },
                    None => {
                        debug!("{}.run | Not connection", self_id);
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                thread::sleep(Duration::from_millis(100));
            };
            info!("{}.run | stopped", self_id);
        });
        info!("{}.run | started", self.id);
        _handle
    }
    //
    // 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
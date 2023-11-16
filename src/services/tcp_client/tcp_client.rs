#![allow(non_snake_case)]

use std::{sync::{mpsc::{Sender, Receiver, self}, Arc, atomic::{AtomicBool, Ordering}, Mutex}, time::Duration, collections::HashMap, thread, net::TcpStream, io::{Read, Write}};

use log::{info, debug, trace, warn, error};

use crate::{
    core_::{point::{point_type::PointType, point::Point}, net::connection_status::ConnectionStatus, retain_buffer::retain_buffer::RetainBuffer},
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
    exitRW: Arc<AtomicBool>,
}
///
/// 
impl TcpClient {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: TcpClientConfig) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/TcpClient({})", parent.into(), conf.name),
            recv: vec![recv],
            send: HashMap::from([(conf.recvQueue.clone(), send)]),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
            exitRW: Arc::new(AtomicBool::new(false)),
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
    fn send(selfId: &str, point: &PointType, stream: &mut TcpStream, isConnected: &mut bool) -> Result<(), String>{
        match point.toJsonBytes() {
            Ok(bytes) => {
                match stream.write(&bytes) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        *isConnected = false;
                        let message= format!("{}.send | write to tcp stream error: {:?}", selfId, err);
                        warn!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => {
                let message= format!("{}.send | error: {:?}", selfId, err);
                warn!("{}", message);
                Err(message)
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
impl Service for TcpClient {
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
        let exitRW = self.exitRW.clone();
        // let exitW = self.exitRW.clone();
        let conf = self.conf.clone();
        let send = self.send.get(&conf.sendQueue).unwrap().clone();
        let send = Arc::new(Mutex::new(send));
        let recv = Arc::new(Mutex::new(self.recv.pop().unwrap()));
        let (cyclic, cycleInterval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let reconnect = if conf.reconnectCycle.is_some() {conf.reconnectCycle.unwrap()} else {Duration::from_secs(3)};
        let _queueMaxLength = conf.recvQueueMaxLength;
        let _h = thread::Builder::new().name(format!("{} - main", selfId)).spawn(move || {
            let mut isConnected = false;
            let buffer = Arc::new(Mutex::new(RetainBuffer::new(&selfId, "", Some(conf.recvQueueMaxLength as usize))));
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut connect = TcpSocketClientConnect::new(selfId.clone() + "/TcpSocketClientConnect", conf.address);
            let mut stream = None;
            'main: loop {
                if !isConnected {
                    stream = connect.connect(reconnect);
                    match stream {
                        Some(mut stream) => {
                            match stream.set_read_timeout(Some(Duration::from_secs(10))) {
                                Ok(_) => {},
                                Err(err) => {
                                    debug!("{}.run | TcpStream.set_timeout error: {:?}", selfId, err);
                                },                            
                            };
                            isConnected = true;
                            let selfIdR = selfId.clone();
                            let exitR = exitRW.clone();
                            let send = send.clone();
                            let mut streamR = stream.try_clone().unwrap();
                            let _hR = thread::Builder::new().name(format!("{} - Read", selfIdR.clone())).spawn(move || {
                                let send = send.lock().unwrap();
                                'read: loop {
                                    match Self::readAll(&selfIdR, &mut streamR) {
                                        ConnectionStatus::Active(bytes) => {
                                            match PointType::fromJsonBytes(bytes) {
                                                Ok(point) => {
                                                    match send.send(point) {
                                                        Ok(_) => {},
                                                        Err(err) => {
                                                            warn!("{}.send | write to tcp stream error: {:?}", selfIdR, err);
                                                        },
                                                    };
                                                },
                                                Err(err) => {
                                                    warn!("{}.run | Point prsing from json error: {:?}", selfIdR, err);
                                                },
                                            }
                                        },
                                        ConnectionStatus::Closed => {
                                            isConnected = false;
                                            exitR.store(true, Ordering::SeqCst);
                                            break;
                                        },
                                    };
                                    if exitR.load(Ordering::SeqCst) {
                                        break 'read;
                                    }
                                }
                            }).unwrap();
                            let selfIdW = selfId.clone();
                            let buffer = buffer.clone();
                            let exitW = exitRW.clone();
                            let recv = recv.clone();
                            let _hW = thread::Builder::new().name(format!("{} - Write", selfIdW.clone())).spawn(move || {
                                let mut buffer = buffer.lock().unwrap();
                                let recv = recv.lock().unwrap();
                                'write: loop {
                                    Self::readQueue(&selfIdW, &recv, &mut buffer);
                                    let mut count = buffer.len();
                                    while count > 0 {
                                        match buffer.first() {
                                            Some(point) => {
                                                match Self::send(&selfIdW, point, &mut stream, &mut isConnected) {
                                                    Ok(_) => {
                                                        buffer.remove(0);
                                                    },
                                                    Err(err) => {
                                                        
                                                    },
                                                }
                                            },
                                            None => {break;},
                                        };
                                        count -=1;
                                    }
                                    if exitW.load(Ordering::SeqCst) {
                                        break 'write;
                                    }
                                }
                            }).unwrap();
                            _hR.join().unwrap();
                            _hW.join().unwrap();
                        },
                        None => {
                            isConnected = false;
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
        // h.join().unwrap();
    }
    ///
    /// 
    fn exit(&self) {
        self.exitRW.store(true, Ordering::SeqCst);
        self.exit.store(true, Ordering::SeqCst);
    }
}

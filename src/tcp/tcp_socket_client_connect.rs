#![allow(non_snake_case)]

use std::{net::{TcpStream, SocketAddr, ToSocketAddrs}, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering, AtomicUsize}, Mutex, mpsc::{Sender, Receiver, self}, Once}, thread};

use log::{warn, LevelFilter, debug, info};

use crate::services::task::task_cycle::ServiceCycle;


#[derive(Debug, PartialEq)]
enum ConnectState {
    Closed,
    Connecting,
    Connected,
}
impl ConnectState {
    fn from(value: usize) -> Self {
        match value {
            0 => ConnectState::Closed,
            1 => ConnectState::Connecting,
            2 => ConnectState::Connected,
            _ => panic!("Invalid value: '{}'", value)
        }
    }
    fn value(&self) -> usize {
        match self {
            ConnectState::Closed => 0,
            ConnectState::Connecting => 1,
            ConnectState::Connected => 2,
        }
    }
}



///
/// Opens a TCP connection to a remote host
/// - returns connected Result<TcpStream, Err>
pub struct TcpClientConnect {
    id: String,
    addr: SocketAddr,
    state: Arc<AtomicUsize>,
    stream: Arc<Mutex<Vec<TcpStream>>>,
    reconnect: Duration,
    exitSend: Sender<bool>,
    exitRecv: Arc<Mutex<Receiver<bool>>>,
    once: Once,
}
///
/// Opens a TCP connection to a remote host
impl TcpClientConnect {
    ///
    /// Creates a new instance of TcpClientConnect
    pub fn new(parent: impl Into<String>, addr: impl ToSocketAddrs + std::fmt::Debug, reconnect: Duration) -> TcpClientConnect {
        let addr = match addr.to_socket_addrs() {
            Ok(mut addrIter) => {
                match addrIter.next() {
                    Some(addr) => addr,
                    None => panic!("TcpClientConnect({}).connect | Empty address found: {:?}", parent.into(), addr),
                }
            },
            Err(err) => panic!("TcpClientConnect({}).connect | Address parsing error: \n\t{:?}", parent.into(), err),
        };
        let (send, recv) = mpsc::channel();
        Self {
            id: format!("{}/TcpClientConnect", parent.into()),
            addr,
            state: Arc::new(AtomicUsize::new(ConnectState::Closed.value())),
            stream: Arc::new(Mutex::new(Vec::new())),
            reconnect,
            exitSend: send,
            exitRecv: Arc::new(Mutex::new(recv)),
            once: Once::new(),
        }
    }
    ///
    /// Opens a TCP connection to a remote host until succeed.
    pub fn connect(&mut self, closed: bool) -> Result<TcpStream, String> {
        info!("TcpClientConnect({}).connect | state: {:?}...", self.id, ConnectState::from( self.state.load(Ordering::Relaxed) ));
        match ConnectState::from( self.state.load(Ordering::Relaxed) ) {
            ConnectState::Closed => {
                if self.once.is_completed() {
                    self.once = Once::new();
                }
                self.inner_connect(self.reconnect);
            },
            ConnectState::Connected => {
                if closed {
                    if self.once.is_completed() {
                        self.once = Once::new();
                    }
                    self.state.store(ConnectState::Closed.value(), Ordering::SeqCst);
                    self.inner_connect(self.reconnect);
                }
            },
            _ => {},
        };
        match ConnectState::from( self.state.load(Ordering::Relaxed) ) {
            ConnectState::Connected => { 
                let stream = self.stream.lock().unwrap();                   
                let stream = stream.first().unwrap();
                match stream.try_clone() {
                    Ok(stream) => {
                        Ok(stream)
                    },
                    Err(err) => {
                        let message = format!("TcpClientConnect({}).connect | TcpSream.try_clone error: {:?}", self.id, err);
                        warn!("{}", message);
                        Err(message)
                    },
                }
            },
            _ => {
                let state = ConnectState::from( self.state.load(Ordering::Relaxed) );
                let message = format!("TcpClientConnect({}).connect | State: {:?}", self.id, state);
                debug!("{}", message);
                Err(message)
            },
        }
    }
    /// 
    /// Opens a TCP connection to a remote host until succeed.
    fn inner_connect(&mut self, cycle: Duration) {
        self.once.call_once(|| {
            self.state.store(ConnectState::Connecting.value(), Ordering::SeqCst);
            let id = self.id.clone();
            let addr = self.addr.clone();
            info!("TcpClientConnect({}).inner_connect | connecting to: {:?}...", id, addr);
            let state = self.state.clone();
            let selfStream = self.stream.clone();
            let exit = self.exitRecv.clone();
            let _handle = thread::spawn(move || {
                let exit = exit.lock().unwrap();
                let mut cycle = ServiceCycle::new(cycle);
                loop {
                    cycle.start();
                    if ConnectState::from( state.load(Ordering::Relaxed) ) != ConnectState::Connected {
                        match TcpStream::connect(addr) {
                            Ok(tcpStream) => {
                                selfStream.lock().unwrap().push(tcpStream);
                                state.store(ConnectState::Connected.value(), Ordering::SeqCst);
                                info!("TcpClientConnect({}).inner_connect | connected to: \n\t{:?}", id, selfStream.lock().unwrap().first().unwrap());
                                break;
                            },
                            Err(err) => {
                                state.store(ConnectState::Closed.value(), Ordering::SeqCst);
                                if log::max_level() == LevelFilter::Debug {
                                    warn!("TcpClientConnect({}).inner_connect | connection error: \n\t{:?}", id, err);
                                }
                            }
                        };
                    }
                    match exit.try_recv() {
                        Ok(exit) => {
                            debug!("TcpClientConnect({}).inner_connect | exit: {}", id, exit);
                            if exit {
                                match ConnectState::from( state.load(Ordering::Relaxed) ) {
                                    ConnectState::Connecting => state.store(ConnectState::Closed.value(), Ordering::SeqCst),
                                    _ => {},
                                };
                                break;
                            }
                        },
                        Err(_) => {},
                    }
                    cycle.wait();
                }
                debug!("TcpClientConnect({}).inner_connect | exit", id);
            });
        });
        // if ConnectState::from( self.state.load(Ordering::Relaxed) ) == ConnectState::Closed {
        //     // handle.join().unwrap();
        // }
    }
    // ///
    // /// Opens a TCP connection to a remote host
    // /// - tries to connect [attempts] times
    // pub fn connect_attempts(&mut self, attempts: u8) -> Result<TcpStream, std::io::Error> {
    //     let exit = self.exitRecv.clone();
    //     let exit = exit.lock().unwrap();
    //     let mut result: Option<Result<TcpStream, std::io::Error>> = None;
    //     for attempt in 0..=attempts {
    //         let r = TcpStream::connect(self.addr);
    //         match r {
    //             Ok(_) => {
    //                 result = Some(r)
    //             },
    //             Err(err) => {
    //                 if log::max_level() == LevelFilter::Trace {
    //                     warn!("TcpClientConnect({}).connect_attempts | attempt {}/{} connection error: \n\t{:?}", attempt, attempts, self.id, err);
    //                 }
    //                 result = Some(Err(err));
    //             }
    //         }
    //         match exit.try_recv() {
    //             Ok(exit) => {
    //                 debug!("TcpClientConnect({}).connect | exit: {}", self.id, exit);
    //                 if exit {
    //                     result = None;
    //                     break;
    //                 }
    //             },
    //             Err(_) => {},
    //         }
    //     }
    //     result.unwrap()
    // }
    ///
    /// Opens a TCP connection to a remote host with a timeout.
    pub fn connect_timeout(&self, timeout: Duration) -> Result<TcpStream, std::io::Error> {
        TcpStream::connect_timeout(&self.addr, timeout)
    }
    ///
    /// Exit thread
    pub fn exit(&self) -> Sender<bool> {
        self.exitSend.clone()
    }
}
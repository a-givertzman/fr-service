#![allow(non_snake_case)]

use std::{net::{TcpStream, SocketAddr, ToSocketAddrs}, time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}, Mutex, mpsc::{Receiver, Sender, self}}, thread};

use log::{warn, LevelFilter, debug};

use crate::services::task::task_cycle::ServiceCycle;

///
/// Opens a TCP connection to a remote host
/// - returns connected Result<TcpStream, Err>
pub struct TcpSocketClientConnect {
    id: String,
    addr: SocketAddr,
    exitRecv: Vec<Receiver<bool>>,
    exitSend: Sender<bool>,
    // exit: Arc<AtomicBool>,
}
///
/// Opens a TCP connection to a remote host
impl TcpSocketClientConnect {
    ///
    /// Creates a new instance of TcpSocketClientConnect
    pub fn new(id: impl Into<String>, addr: &str) -> TcpSocketClientConnect {
        let (send, recv) = mpsc::channel();
        let addr = match addr.to_socket_addrs() {
            Ok(mut addrIter) => {
                match addrIter.next() {
                    Some(addr) => addr,
                    None => panic!("TcpSocketClientConnect({}).connect | Empty address found: {}", id.into(), addr),
                }
            },
            Err(err) => panic!("TcpSocketClientConnect({}).connect | Address parsing error: \n\t{:?}", id.into(), err),
        };
        Self { 
            id: id.into(), 
            addr,
            exitRecv: vec![recv],
            exitSend: send,
            // exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Opens a TCP connection to a remote host until succeed.
    pub fn connect(&mut self, cycle: u64) -> Option<TcpStream> {
        let id = self.id.clone();
        let addr = self.addr.clone();
        let exit = self.exitRecv.pop().unwrap();
        let result: Arc<Mutex<Vec<Option<TcpStream>>>> = Arc::new(Mutex::new(vec![]));
        let resultRef = result.clone();
        let h = thread::spawn(move || {
            let mut cycle = ServiceCycle::new(Duration::from_millis(cycle));
            let mut result = resultRef.lock().unwrap();
            'main: loop {
                cycle.start();
                match TcpStream::connect(addr) {
                    Ok(stream) => {
                        result.push(Some(stream));
                        break 'main;
                    },
                    Err(err) => {
                        if log::max_level() == LevelFilter::Debug {
                            warn!("TcpSocketClientConnect({}).connect | connection error: \n\t{:?}", id, err);
                        }
                    }
                };
                match exit.try_recv() {
                    Ok(exit) => {
                        debug!("TcpSocketClientConnect({}).connect | exit: {}", id, exit);
                        if exit {
                            result.push(None);
                            break 'main;
                        }
                    },
                    Err(_) => {},
                }
                cycle.wait();
            }
            debug!("TcpSocketClientConnect({}).connect | exit", id);
        });
        h.join().unwrap();
        let tcpStream = result.lock().unwrap().pop().unwrap();
        tcpStream
    }
    ///
    /// Opens a TCP connection to a remote host
    /// - tries to connect [attempts] times
    pub fn connect_attempts(&mut self, attempts: u8) -> Result<TcpStream, std::io::Error> {
        let exit = self.exitRecv.pop().unwrap();
        let mut result: Option<Result<TcpStream, std::io::Error>> = None;
        for attempt in 0..=attempts {
            let r = TcpStream::connect(self.addr);
            match r {
                Ok(_) => {
                    result = Some(r)
                },
                Err(err) => {
                    if log::max_level() == LevelFilter::Trace {
                        warn!("TcpSocketClientConnect({}).connect_attempts | attempt {}/{} connection error: \n\t{:?}", attempt, attempts, self.id, err);
                    }
                    result = Some(Err(err));
                }
            }
            match exit.try_recv() {
                Ok(exit) => {
                    debug!("TcpSocketClientConnect({}).connect | exit: {}", self.id, exit);
                    if exit {
                        result = None;
                        break;
                    }
                },
                Err(_) => {},
            }
        }
        result.unwrap()
    }
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
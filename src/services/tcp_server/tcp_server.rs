#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{self, JoinHandle}, net::{TcpListener, TcpStream}, io::Read};

use log::{info, warn, debug};

use crate::{
    services::{services::Services, service::Service}, 
    conf::tcp_server_config::TcpServerConfig, core_::point::point_type::PointType,
};


///
/// Bounds TCP socket server
/// Listening socket for incoming connections
/// Verified incoming connections handles in the separate thread
pub struct TcpServer {
    id: String,
    conf: TcpServerConfig,
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
            id: format!("{}/TcpClient({})", parent.into(), conf.name),
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Setup thread for incomming connection
    fn setupConnection(selfId: String, mut tcpStream: TcpStream, exit: Arc<AtomicBool>) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.setupConnection | starting...", selfId);
        let selfIdClone = selfId.clone();
        info!("{}.setupConnection | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.setupConnection", selfId.clone())).spawn(move || {
            loop {
                let mut bytes = vec![];
                match tcpStream.read(&mut bytes) {
                    Ok(len) => {
                        debug!("{}.setupConnection | received {} bytes", selfId, len);
                    },
                    Err(err) => {
                        debug!("{}.setupConnection | error: {:?}", selfId, err);
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        info!("{}.setupConnection | started", selfIdClone);
        handle
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
    fn getLink(&mut self, name: &str) -> Sender<PointType> {
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
        info!("{}.run | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            let mut handles = vec![];
            loop {
                match TcpListener::bind(conf.address) {
                    Ok(listener) => {
                        for stream in listener.incoming() {
                            match stream {
                                Ok(stream) => {
                                    match Self::setupConnection(selfId.clone(), stream, exit.clone()) {
                                        Ok(handle) => {
                                            handles.push(handle);
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
                    break;
                }
            }
        });
        info!("{}.run | started", self.id);
        handle
    }
    ///
    /// 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}
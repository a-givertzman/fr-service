#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{self, JoinHandle}, net::{TcpListener, TcpStream, Shutdown, SocketAddr}, io::Read, time::Duration, rc::Rc, any::Any};

use log::{info, warn, debug, error};

use crate::{
    services::{services::Services, service::Service, task::task_cycle::ServiceCycle, queue_name::QueueName}, 
    conf::tcp_server_config::TcpServerConfig, core_::{point::{point_type::PointType, point_tx_id::PointTxId}, net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}}, tcp::{tcp_read_alive::TcpReadAlive, tcp_write_alive::TcpWriteAlive, tcp_stream_write::TcpStreamWrite},
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
    fn setupConnection(selfId: String, tcpStream: TcpStream, services: Arc<Mutex<Services>>, conf: TcpServerConfig, exit: Arc<AtomicBool>) -> Result<JoinHandle<()>, std::io::Error> {
        let remIp = match tcpStream.peer_addr() {
            Ok(addr) => {addr.to_string()},
            Err(err) => {
                warn!("{}.setupConnection | tcpStream.peer_addr error: {:?}", selfId, err);
                "Uncnown remote IP".to_string()
            },
        };
        let selfId = format!("{}({})", selfId, remIp);
        info!("{}.setupConnection | starting...", selfId);
        let selfIdClone = selfId.clone();
        let selfConfTx = conf.tx.clone();
        let txQueueName = QueueName::new(&selfConfTx);
        info!("{}.setupConnection | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.setupConnection", selfId.clone())).spawn(move || {
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
            let hR = tcpReadAlive.run(tcpStream.try_clone().unwrap());
            let hW = tcpWriteAlive.run(tcpStream);
            hR.join().unwrap();
            hW.join().unwrap();
        });
        info!("{}.setupConnection | started", selfIdClone);
        handle
    }
    ///
    /// 
    fn waitForThread(thd: JoinHandle<()>) -> Result<(), Box<dyn Any + Send>>{
        let thdId = format!("{:?}-{:?}", thd.thread().id(), thd.thread().name());
        info!("Waiting for thread: {:?}...", thdId);
        let r = thd.join();
        match &r {
            Ok(_) => {
                info!("Waiting for thread: '{}' - finished", thdId);
            },
            Err(err) => {
                error!("Waiting for thread '{}' error: {:?}", thdId, err);                
            },
        }
        r
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
        let services = self.services.clone();
        let reconnectCycle = conf.reconnectCycle.unwrap_or(Duration::ZERO);
        info!("{}.run | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            let mut cycle = ServiceCycle::new(reconnectCycle);
            let mut handles: Vec<JoinHandle<()>> = vec![];
            loop {
                cycle.start();
                match TcpListener::bind(conf.address) {
                    Ok(listener) => {
                        for stream in listener.incoming() {
                            if exit.load(Ordering::SeqCst) {
                                while handles.len() > 0 {
                                    Self::waitForThread(handles.pop().unwrap()).unwrap();
                                } 
                                break;
                            }
                            match stream {
                                Ok(stream) => {
                                    match Self::setupConnection(selfId.clone(), stream, services.clone(), conf.clone(), exit.clone()) {
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
                    for handle in handles {
                        Self::waitForThread(handle).unwrap();
                    }
                    break;
                }
                cycle.wait();
                if exit.load(Ordering::SeqCst) {
                    for handle in handles {
                        Self::waitForThread(handle).unwrap();
                    }
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
        thread::sleep(Duration::from_millis(10));
        let s = TcpStream::connect_timeout(&self.conf.address, Duration::from_millis(100)).unwrap();
        s.shutdown(Shutdown::Both).unwrap();
    }    
}
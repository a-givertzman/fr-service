#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{JoinHandle, self}, time::Duration, net::TcpStream, io::BufReader};

use log::{warn, info, LevelFilter};

use crate::{core_::{
    net::{connection_status::ConnectionStatus, protocols::jds::{jds_deserialize::JdsDeserialize, jds_decode_message::JdsDecodeMessage}}, 
    point::point_type::PointType,
}, services::task::service_cycle::ServiceCycle};


pub struct TcpReadAlive {
    id: String,
    jdsStream: Arc<Mutex<JdsDeserialize>>,
    send: Sender<PointType>,
    cycle: Duration,
    exit: Arc<AtomicBool>,
}
impl TcpReadAlive {
    ///
    /// Creates new instance of [TcpReadAlive]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, send: Sender<PointType>, cycle: Duration, exit: Option<Arc<AtomicBool>>) -> Self {
        let selfId = format!("{}/TcpReadAlive", parent.into());
        Self {
            id: selfId.clone(),
            jdsStream: Arc::new(Mutex::new(JdsDeserialize::new(
                selfId.clone(),
                JdsDecodeMessage::new(
                    selfId,
                ),
            ))),
            send: send,
            cycle,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Main loop of the [TcpReadAlive]
    pub fn run(&mut self, tcpStream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let send = self.send.clone();
        let jdsStream = self.jdsStream.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Read", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let mut tcpStream = BufReader::new(tcpStream);
            let mut jdsStream = jdsStream.lock().unwrap();
            info!("{}.run | Main loop started", selfId);
            loop {
                cycle.start();
                match jdsStream.read(&mut tcpStream) {
                    ConnectionStatus::Active(point) => {
                        match point {
                            Ok(point) => {
                                match send.send(point) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        warn!("{}.run | write to queue error: {:?}", selfId, err);
                                    },
                                };
                            },
                            Err(err) => {
                                if log::max_level() == LevelFilter::Trace {
                                    warn!("{}.run | error: {:?}", selfId, err);
                                }
                            },
                        }
                    },
                    ConnectionStatus::Closed(err) => {
                        warn!("{}.run | error: {:?}", selfId, err);
                        exit.store(true, Ordering::SeqCst);
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                cycle.wait();
            }
            info!("{}.run | Exit", selfId);
        }).unwrap();
        info!("{}.run | started", self.id);
        handle
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}    
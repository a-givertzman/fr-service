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
    exitPair: Arc<AtomicBool>,
}
impl TcpReadAlive {
    ///
    /// Creates new instance of [TcpReadAlive]
    /// - [parent] - the ID if the parent entity
    /// - [exit] - notification from parent to exit 
    /// - [exitPair] - notification from / to sibling pair to exit 
    pub fn new(parent: impl Into<String>, send: Sender<PointType>, cycle: Duration, exit: Option<Arc<AtomicBool>>, exitPair: Option<Arc<AtomicBool>>) -> Self {
        let self_id = format!("{}/TcpReadAlive", parent.into());
        Self {
            id: self_id.clone(),
            jdsStream: Arc::new(Mutex::new(JdsDeserialize::new(
                self_id.clone(),
                JdsDecodeMessage::new(
                    self_id,
                ),
            ))),
            send: send,
            cycle,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exitPair: exitPair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Main loop of the [TcpReadAlive]
    pub fn run(&mut self, tcpStream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let exitPair = self.exitPair.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let send = self.send.clone();
        let jdsStream = self.jdsStream.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Read", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let mut tcpStream = BufReader::new(tcpStream);
            let mut jdsStream = jdsStream.lock().unwrap();
            info!("{}.run | Main loop started", self_id);
            loop {
                cycle.start();
                match jdsStream.read(&mut tcpStream) {
                    ConnectionStatus::Active(point) => {
                        match point {
                            Ok(point) => {
                                match send.send(point) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        warn!("{}.run | write to queue error: {:?}", self_id, err);
                                    },
                                };
                            },
                            Err(err) => {
                                if log::max_level() == LevelFilter::Trace {
                                    warn!("{}.run | error: {:?}", self_id, err);
                                }
                            },
                        }
                    },
                    ConnectionStatus::Closed(err) => {
                        warn!("{}.run | error: {:?}", self_id, err);
                        exitPair.store(true, Ordering::SeqCst);
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) | exitPair.load(Ordering::SeqCst) {
                    break;
                }
                cycle.wait();
            }
            info!("{}.run | Exit", self_id);
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
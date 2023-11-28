#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{JoinHandle, self}, time::Duration, net::TcpStream, io::BufReader};

use log::{warn, info};

use crate::{core_::{
    net::{connection_status::ConnectionStatus, protocols::jds::{jds_deserialize::JdsDeserialize, jds_decode_message::JdsDecodeMessage}}, 
    point::point_type::PointType,
}, services::task::task_cycle::ServiceCycle};


pub struct TcpReadAlive {
    id: String,
    jdsStream: Arc<Mutex<JdsDeserialize>>,
    send: Arc<Mutex<Sender<PointType>>>,
    exit: Arc<AtomicBool>,
}
impl TcpReadAlive {
    ///
    /// Creates new instance of [TcpReadAlive]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, send: Arc<Mutex<Sender<PointType>>>) -> Self {
        let selfId = format!("{}/TcpReadAlive", parent.into());
        Self {
            id: selfId.clone(),
            jdsStream: Arc::new(Mutex::new(JdsDeserialize::new(
                selfId.clone(),
                JdsDecodeMessage::new(
                    selfId,
                ),
            ))),
            send,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Main loop of the [TcpReadAlive]
    pub fn run(&self, tcpStream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let cycleInterval = Duration::from_millis(1000);
        let mut cycle = ServiceCycle::new(cycleInterval);
        let send = self.send.clone();
        let jdsStream = self.jdsStream.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Read", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let send = send.lock().unwrap();
            let mut tcpStream = BufReader::new(tcpStream);
            let mut jdsStream = jdsStream.lock().unwrap();
            info!("{}.run | Starting main loop...", selfId);
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
                                warn!("{}.run | error: {:?}", selfId, err);
                            },
                        }
                    },
                    ConnectionStatus::Closed(err) => {
                        warn!("{}.run | error: {:?}", selfId, err);
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                cycle.wait();
            }
        }).unwrap();
        handle
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}    
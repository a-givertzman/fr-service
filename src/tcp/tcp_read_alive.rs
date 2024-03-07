use std::{
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, 
    thread::{JoinHandle, self}, time::Duration, net::TcpStream, io::BufReader,
};
use log::{warn, info, LevelFilter};
use crate::{core_::{
    net::{connection_status::ConnectionStatus, protocols::jds::{jds_deserialize::JdsDeserialize, jds_decode_message::JdsDecodeMessage}}, 
    point::point_type::PointType,
}, services::task::service_cycle::ServiceCycle};

///
/// Transfering points from JdsStream (socket) to the Channel Sender<PointType>
pub struct TcpReadAlive {
    id: String,
    jds_stream: Arc<Mutex<JdsDeserialize>>,
    send: Sender<PointType>,
    cycle: Duration,
    exit: Arc<AtomicBool>,
    exit_pair: Arc<AtomicBool>,
}
impl TcpReadAlive {
    ///
    /// Creates new instance of [TcpReadAlive]
    /// - [parent] - the ID if the parent entity
    /// - [exit] - notification from parent to exit 
    /// - [exitPair] - notification from / to sibling pair to exit 
    pub fn new(parent: impl Into<String>, send: Sender<PointType>, cycle: Duration, exit: Option<Arc<AtomicBool>>, exit_pair: Option<Arc<AtomicBool>>) -> Self {
        let self_id = format!("{}/TcpReadAlive", parent.into());
        Self {
            id: self_id.clone(),
            jds_stream: Arc::new(Mutex::new(JdsDeserialize::new(
                self_id.clone(),
                JdsDecodeMessage::new(
                    self_id,
                ),
            ))),
            send: send,
            cycle,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exit_pair: exit_pair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Main loop of the [TcpReadAlive]
    pub fn run(&mut self, tcp_stream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let exit_pair = self.exit_pair.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let send = self.send.clone();
        let jds_stream = self.jds_stream.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Read", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let mut tcp_stream = BufReader::new(tcp_stream);
            let mut jds_stream = jds_stream.lock().unwrap();
            info!("{}.run | Main loop started", self_id);
            loop {
                cycle.start();
                match jds_stream.read(&mut tcp_stream) {
                    ConnectionStatus::Active(point) => {
                        match point {
                            Ok(point) => {
                                match point.cot() {
                                    crate::core_::cot::cot::Cot::Inf => todo!(),
                                    crate::core_::cot::cot::Cot::Act => todo!(),
                                    crate::core_::cot::cot::Cot::ActCon => todo!(),
                                    crate::core_::cot::cot::Cot::ActErr => todo!(),
                                    crate::core_::cot::cot::Cot::Req => todo!(),
                                    crate::core_::cot::cot::Cot::ReqCon => todo!(),
                                    crate::core_::cot::cot::Cot::ReqErr => todo!(),
                                    crate::core_::cot::cot::Cot::Read => todo!(),
                                    crate::core_::cot::cot::Cot::Write => todo!(),
                                    crate::core_::cot::cot::Cot::All => todo!(),
                                }
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
                        exit_pair.store(true, Ordering::SeqCst);
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) | exit_pair.load(Ordering::SeqCst) {
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
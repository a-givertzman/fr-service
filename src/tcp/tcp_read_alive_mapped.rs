use std::{
    collections::HashMap, hash::BuildHasherDefault, io::BufReader, net::TcpStream, sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc, Mutex, RwLock}, thread::{self, JoinHandle}, time::Duration
};
use hashers::fx_hash::FxHasher;
use log::{warn, info, LevelFilter};
use crate::{core_::{
    cot::cot::Cot, 
    net::{connection_status::ConnectionStatus, protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize}}, 
    point::point_type::PointType,
}, services::task::service_cycle::ServiceCycle};

///
/// Transfering points from JdsStream (socket) to the Channels Map<ReceiverId, Sender<PointType>>
pub struct TcpReadAliveMapped {
    id: String,
    jds_stream: Arc<Mutex<JdsDeserialize>>,
    receivers: Arc<RwLock<HashMap<Cot, Sender<PointType>, BuildHasherDefault<FxHasher>>>>,
    cycle: Duration,
    exit: Arc<AtomicBool>,
    exit_pair: Arc<AtomicBool>,
}
impl TcpReadAliveMapped {
    ///
    /// Creates new instance of [TcpReadAliveMapped]
    /// - [parent] - the ID if the parent entity
    /// - [exit] - notification from parent to exit 
    /// - [exitPair] - notification from / to sibling pair to exit 
    pub fn new(
        parent: impl Into<String>, 
        receivers: Arc<RwLock<HashMap<Cot, Sender<PointType>, BuildHasherDefault<FxHasher>>>>, 
        cycle: Duration, 
        exit: Option<Arc<AtomicBool>>, 
        exit_pair: Option<Arc<AtomicBool>>,
    ) -> Self {
        let self_id = format!("{}/TcpReadAliveMapped", parent.into());
        Self {
            id: self_id.clone(),
            jds_stream: Arc::new(Mutex::new(JdsDeserialize::new(
                self_id.clone(),
                JdsDecodeMessage::new(
                    self_id,
                ),
            ))),
            receivers,
            cycle,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exit_pair: exit_pair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Main loop of the [TcpReadAliveMapped]
    pub fn run(&mut self, tcp_stream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let exit_pair = self.exit_pair.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let receivers = self.receivers.clone();
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
                                let point_cot = point.cot();
                                match receivers.read().unwrap().get(&point_cot) {
                                    Some(receiver) => {
                                        match receiver.send(point) {
                                            Ok(_) => {},
                                            Err(err) => {
                                                warn!("{}.run | write to receiver by {:?}, error: {:?}", self_id, point_cot, err);
                                            },
                                        };
                                    },
                                    None => {
                                        if log::max_level() > log::LevelFilter::Info {
                                            warn!("{}.run | Point with cot {:?} - ignored", self_id, point.cot());
                                        }
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
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{JoinHandle, self}, net::TcpStream, time::Duration};
use log::{warn, info};
use crate::{
    core_::net::connection_status::ConnectionStatus,
    tcp::tcp_stream_write::TcpStreamWrite, services::task::service_cycle::ServiceCycle, 
};
///
/// Transfering points from Channel Sender<PointType> to the JdsStream (socket)
pub struct TcpWriteAlive {
    id: String,
    cycle: Duration,
    stream_write: Arc<Mutex<TcpStreamWrite>>,
    exit: Arc<AtomicBool>,
    exit_pair: Arc<AtomicBool>,
}
impl TcpWriteAlive {
    ///
    /// Creates new instance of [TcpWriteAlive]
    /// - [parent] - the ID if the parent entity
    /// - [exit] - notification from parent to exit 
    /// - [exitPair] - notification from / to sibling pair to exit 
    pub fn new(parent: impl Into<String>, cycle: Duration, stream_write: Arc<Mutex<TcpStreamWrite>>, exit: Option<Arc<AtomicBool>>, exit_pair: Option<Arc<AtomicBool>>) -> Self {
        Self {
            id: format!("{}/TcpWriteAlive", parent.into()),
            cycle,
            stream_write,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exit_pair: exit_pair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// 
    pub fn run(&self, mut tcp_stream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let exit_pair = self.exit_pair.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let stream_write = self.stream_write.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Write", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let mut stream_write = stream_write.lock().unwrap();
            info!("{}.run | Main loop started", self_id);
            'main: loop {
                cycle.start();
                match stream_write.write(&mut tcp_stream) {
                    ConnectionStatus::Active(result) => {
                        match result {
                            Ok(_) => {},
                            Err(err) => {
                                warn!("{}.run | error: {:?}", self_id, err);
                            },
                        }
                    },
                    ConnectionStatus::Closed(err) => {
                        warn!("{}.run | error: {:?}", self_id, err);
                        exit_pair.store(true, Ordering::SeqCst);
                        break 'main;
                    },
                };
                if exit.load(Ordering::SeqCst) | exit_pair.load(Ordering::SeqCst) {
                    break 'main;
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

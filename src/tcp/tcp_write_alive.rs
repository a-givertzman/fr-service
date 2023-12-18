#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{JoinHandle, self}, net::TcpStream, time::Duration};

use log::{warn, info};

use crate::{
    core_::net::connection_status::ConnectionStatus,
    tcp::tcp_stream_write::TcpStreamWrite, services::task::task_cycle::ServiceCycle, 
};


pub struct TcpWriteAlive {
    id: String,
    cycle: Duration,
    streamWrite: Arc<Mutex<TcpStreamWrite>>,
    exit: Arc<AtomicBool>,
}
impl TcpWriteAlive {
    ///
    /// Creates new instance of [TcpWriteAlive]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, cycle: Duration, streamWrite: Arc<Mutex<TcpStreamWrite>>, exit: Option<Arc<AtomicBool>>) -> Self {
        Self {
            id: format!("{}/TcpWriteAlive", parent.into()),
            cycle,
            streamWrite,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// 
    pub fn run(&self, mut tcpStream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let streamWrite = self.streamWrite.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Write", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let mut streamWrite = streamWrite.lock().unwrap();
            info!("{}.run | Starting main loop...", selfId);
            'main: loop {
                cycle.start();
                match streamWrite.write(&mut tcpStream) {
                    ConnectionStatus::Active(result) => {
                        match result {
                            Ok(_) => {},
                            Err(err) => {
                                warn!("{}.run | error: {:?}", selfId, err);
                            },
                        }
                    },
                    ConnectionStatus::Closed(err) => {
                        warn!("{}.run | error: {:?}", selfId, err);
                        break 'main;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                cycle.wait();
            }
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

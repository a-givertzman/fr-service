#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{JoinHandle, self}, net::TcpStream, time::Duration};

use log::{warn, info};

use crate::{
    core_::net::connection_status::ConnectionStatus,
    tcp::tcp_stream_write::TcpStreamWrite, services::task::service_cycle::ServiceCycle, 
};


pub struct TcpWriteAlive {
    id: String,
    cycle: Duration,
    streamWrite: Arc<Mutex<TcpStreamWrite>>,
    exit: Arc<AtomicBool>,
    exitPair: Arc<AtomicBool>,
}
impl TcpWriteAlive {
    ///
    /// Creates new instance of [TcpWriteAlive]
    /// - [parent] - the ID if the parent entity
    /// - [exit] - notification from parent to exit 
    /// - [exitPair] - notification from / to sibling pair to exit 
    pub fn new(parent: impl Into<String>, cycle: Duration, streamWrite: Arc<Mutex<TcpStreamWrite>>, exit: Option<Arc<AtomicBool>>, exitPair: Option<Arc<AtomicBool>>) -> Self {
        Self {
            id: format!("{}/TcpWriteAlive", parent.into()),
            cycle,
            streamWrite,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exitPair: exitPair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// 
    pub fn run(&self, mut tcpStream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let exitPair = self.exitPair.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let streamWrite = self.streamWrite.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Write", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let mut streamWrite = streamWrite.lock().unwrap();
            info!("{}.run | Main loop started", selfId);
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
                        exitPair.store(true, Ordering::SeqCst);
                        break 'main;
                    },
                };
                if exit.load(Ordering::SeqCst) | exitPair.load(Ordering::SeqCst) {
                    break 'main;
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

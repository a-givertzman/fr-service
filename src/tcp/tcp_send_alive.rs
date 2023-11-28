#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{JoinHandle, self}, time::Duration, net::TcpStream};

use log::{warn, info};

use crate::{
    core_::net::connection_status::ConnectionStatus,
    tcp::{tcp_client_connect::TcpClientConnect, tcp_stream_write::TcpStreamWrite}, 
};


pub struct TcpSendAlive {
    id: String,
    streamWrite: Arc<Mutex<TcpStreamWrite>>,
    exit: Arc<AtomicBool>,
}
impl TcpSendAlive {
    ///
    /// Creates new instance of [TcpSendAlive]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, streamWrite: Arc<Mutex<TcpStreamWrite>>) -> Self {
        Self {
            id: format!("{}/TcpSendAlive", parent.into()),
            streamWrite,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn run(&self, mut tcpStream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let streamWrite = self.streamWrite.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Write", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let mut streamWrite = streamWrite.lock().unwrap();
            info!("{}.run | Starting main loop...", selfId);
            'main: loop {
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
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                thread::sleep(Duration::from_millis(10));
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

#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{JoinHandle, self}, time::Duration};

use log::{warn, info};

use crate::{
    core_::net::connection_status::ConnectionStatus,
    tcp::{tcp_socket_client_connect::TcpSocketClientConnect, tcp_stream_write::TcpStreamWrite}, 
};


pub struct TcpSendAlive {
    id: String,
    socketClientConnect: Arc<Mutex<TcpSocketClientConnect>>,
    socketClientConnectExit: Sender<bool>,
    streamWrite: Arc<Mutex<TcpStreamWrite>>,
    exit: Arc<AtomicBool>,
}
impl TcpSendAlive {
    ///
    /// Creates new instance of [TcpSendAlive]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, socketClientConnect: Arc<Mutex<TcpSocketClientConnect>>, streamWrite: Arc<Mutex<TcpStreamWrite>>) -> Self {
        let socketClientConnectExit = socketClientConnect.lock().unwrap().exit();
        Self {
            id: format!("{}/TcpSendAlive", parent.into()),
            socketClientConnect,
            socketClientConnectExit,
            streamWrite,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn run(&self) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let connect = self.socketClientConnect.clone();
        let streamWrite = self.streamWrite.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Write", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let mut connectionClosed = false;
            let mut streamWrite = streamWrite.lock().unwrap();
            info!("{}.run | Starting main loop...", selfId);
            'main: loop {
                info!("{}.run | connect.try_lock()...", selfId);
                match connect.try_lock() {
                    Ok(mut connect) => {
                        match connect.connect(connectionClosed) {
                            Ok(mut tcpStream) => {
                                drop(connect);
                                info!("{}.run | connected: {:?}", selfId, tcpStream);
                                loop {
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
                                            connectionClosed = true;
                                            break;
                                        },
                                    };
                                    if exit.load(Ordering::SeqCst) {
                                        break;
                                    }
                                }
                            },
                            Err(err) => {
                                warn!("{}.run | error: {:?}", selfId, err);
                            },
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | connect.try_lock() error: {:?}", selfId, err);
                    },
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
        self.socketClientConnectExit.send(true).unwrap();
    }
}

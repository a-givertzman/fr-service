#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{JoinHandle, self}};

use log::warn;

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
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let connect = self.socketClientConnect.clone();
        let streamWrite = self.streamWrite.clone();
        let handle = thread::Builder::new().name(format!("{} - Write", selfId.clone())).spawn(move || {
            let mut connectionClosed = false;
            let mut streamWrite = streamWrite.lock().unwrap();
            'main: loop {
                match connect.lock().unwrap().connect(connectionClosed) {
                    Ok(mut tcpStream) => {
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
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
            }
        }).unwrap();
        handle
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        self.socketClientConnectExit.send(true).unwrap();
    }
}

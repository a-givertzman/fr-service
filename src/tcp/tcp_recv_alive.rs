#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread::{JoinHandle, self}, time::Duration};

use log::{warn, info};

use crate::{
    core_::{net::{connection_status::ConnectionStatus, protocols::jds::{jds_deserialize::JdsDeserialize, jds_decode_message::JdsDecodeMessage}}, point::point_type::PointType},
    tcp::tcp_socket_client_connect::TcpClientConnect, 
};


pub struct TcpRecvAlive {
    id: String,
    socketClientConnect: Arc<Mutex<TcpClientConnect>>,
    socketClientConnectExit: Sender<bool>,
    send: Arc<Mutex<Sender<PointType>>>,
    exit: Arc<AtomicBool>,
}
impl TcpRecvAlive {
    ///
    /// Creates new instance of [TcpRecvAlive]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, socketClientConnect: Arc<Mutex<TcpClientConnect>>, send: Arc<Mutex<Sender<PointType>>>) -> Self {
        let socketClientConnectExit = socketClientConnect.lock().unwrap().exit();
        Self {
            id: format!("{}/TcpRecvAlive", parent.into()),
            socketClientConnect,
            socketClientConnectExit,
            send,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Main loop of the [TcpRecvAlive]
    pub fn run(&self) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let connect = self.socketClientConnect.clone();
        let send = self.send.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Read", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            let mut connectionClosed = false;
            let send = send.lock().unwrap();
            info!("{}.run | Starting main loop...", selfId);
            'main: loop {
                info!("{}.run | connect.try_lock()...", selfId);
                match connect.try_lock() {
                    Ok(mut connect) => {
                        match connect.connect(connectionClosed) {
                            Ok(tcpStream) => {
                                drop(connect);
                                info!("{}.run | connected: {:?}", selfId, tcpStream);
                                let mut jdsStream = JdsDeserialize::new(
                                    selfId.clone(),
                                    JdsDecodeMessage::new(
                                        selfId.clone(),
                                        tcpStream,
                                    ),
                                );
                                loop {
                                    match jdsStream.read() {
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
        handle
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        self.socketClientConnectExit.send(true).unwrap();
    }
}    
#![allow(non_snake_case)]

use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{JoinHandle, self}, time::Duration, net::{TcpStream, SocketAddr}};

use log::{info, debug, trace, warn};

use crate::{
    core_::{
        testing::test_stuff::test_value::Value, point::{point_type::PointType, point_tx_id::PointTxId}, 
        net::{protocols::jds::{jds_deserialize::JdsDeserialize, jds_decode_message::JdsDecodeMessage}, connection_status::ConnectionStatus},
    },
    services::service::Service, 
};


///
/// Jast connects to the tcp socket on [address]
/// - all point from [testData] will be sent via socket
/// - all received point in the received() method
/// - if [recvLimit] is some then thread exit when riched recvLimit
pub struct EmulatedTcpClient {
    id: String,
    addr: SocketAddr,
    testData: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    received: Arc<Mutex<Vec<PointType>>>,
    recvLimit: Option<usize>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl EmulatedTcpClient {
    pub fn new(parent: impl Into<String>, addr: &str, testData: Vec<Value>, recvLimit: Option<usize>) -> Self {
        let selfId = format!("{}/EmulatedTcpClient", parent.into());
        Self {
            id: selfId.clone(),
            addr: addr.parse().unwrap(),
            testData,
            sent: Arc::new(Mutex::new(vec![])),
            received: Arc::new(Mutex::new(vec![])),
            recvLimit,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn id(&self) -> String {
        self.id.clone()
    }
    ///
    /// 
    pub fn sent(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.sent.clone()
    }
    ///
    /// 
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
    }
}
///
/// 
impl Service for EmulatedTcpClient {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn getLink(&mut self, _name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        panic!("{}.getLink | Does not support static producer", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }
    }
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let addr = self.addr.clone();
        let received = self.received.clone();
        let recvLimit = self.recvLimit.clone();
        let handle = thread::Builder::new().name(format!("{}.run Read", selfId)).spawn(move || {
            info!("{}.run | Preparing thread Read - ok", selfId);
            'connect: loop {
                match TcpStream::connect(addr) {
                    Ok(tcpStream) => {
                        info!("{}.run | connected on: {:?}", selfId, addr);
                        let mut jdsDeserialize = JdsDeserialize::new(
                            selfId.clone(),
                            JdsDecodeMessage::new(
                                selfId.clone(),
                            ),
                        );
                        match recvLimit {
                            Some(recvLimit) => {
                                let mut receivedCount = 0;
                                loop {
                                    match jdsDeserialize.read(&tcpStream) {
                                        ConnectionStatus::Active(result) => {
                                            trace!("{}.run | received: {:?}", selfId, result);
                                            match result {
                                                Ok(point) => {
                                                    debug!("{}.run | received: {:?}", selfId, point);
                                                    received.lock().unwrap().push(point);
                                                    receivedCount += 1;
                                                },
                                                Err(err) => {
                                                    warn!("{}.run | read socket error: {:?}", selfId, err);
                                                },
                                            }
                                        },
                                        ConnectionStatus::Closed(err) => {
                                            warn!("{}.run | socket connection closed: {:?}", selfId, err);
                                            break;
                                        },
                                    };
                                    if receivedCount >= recvLimit {
                                        break;
                                    }
                                    if exit.load(Ordering::SeqCst) {
                                        break;
                                    }
                                }
                            },
                            None => {
                                loop {
                                    match jdsDeserialize.read(&tcpStream) {
                                        ConnectionStatus::Active(result) => {
                                            trace!("{}.run | received: {:?}", selfId, result);
                                            match result {
                                                Ok(point) => {
                                                    received.lock().unwrap().push(point);
                                                },
                                                Err(err) => {
                                                    warn!("{}.run | read socket error: {:?}", selfId, err);
                                                },
                                            }
                                        },
                                        ConnectionStatus::Closed(err) => {
                                            warn!("{}.run | socket connection closed: {:?}", selfId, err);
                                            break;
                                        },
                                    };
                                    if exit.load(Ordering::SeqCst) {
                                        break;
                                    }
                                }
                            },
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | connection error: {:?}", selfId, err);
                        thread::sleep(Duration::from_millis(1000))
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break 'connect;
                }
            }
            info!("{}.run | Exit thread Recv", selfId);
        });
        info!("{}.run | starting - ok", self.id);
        handle
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

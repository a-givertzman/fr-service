#![allow(non_snake_case)]

use log::{info, warn, debug};
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc}, thread::{JoinHandle, self}, time::Duration, net::{TcpStream, SocketAddr}, io::Write};
use testing::entities::test_value::Value;
use crate::{
    core_::{
        net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, point::{point_tx_id::PointTxId, point_type::{PointType, ToPoint}}, state::{switch_state::{Switch, SwitchCondition, SwitchState}, switch_state_changed::SwitchStateChanged}
    },
    services::service::Service, tcp::steam_read::StreamRead, 
};


///
/// Jast connects to the tcp socket on [address]
/// - all point from [testData] will be sent via socket
/// - all received point in the received() method
/// - if [recvLimit] is some then thread exit when riched recvLimit
/// - [disconnect] - contains percentage (0..100) of testData / iterations, where socket will be disconnected and connected again
pub struct EmulatedTcpClientSend {
    id: String,
    addr: SocketAddr,
    testData: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    disconnect: Vec<i8>,
    waitOnFinish: bool,
    exit: Arc<AtomicBool>,
}
///
/// 
impl EmulatedTcpClientSend {
    pub fn new(parent: impl Into<String>, addr: &str, testData: Vec<Value>, disconnect: Vec<i8>, waitOnFinish: bool) -> Self {
        let selfId = format!("{}/EmulatedTcpClientSend", parent.into());
        Self {
            id: selfId.clone(),
            addr: addr.parse().unwrap(),
            testData,
            sent: Arc::new(Mutex::new(vec![])),
            disconnect,
            waitOnFinish,
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
    fn switchState<T: std::cmp::PartialOrd + Clone + 'static>(initial: u8, steps: Vec<T>, fin: T) -> SwitchStateChanged<u8, T> {
        fn switch<T: std::cmp::PartialOrd + Clone + 'static>(state: &mut u8, input: Option<T>) -> Switch<u8, T> {
            let state_ = *state;
            *state = *state + 1;
            let target = state;
            Switch{
                state: state_,
                conditions: vec![
                    SwitchCondition {
                        condition: Box::new(move |value| {
                            match input.clone() {
                                Some(input) => value >= input,
                                None => false,
                            }
                        }),
                        target: *target,        
                    },
                ],
            }
        }
        let mut state: u8 = initial;
        let mut switches: Vec<Switch<u8, T>> = steps.into_iter().map(|input| {switch(&mut state, Some(input))}).collect();
            let state_ = state;
            state = state + 1;
            let target = state;
        switches.push(
            Switch{
                state: state_,
                conditions: vec![
                    SwitchCondition {
                        condition: Box::new(move |value| { value == fin}),
                        target: target,        
                    },
                ],
            }
        );
        let switchState: SwitchStateChanged<u8, T> = SwitchStateChanged::new(
            SwitchState::new(
                initial,
                switches,
            ),
        );
        switchState
    }
}
///
/// 
impl Service for EmulatedTcpClientSend {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn get_link(&mut self, _name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
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
        let mut testData = self.testData.clone();
        let totalCount = testData.len();
        let sent = self.sent.clone();
        let disconnect = self.disconnect.iter().map(|v| {(*v as f32) / 100.0}).collect();
        let _waitOnFinish = self.waitOnFinish;
        let handle = thread::Builder::new().name(format!("{}.run Read", selfId)).spawn(move || {
            info!("{}.run | Preparing thread Read - ok", selfId);
            let mut switchState = Self::switchState(1, disconnect, 1.0);
            'connect: loop {
                match TcpStream::connect(addr) {
                    Ok(mut tcpStream) => {
                        info!("{}.run | connected on: {:?}", selfId, addr);
                        thread::sleep(Duration::from_millis(100));
                        if !testData.is_empty() {
                            let (send, recv) = mpsc::channel();
                            let mut JdsMessage = JdsEncodeMessage::new(
                                &selfId,
                                JdsSerialize::new(&selfId, recv)
                            );
                            let txId = PointTxId::fromStr(&selfId);
                            let mut sentCount = 0;
                            let mut progressPercent = 0.0;
                            while testData.len() > 0 {
                                let value = testData.remove(0);
                                let point = value.to_point(txId, "test");
                                send.send(point.clone()).unwrap();
                                match JdsMessage.read() {
                                    Ok(bytes) => {
                                        match &tcpStream.write(&bytes) {
                                            Ok(_) => {
                                                sent.lock().unwrap().push(point);
                                                sentCount += 1;
                                                progressPercent = (sentCount as f32) / (totalCount as f32);
                                                switchState.add(progressPercent);
                                                debug!("{}.run | sent: {:?}", selfId, value);
                                            },
                                            Err(err) => {
                                                warn!("{}.run | socket write error: {:?}", selfId, err);
                                            },
                                        }
                                    },
                                    Err(err) => {
                                        panic!("{}.run | jdsSerialize error: {:?}", selfId, err);
                                    },
                                };
                                // if testData.is_empty() && waitOnFinish {
                                //     info!("{}.run | waitOnFinish: {}", selfId, waitOnFinish);
                                //     while !exit.load(Ordering::SeqCst) {
                                //         thread::sleep(Duration::from_millis(100));
                                //     }
                                // }
                                if switchState.changed() {
                                    info!("{}.run | state: {} progress percent: {}", selfId, switchState.state(), progressPercent);
                                    thread::sleep(Duration::from_millis(1000));
                                    tcpStream.flush().unwrap();
                                    thread::sleep(Duration::from_millis(1000));
                                    tcpStream.shutdown(std::net::Shutdown::Both).unwrap();
                                    // drop(tcpStream);
                                    thread::sleep(Duration::from_millis(1000));
                                    break;
                                } 
                                if exit.load(Ordering::SeqCst) {
                                    break;
                                }
                            }
                        }
                        if switchState.isMax() {
                            info!("{}.run | switchState.isMax, exiting", selfId);
                            break 'connect;
                        }
                        if testData.is_empty() {
                            info!("{}.run | testData.is_empty, exiting", selfId);
                            tcpStream.flush().unwrap();
                            thread::sleep(Duration::from_millis(1000));
                            break 'connect;
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | connection error: {:?}", selfId, err);
                        thread::sleep(Duration::from_millis(1000))
                    },
                }
                if switchState.isMax() {
                    info!("{}.run | switchState.isMax, exiting", selfId);
                    break 'connect;
                }
                if testData.is_empty() {
                    info!("{}.run | testData.is_empty, exiting", selfId);
                    break 'connect;
                }
                if exit.load(Ordering::SeqCst) {
                    info!("{}.run | exit detected, exiting", selfId);
                    break 'connect;
                }
            }
            info!("{}.run | Exit", selfId);
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

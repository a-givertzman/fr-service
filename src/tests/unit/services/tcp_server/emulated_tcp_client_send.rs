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
/// - all point from [test_data] will be sent via socket
/// - all received point in the received() method
/// - if [recvLimit] is some then thread exit when riched recvLimit
/// - [disconnect] - contains percentage (0..100) of test_data / iterations, where socket will be disconnected and connected again
pub struct EmulatedTcpClientSend {
    id: String,
    addr: SocketAddr,
    test_data: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    disconnect: Vec<i8>,
    waitOnFinish: bool,
    exit: Arc<AtomicBool>,
}
///
/// 
impl EmulatedTcpClientSend {
    pub fn new(parent: impl Into<String>, addr: &str, test_data: Vec<Value>, disconnect: Vec<i8>, waitOnFinish: bool) -> Self {
        let self_id = format!("{}/EmulatedTcpClientSend", parent.into());
        Self {
            id: self_id.clone(),
            addr: addr.parse().unwrap(),
            test_data,
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
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let addr = self.addr.clone();
        let mut test_data = self.test_data.clone();
        let totalCount = test_data.len();
        let sent = self.sent.clone();
        let disconnect = self.disconnect.iter().map(|v| {(*v as f32) / 100.0}).collect();
        let _waitOnFinish = self.waitOnFinish;
        let handle = thread::Builder::new().name(format!("{}.run Read", self_id)).spawn(move || {
            info!("{}.run | Preparing thread Read - ok", self_id);
            let mut switchState = Self::switchState(1, disconnect, 1.0);
            'connect: loop {
                match TcpStream::connect(addr) {
                    Ok(mut tcpStream) => {
                        info!("{}.run | connected on: {:?}", self_id, addr);
                        thread::sleep(Duration::from_millis(100));
                        if !test_data.is_empty() {
                            let (send, recv) = mpsc::channel();
                            let mut JdsMessage = JdsEncodeMessage::new(
                                &self_id,
                                JdsSerialize::new(&self_id, recv)
                            );
                            let txId = PointTxId::fromStr(&self_id);
                            let mut sentCount = 0;
                            let mut progressPercent = 0.0;
                            while test_data.len() > 0 {
                                let value = test_data.remove(0);
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
                                                debug!("{}.run | sent: {:?}", self_id, value);
                                            },
                                            Err(err) => {
                                                warn!("{}.run | socket write error: {:?}", self_id, err);
                                            },
                                        }
                                    },
                                    Err(err) => {
                                        panic!("{}.run | jdsSerialize error: {:?}", self_id, err);
                                    },
                                };
                                // if test_data.is_empty() && waitOnFinish {
                                //     info!("{}.run | waitOnFinish: {}", self_id, waitOnFinish);
                                //     while !exit.load(Ordering::SeqCst) {
                                //         thread::sleep(Duration::from_millis(100));
                                //     }
                                // }
                                if switchState.changed() {
                                    info!("{}.run | state: {} progress percent: {}", self_id, switchState.state(), progressPercent);
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
                            info!("{}.run | switchState.isMax, exiting", self_id);
                            break 'connect;
                        }
                        if test_data.is_empty() {
                            info!("{}.run | test_data.is_empty, exiting", self_id);
                            tcpStream.flush().unwrap();
                            thread::sleep(Duration::from_millis(1000));
                            break 'connect;
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | connection error: {:?}", self_id, err);
                        thread::sleep(Duration::from_millis(1000))
                    },
                }
                if switchState.isMax() {
                    info!("{}.run | switchState.isMax, exiting", self_id);
                    break 'connect;
                }
                if test_data.is_empty() {
                    info!("{}.run | test_data.is_empty, exiting", self_id);
                    break 'connect;
                }
                if exit.load(Ordering::SeqCst) {
                    info!("{}.run | exit detected, exiting", self_id);
                    break 'connect;
                }
            }
            info!("{}.run | Exit", self_id);
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

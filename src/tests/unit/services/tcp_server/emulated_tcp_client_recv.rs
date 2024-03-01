#![allow(non_snake_case)]

use log::{info, trace, warn, debug};
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread::{JoinHandle, self}, time::Duration, net::{TcpStream, SocketAddr}, io::Write};
use testing::entities::test_value::Value;
use crate::{
    core_::{
        point::point_type::PointType, 
        net::{
            connection_status::ConnectionStatus,
            protocols::jds::{jds_deserialize::JdsDeserialize, jds_decode_message::JdsDecodeMessage}, 
        }, state::{switch_state::{SwitchState, Switch, SwitchCondition}, switch_state_changed::SwitchStateChanged},
    },
    services::service::service::Service, 
};


///
/// Jast connects to the tcp socket on [address]
/// - all point from [test_data] will be sent via socket
/// - all received point in the received() method
/// - if [recvLimit] is some then thread exit when riched recvLimit
/// - [disconnect] - contains percentage (0..100) of test_data / iterations, where socket will be disconnected and connected again
pub struct EmulatedTcpClientRecv {
    id: String,
    addr: SocketAddr,
    received: Arc<Mutex<Vec<PointType>>>,
    recvLimit: Option<usize>,
    mustReceived: Option<Value>,
    disconnect: Vec<i8>,
    markerReceived: Arc<AtomicBool>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl EmulatedTcpClientRecv {
    pub fn new(parent: impl Into<String>, addr: &str, recvLimit: Option<usize>, mustReceived: Option<Value>, disconnect: Vec<i8>) -> Self {
        let self_id = format!("{}/EmulatedTcpClientRecv", parent.into());
        Self {
            id: self_id.clone(),
            addr: addr.parse().unwrap(),
            received: Arc::new(Mutex::new(vec![])),
            recvLimit,
            mustReceived,
            disconnect,
            markerReceived: Arc::new(AtomicBool::new(false)),
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
    pub fn received(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.received.clone()
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
    ///
    /// 
    pub fn waitAllReceived(&self) {
        let recvLimit = self.recvLimit.unwrap_or(0);
        info!("{}.waitAllReceived | wait all beeng received: {}/{}", self.id(), self.received.lock().unwrap().len(), recvLimit);
        loop {
            if self.received.lock().unwrap().len() >= recvLimit {
                break;
            }
            thread::sleep(Duration::from_millis(100));
            trace!("{}.waitAllReceived | wait all beeng received: {}/{}", self.id(), self.received.lock().unwrap().len(), recvLimit);
        }
    }
    ///
    /// 
    pub fn waitMarkerReceived(&self) {
        match &self.mustReceived {
            Some(mustReceived) => {
                info!("{}.waitMarkerReceived | Wait for {:?} marker beeng received", self.id, mustReceived);
                loop {
                    if self.markerReceived.load(Ordering::SeqCst) {
                        break;
                    }
                    thread::sleep(Duration::from_millis(100));
                    trace!("{}.waitMarkerReceived | wait for {:?} marker beeng received", self.id, self.mustReceived);
                }
            },
            None => {},
        }
    }
}
///
/// 
impl Service for EmulatedTcpClientRecv {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn get_link(&mut self, _name: &str) -> std::sync::mpsc::Sender<crate::core_::point::point_type::PointType> {
        panic!("{}.get_link | Does not support static producer", self.id())
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
        let markerReceived = self.markerReceived.clone();
        let addr = self.addr.clone();
        let received = self.received.clone();
        let recvLimit = self.recvLimit.clone();
        let mustReceived = self.mustReceived.clone();
        let disconnect = self.disconnect.iter().map(|v| {(*v as f32) / 100.0}).collect();
        let handle = thread::Builder::new().name(format!("{}.run Read", self_id)).spawn(move || {
            info!("{}.run | Preparing thread Read - ok", self_id);
            let mut switchState = Self::switchState(1, disconnect, 1.0);
            let mut switchStateChanged = false;
            'connect: loop {
                match TcpStream::connect(addr) {
                    Ok(mut tcpStream) => {
                        info!("{}.run | connected on: {:?}", self_id, addr);
                        let mut jdsDeserialize = JdsDeserialize::new(
                            self_id.clone(),
                            JdsDecodeMessage::new(
                                self_id.clone(),
                            ),
                        );
                        match recvLimit {
                            Some(recvLimit) => {
                                if recvLimit > 0 {
                                    let mut progressPercent = 0.0;
                                    let mut receivedCount = 0;
                                    loop {
                                        match jdsDeserialize.read(&tcpStream) {
                                            ConnectionStatus::Active(result) => {
                                                trace!("{}.run | received: {:?}", self_id, result);
                                                match result {
                                                    Ok(point) => {
                                                        debug!("{}.run | received: {:?}", self_id, point);
                                                        received.lock().unwrap().push(point.clone());
                                                        receivedCount += 1;
                                                        progressPercent = (receivedCount as f32) / (recvLimit as f32);
                                                        switchState.add(progressPercent);
                                                        if let Some(mustReceived) = &mustReceived {
                                                            let markerReceived_ = match mustReceived {
                                                                Value::Bool(value) => value == &point.as_bool().value.0,
                                                                Value::Int(value) => value == &point.as_int().value,
                                                                Value::Float(value) => value == &point.as_float().value,
                                                                Value::String(value) => value == &point.as_string().value,
                                                            };
                                                            if markerReceived_ {
                                                                info!("{}.run | received marker {:?}, exiting...", self_id, point);
                                                                markerReceived.store(markerReceived_, Ordering::SeqCst);
                                                                break;
                                                            }
                                                        }
                                                    },
                                                    Err(err) => {
                                                        warn!("{}.run | read socket error: {:?}", self_id, err);
                                                    },
                                                }
                                            },
                                            ConnectionStatus::Closed(err) => {
                                                warn!("{}.run | socket connection closed: {:?}", self_id, err);
                                                break;
                                            },
                                        };
                                        if switchStateChanged {
                                            switchStateChanged = false;
                                            info!("{}.run | state: {} progress percent: {}", self_id, switchState.state(), progressPercent);
                                            tcpStream.shutdown(std::net::Shutdown::Both).unwrap();
                                            drop(tcpStream);
                                            thread::sleep(Duration::from_millis(1000));
                                            break;
                                        }
                                        if switchState.changed() {
                                            info!("{}.run | state: {} progress percent: {}", self_id, switchState.state(), progressPercent);
                                            switchStateChanged = true;
                                            tcpStream.flush().unwrap();
                                        } 
                                        if receivedCount >= recvLimit {
                                            exit.store(true, Ordering::SeqCst);
                                            break;
                                        }
                                        if exit.load(Ordering::SeqCst) {
                                            break;
                                        }
                                    }
                                }
                            },
                            None => {
                                loop {
                                    match jdsDeserialize.read(&tcpStream) {
                                        ConnectionStatus::Active(result) => {
                                            trace!("{}.run | received: {:?}", self_id, result);
                                            match result {
                                                Ok(point) => {
                                                    received.lock().unwrap().push(point);
                                                },
                                                Err(err) => {
                                                    warn!("{}.run | read socket error: {:?}", self_id, err);
                                                },
                                            }
                                        },
                                        ConnectionStatus::Closed(err) => {
                                            warn!("{}.run | socket connection closed: {:?}", self_id, err);
                                            break;
                                        },
                                    };
                                    if exit.load(Ordering::SeqCst) {
                                        break;
                                    }
                                }
                            },
                        };
                    },
                    Err(err) => {
                        warn!("{}.run | connection error: {:?}", self_id, err);
                        thread::sleep(Duration::from_millis(1000))
                    },
                }
                if exit.load(Ordering::SeqCst) {
                    break 'connect;
                }
            }
            info!("{}.run | Exit thread Recv", self_id);
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

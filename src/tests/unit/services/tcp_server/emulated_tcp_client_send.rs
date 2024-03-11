use const_format::formatcp;
use log::{info, warn, debug};
use serde_json::json;
use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc}, thread::{JoinHandle, self}, time::Duration, net::{TcpStream, SocketAddr}, io::Write};
use testing::entities::test_value::Value;
use crate::{
    conf::point_config::{point_config::PointConfig, point_name::PointName}, core_::{
        cot::cot::Cot, net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, object::object::Object, point::{point::Point, point_tx_id::PointTxId, point_type::{PointType, ToPoint}}, state::{switch_state::{Switch, SwitchCondition, SwitchState}, switch_state_changed::SwitchStateChanged}, status::status::Status
    }, services::service::service::Service, tcp::steam_read::StreamRead 
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
    point_path: String,
    test_data: Vec<Value>,
    sent: Arc<Mutex<Vec<PointType>>>,
    disconnect: Vec<i8>,
    wait_on_finish: bool,
    exit: Arc<AtomicBool>,
}
///
/// 
impl EmulatedTcpClientSend {
    pub fn new(parent: impl Into<String>, point_path: impl Into<String>, addr: &str, test_data: Vec<Value>, disconnect: Vec<i8>, wait_on_finish: bool) -> Self {
        let self_id = format!("{}/EmulatedTcpClientSend", parent.into());
        Self {
            id: self_id.clone(),
            addr: addr.parse().unwrap(),
            point_path: point_path.into(),
            test_data,
            sent: Arc::new(Mutex::new(vec![])),
            disconnect,
            wait_on_finish,
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
    #[allow(dead_code)]
    pub fn sent(&self) -> Arc<Mutex<Vec<PointType>>> {
        self.sent.clone()
    }
    ///
    /// 
    fn switch_state<T: std::cmp::PartialOrd + Clone + 'static>(initial: u8, steps: Vec<T>, fin: T) -> SwitchStateChanged<u8, T> {
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
        let switch_state: SwitchStateChanged<u8, T> = SwitchStateChanged::new(
            SwitchState::new(
                initial,
                switches,
            ),
        );
        switch_state
    }
}
///
/// 
impl Object for EmulatedTcpClientSend {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for EmulatedTcpClientSend {
    //
    //
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let point_path = self.point_path.clone();
        let exit = self.exit.clone();
        let addr = self.addr.clone();
        let mut test_data = self.test_data.clone();
        let total_count = test_data.len();
        let sent = self.sent.clone();
        let disconnect = self.disconnect.iter().map(|v| {(*v as f32) / 100.0}).collect();
        let _wait_on_finish = self.wait_on_finish;
        let handle = thread::Builder::new().name(format!("{}.run Read", self_id)).spawn(move || {
            info!("{}.run | Preparing thread Read - ok", self_id);
            let mut switch_state = Self::switch_state(1, disconnect, 1.0);
            'connect: loop {
                match TcpStream::connect(addr) {
                    Ok(mut tcp_stream) => {
                        info!("{}.run | connected on: {:?}", self_id, addr);
                        thread::sleep(Duration::from_millis(100));
                        if !test_data.is_empty() {
                            let (send, recv) = mpsc::channel();
                            let mut jds_message = JdsEncodeMessage::new(
                                &self_id,
                                JdsSerialize::new(&self_id, recv)
                            );
                            // let request = PointType::String(Point::new(
                            //     0, 
                            //     &PointName::new(&point_path, "/Subscribe").full(),
                            //     json!(["/test/Jds/test"]).to_string(),
                            //     Status::Ok,
                            //     Cot::Req,
                            //     chrono::offset::Utc::now(),
                            // ));
                            // send.send(request).unwrap();
                            // thread::sleep(Duration::from_millis(100));
                            let tx_id = PointTxId::fromStr(&self_id);
                            let mut sent_count = 0;
                            let mut progress_percent = 0.0;
                            while test_data.len() > 0 {
                                let value = test_data.remove(0);
                                let point = value.to_point(tx_id, &PointName::new(&point_path, "/test").full());
                                send.send(point.clone()).unwrap();
                                match jds_message.read() {
                                    Ok(bytes) => {
                                        match &tcp_stream.write(&bytes) {
                                            Ok(_) => {
                                                sent.lock().unwrap().push(point);
                                                sent_count += 1;
                                                progress_percent = (sent_count as f32) / (total_count as f32);
                                                switch_state.add(progress_percent);
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
                                if switch_state.changed() {
                                    info!("{}.run | state: {} progress percent: {}", self_id, switch_state.state(), progress_percent);
                                    thread::sleep(Duration::from_millis(1000));
                                    tcp_stream.flush().unwrap();
                                    thread::sleep(Duration::from_millis(1000));
                                    tcp_stream.shutdown(std::net::Shutdown::Both).unwrap();
                                    // drop(tcpStream);
                                    thread::sleep(Duration::from_millis(1000));
                                    break;
                                } 
                                if exit.load(Ordering::SeqCst) {
                                    break;
                                }
                            }
                        }
                        if switch_state.isMax() {
                            info!("{}.run | switchState.isMax, exiting", self_id);
                            break 'connect;
                        }
                        if test_data.is_empty() {
                            info!("{}.run | test_data.is_empty, exiting", self_id);
                            tcp_stream.flush().unwrap();
                            thread::sleep(Duration::from_millis(1000));
                            break 'connect;
                        }
                    },
                    Err(err) => {
                        warn!("{}.run | connection error: {:?}", self_id, err);
                        thread::sleep(Duration::from_millis(1000))
                    },
                }
                if switch_state.isMax() {
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
    // fn points(&self) -> Vec<crate::conf::point_config::point_config::PointConfig> {
    //     let types = vec!["Bool", "Int", "Float", "String"];
    //     types.iter().map(|type_| {
    //         let conf = format!(
    //             r#"{}:
    //                 type: {}      # Bool / Int / Float / String / Json
    //                 comment: Auth request, contains token / pass string"#, 
    //             PointName::new(&self.point_path, "/test").full(),
    //             type_,
    //         );
    //         let conf = serde_yaml::from_str(&conf).unwrap();
    //         PointConfig::from_yaml(&self.point_path, &conf)
    //     }).collect()
    // }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

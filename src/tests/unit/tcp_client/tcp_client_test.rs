#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug, error, trace, warn};
    use rand::Rng;
    use std::{sync::{Once, Arc, Mutex, atomic::AtomicUsize}, thread, time::{Duration, Instant}, net::TcpListener, io::{Read, Write}, process::exit};
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::point_type::{ToPoint, PointType}, net::{protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize}, connection_status::ConnectionStatus}},
        conf::tcp_client_config::TcpClientConfig,  
        services::{service::Service, tcp_client::tcp_client::TcpClient, services::Services},
    }; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn initOnce() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach() -> () {
    
    }
    
    #[derive(Clone)]
    enum Value {
        Bool(bool),
        Int(i64),
        Float(f64),
        String(String),
    }
    impl Value {
        fn toString(&self) -> String {
            match &self {
                Value::Bool(v) => v.to_string(),
                Value::Int(v) => v.to_string(),
                Value::Float(v) => v.to_string(),
                Value::String(v) => v.to_string(),
            }
        }
        fn toPoint(&self, name: &str) -> PointType {
            match &self {
                Value::Bool(v) => v.toPoint(name),
                Value::Int(v) => v.toPoint(name),
                Value::Float(v) => v.toPoint(name),
                Value::String(v) => v.clone().toPoint(name),
            }
        }
    }
    
    #[test]
    fn test_TcpClient() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_TcpClient");
        let mut rnd = rand::thread_rng();
        let path = "./src/tests/unit/tcp_client/tcp_client.yaml";
        let conf = TcpClientConfig::read(path);
        let addr = conf.address.clone();
        let tcpClient = TcpClient::new("test TcpClient", conf);
        let mut services = Services::new("test");
        let tcpClientServiceId = "TcpClient";
        services.insert(tcpClientServiceId, Box::new(tcpClient));
        // let tcpClient = Arc::new(Mutex::new(tcpClient));

        let maxTestDuration = Duration::from_secs(10);
        let count = 300;
        let mut state = 0;
        let testData = vec![
            Value::Int(7),
            Value::Float(1.3),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_owned()),
            Value::String("test2".to_owned()),
        ];
        let testDataLen = testData.len();

        let mut sent = vec![];
        let received = Arc::new(Mutex::new(vec![]));


        mocTcpServer(addr.to_string(), count, testData.clone(), received.clone());
        thread::sleep(Duration::from_micros(100));


        // return;
        let timer = Instant::now();
        let tcpClient = services.get(tcpClientServiceId);
        let send = tcpClient.getLink("link");
        for _ in 0..count {
            let index = rnd.gen_range(0..testDataLen);
            let value = testData.get(index).unwrap();
            let point = value.toPoint("teset");
            send.send(point.clone()).unwrap();
            sent.push(point);
        }
        let waitDuration = Duration::from_millis(10);
        let mut waitAttempts = maxTestDuration.as_micros() / waitDuration.as_micros();
        while received.lock().unwrap().len() < count {
            debug!("waiting while all data beeng received {}/{}...", received.lock().unwrap().len(), count);
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), maxTestDuration);
        }
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", count);
        println!("sent events: {:?}", sent.len());
        let mut received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent.len() == count, "sent: {:?}\ntarget: {:?}", sent.len(), count);
        assert!(received.len() == count, "received: {:?}\ntarget: {:?}", received.len(), count);
        // while &sent.len() > &0 {
        //     let target = sent.pop().unwrap();
        //     let result = received.pop().unwrap();
        //     let result = result.as_object().unwrap().get("sql").unwrap().as_object().unwrap().get("sql").unwrap().as_str().unwrap();
        //     debug!("\nresult({}): {:?}\ntarget({}): {:?}", received.len(), result, sent.len(), target);
        //     assert!(result == &target, "\nresult: {:?}\ntarget: {:?}", result, target);
        // }
    }
    ///
    /// TcpServer setup
    fn mocTcpServer(addr: String, count: usize, testData: Vec<Value>, received: Arc<Mutex<Vec<serde_json::Value>>>) {
        let mut sent = 0;
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            let mut rng = rand::thread_rng();
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok");
                    let mut acceptCount = 2;
                    let mut maxReadErrors = 3;
                    while acceptCount > 0 {
                        acceptCount -= 1;
                        match listener.accept() {
                            Ok((mut _socket, addr)) => {
                                info!("TCP server | accept connection - ok\n\t{:?}", addr);
                                let mut jds = JdsDeserialize::new(
                                    "test", 
                                    JdsDecodeMessage::new("test", _socket),
                                );
                                for _ in 0..count {
                                    match jds.read() {
                                        ConnectionStatus::Active(point) => {

                                        },
                                        ConnectionStatus::Closed => {
                                            
                                        },
                                    }

                                }
                                info!("TCP server | all sent: {:?}", sent);
                                while received.lock().unwrap().len() < count {
                                    thread::sleep(Duration::from_micros(100));
                                }
                                // while received.len() < count {}
                            },
                            Err(err) => {
                                warn!("incoming connection - error: {:?}", err);
                            },
                        }
                    }
                },
                Err(err) => {
                    // connectExit.send(true).unwrap();
                    // okRef.store(false, Ordering::SeqCst);
                    panic!("Preparing test TCP server - error: {:?}", err);
                },
            };
        });
    }    
}



















        // thread::spawn(move || {
        //     let mut received = receivedRef.lock().unwrap();
        //     info!("TCP server | Preparing test server...");
        //     match TcpListener::bind(addr) {
        //         Ok(listener) => {
        //             info!("TCP server | Preparing test server - ok");
        //             let mut acceptCount = 2;
        //             let mut maxReadErrors = 3;
        //             while acceptCount > 0 {
        //                 acceptCount -= 1;
        //                 match listener.accept() {
        //                     Ok((mut _socket, addr)) => {
        //                         info!("TCP server | accept connection - ok\n\t{:?}", addr);
        //                         _socket.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        //                         while received.len() < count {
        //                             for e in buf.iter_mut() {*e = 0;}
        //                             match _socket.read(&mut buf) {
        //                                 Ok(bytes) => {
        //                                     debug!("TCP server | received bytes: {:?}", bytes);
        //                                     let raw = String::from_utf8(buf.to_vec()).unwrap();
        //                                     let raw = raw.trim_matches(char::from(0));
        //                                     debug!("TCP server | received raw: {:?}", raw);
        //                                     match serde_json::from_str(&raw) {
        //                                         Ok(value) => {
        //                                             let value: serde_json::Value = value;
        //                                             debug!("TCP server | received: {:?}", value);
        //                                             received.push(value.clone());
        //                                             // let obj = value.as_object().unwrap();
        //                                             // let reply = SqlReply {
        //                                             //     authToken: obj.get("authToken").unwrap().as_str().unwrap().to_string(),
        //                                             //     id: obj.get("id").unwrap().as_str().unwrap().to_string(),
        //                                             //     keepAlive: obj.get("keepAlive").unwrap().as_bool().unwrap(),
        //                                             //     query: "".into(),
        //                                             //     data: vec![],
        //                                             //     error: ApiError::empty(),
        //                                             // };
        //                                             // match _socket.write(&reply.asBytes()) {
        //                                             //     Ok(bytes) => {
        //                                             //         debug!("TCP server | sent bytes: {:?}", bytes);
        //                                             //     },
        //                                             //     Err(err) => {
        //                                             //         debug!("TCP server | socket write - error: {:?}", err);
        //                                             //     },
        //                                             // };
        //                                             // // debug!("TCP server | received / count: {:?}", received.len() / count);
        //                                             // if (state == 0) && received.len() as f64 / count as f64 > 0.333 {
        //                                             //     state = 1;
        //                                             //     let duration = Duration::from_millis(500);
        //                                             //     debug!("TCP server | beaking socket connection for {:?}", duration);
        //                                             //     _socket.flush().unwrap();
        //                                             //     _socket.shutdown(std::net::Shutdown::Both).unwrap();
        //                                             //     thread::sleep(duration);
        //                                             //     debug!("TCP server | beaking socket connection for {:?} - elapsed, restoring...", duration);
        //                                             //     break;
        //                                             // }
        //                                         },
        //                                         Err(err) => {
        //                                             debug!("TCP server | parse read data error: {:?}", err);
        //                                         },
        //                                     };
        //                                 },
        //                                 Err(err) => {
        //                                     debug!("socket read - error: {:?}", err);
        //                                     maxReadErrors -= 1;
        //                                     if maxReadErrors <= 0 {
        //                                         error!("TCP server | socket read error: {:?}", err);
        //                                         break;
        //                                     }
        //                                 },
        //                             };
        //                             thread::sleep(Duration::from_micros(100));
        //                         }
        //                     },
        //                     Err(err) => {
        //                         info!("incoming connection - error: {:?}", err);
        //                     },
        //                 }
        //             }
        //         },
        //         Err(err) => {
        //             // connectExit.send(true).unwrap();
        //             // okRef.store(false, Ordering::SeqCst);
        //             panic!("Preparing test TCP server - error: {:?}", err);
        //         },
        //     };
        // });


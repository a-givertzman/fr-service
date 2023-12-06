#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug, warn};
    use rand::Rng;
    use std::{sync::{Once, Arc, Mutex}, thread, time::{Duration, Instant}, net::TcpListener, io::BufReader};
    use crate::{
        core_::{
            debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
            testing::{test_session::TestSession, test_stuff::test_value::Value},
            point::point_type::PointType, 
            net::{protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize}, 
            connection_status::ConnectionStatus}, 
        },
        conf::tcp_client_config::TcpClientConfig,  
        services::{tcp_client::tcp_client::TcpClient, services::Services}, tests::unit::services::tcp_client::mock_multiqueue::MockMultiqueue,
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
    
    #[test]
    fn test_TcpClient_write() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_TcpClient WRITE");
        let mut rnd = rand::thread_rng();
        let path = "./src/tests/unit/services/tcp_client/tcp_client.yaml";
        let mut conf = TcpClientConfig::read(path);
        let addr = "127.0.0.1:".to_owned() + &TestSession::freeTcpPortStr();
        conf.address = addr.parse().unwrap();
        let services = Arc::new(Mutex::new(Services::new("test")));
        let multiQueue = Arc::new(Mutex::new(MockMultiqueue::new()));
        let tcpClient = Arc::new(Mutex::new(TcpClient::new("test TcpClient", conf, services.clone())));
        let multiQueueServiceId = "MultiQueue";
        let tcpClientServiceId = "TcpClient";
        services.lock().unwrap().insert(tcpClientServiceId, tcpClient.clone());
        services.lock().unwrap().insert(multiQueueServiceId, multiQueue.clone());

        let maxTestDuration = Duration::from_secs(10);
        let count = 100;
        let testData = vec![
            Value::Int(7),
            Value::Float(1.3),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_string()),
            Value::String("test2".to_string()),
        ];
        let testDataLen = testData.len();

        let mut sent = vec![];
        let received = Arc::new(Mutex::new(vec![]));


        mockTcpServer(addr.to_string(), count, testData.clone(), received.clone());
        thread::sleep(Duration::from_micros(100));

        debug!("Getting services...");
        let services = services.lock().unwrap();
        debug!("Getting services - ok");
        debug!("Getting service {}...", tcpClientServiceId);
        let tcpClient = services.get(tcpClientServiceId);
        debug!("Getting service {} - ok", tcpClientServiceId);
        debug!("Running service {}...", tcpClientServiceId);
        drop(services);
        tcpClient.lock().unwrap().run().unwrap();
        debug!("Running service {} - ok", tcpClientServiceId);
        let timer = Instant::now();
        let send = tcpClient.lock().unwrap().getLink("link");
        debug!("Test - setup - ok");
        debug!("Sending points...");
        for _ in 0..count {
            let index = rnd.gen_range(0..testDataLen);
            let value = testData.get(index).unwrap();
            let point = value.toPoint(0, "teset");
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
        let received = received.lock().unwrap();
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
    fn mockTcpServer(addr: String, count: usize, _testData: Vec<Value>, received: Arc<Mutex<Vec<PointType>>>) {
        let sent = 0;
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            // let mut rng = rand::thread_rng();
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok");
                    let mut acceptCount = 2;
                    // let mut maxReadErrors = 3;
                    while acceptCount > 0 {
                        acceptCount -= 1;
                        match listener.accept() {
                            Ok((mut _socket, addr)) => {
                                let mut tcpStream = BufReader::new(_socket);
                                info!("TCP server | accept connection - ok\n\t{:?}", addr);
                                let mut jds = JdsDeserialize::new(
                                    "test", 
                                    JdsDecodeMessage::new("test"),
                                );
                                for _ in 0..count {
                                    match jds.read(&mut tcpStream) {
                                        ConnectionStatus::Active(point) => {
                                            match point {
                                                Ok(point) => {
                                                    received.lock().unwrap().push(point);
                                                },
                                                Err(err) => {
                                                    warn!("{:?}", err);
                                                },
                                            }
                                        },
                                        ConnectionStatus::Closed(_err) => {
                                            warn!("TCP server | connection - closed");
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


// struct MockMultiqueue {
//     id: String,
//     send: Sender<PointType>,
//     recv: Receiver<PointType>,
// }
// impl MockMultiqueue {
//     fn new() -> Self {
//         let (send, recv) = std::sync::mpsc::channel();
//         Self {
//             id: "MockMultiqueue".to_owned(),
//             send,
//             recv,
//         }
//     }
// }
// impl Service for MockMultiqueue {
//     //
//     //
//     fn getLink(&self, name: &str) -> Sender<PointType> {
//         assert!(name == "queue", "{}.run | link '{:?}' - not found", self.id, name);
//         self.send.clone()
//     }
//     //
//     // 
//     fn run(&mut self) {
//         todo!()
//     }
//     //
//     // 
//     fn exit(&self) {
//         todo!()
//     }
// }
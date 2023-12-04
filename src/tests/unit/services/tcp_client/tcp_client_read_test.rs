#![allow(non_snake_case)]
#[cfg(test)]


mod tests {
    use log::{info, debug, warn, error};
    use std::{sync::{Once, Arc, Mutex}, thread, time::{Duration, Instant}, net::TcpListener, io::Write};
    use crate::{
        core_::{
            debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
            testing::{test_session::TestSession, test_stuff::test_value::Value},
            point::point_type::{ToPoint, PointType}, 
            net::protocols::jds::{jds_serialize::JdsSerialize, jds_encode_message::JdsEncodeMessage}, 
        },
        conf::tcp_client_config::TcpClientConfig,  
        services::{tcp_client::tcp_client::TcpClient, services::Services, service::Service}, 
        tests::unit::services::tcp_client::mock_multiqueue::MockMultiqueue, 
        tcp::steam_read::StreamRead, 
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
    fn test_TcpClient_read() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_TcpClient READ");
        // let mut rnd = rand::thread_rng();
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
        let iterations = 100;
        let testData = vec![
            Value::Int(7),
            Value::Float(1.3),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_string()),
            Value::String("test2".to_string()),
        ];
        let testDataLen = testData.len();
        let totalCount = testDataLen * iterations;

        let sent = Arc::new(Mutex::new(vec![]));
        // let received = Arc::new(Mutex::new(vec![]));


        mockTcpServer(addr.to_string(), iterations, testData.clone(), sent.clone(), multiQueue.clone());
        thread::sleep(Duration::from_micros(100));

        debug!("Getting services...");
        let services = services.lock().unwrap();
        debug!("Getting services - ok");

        debug!("Getting service {}...", tcpClientServiceId);
        let tcpClient = services.get(tcpClientServiceId);
        debug!("Getting service {} - ok", tcpClientServiceId);

        drop(services);
        debug!("Running service {}...", multiQueueServiceId);
        multiQueue.lock().unwrap().run();
        debug!("Running service {} - ok", multiQueueServiceId);
        debug!("Running service {}...", tcpClientServiceId);
        tcpClient.lock().unwrap().run();
        debug!("Running service {} - ok", tcpClientServiceId);
        let timer = Instant::now();
        debug!("Test - setup - ok");
        let waitDuration = Duration::from_millis(10);
        let mut waitAttempts = maxTestDuration.as_micros() / waitDuration.as_micros();
        // let received = multiQueue.lock().unwrap().received();
        while multiQueue.lock().unwrap().received().lock().unwrap().len() < totalCount {
            debug!("waiting while all data beeng received {}/{}...", multiQueue.lock().unwrap().received().lock().unwrap().len(), totalCount);
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", multiQueue.lock().unwrap().received().lock().unwrap().len(), totalCount, timer.elapsed(), maxTestDuration);
        }
        let mut sent = sent.lock().unwrap();
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", totalCount);
        println!("sent events: {:?}", sent.len());
        let mq = multiQueue.lock().unwrap();
        let received = mq.received();
        let mut received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent.len() == totalCount, "sent: {:?}\ntarget: {:?}", sent.len(), totalCount);
        assert!(received.len() == totalCount, "received: {:?}\ntarget: {:?}", received.len(), totalCount);
        while &sent.len() > &0 {
            let target = sent.pop().unwrap();
            let result = received.pop().unwrap();
            // let result = result.as_object().unwrap().get("sql").unwrap().as_object().unwrap().get("sql").unwrap().as_str().unwrap();
            debug!("\nresult({}): {:?}\ntarget({}): {:?}", received.len(), result, sent.len(), target);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
    ///
    /// TcpServer setup
    fn mockTcpServer(addr: String, count: usize, testData: Vec<Value>, sent: Arc<Mutex<Vec<PointType>>>, multiqueue: Arc<Mutex<MockMultiqueue>>) {
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            let mut rng = rand::thread_rng();
            let (send, recv) = std::sync::mpsc::channel();
            let mut jds = JdsEncodeMessage::new(
                "test", 
                JdsSerialize::new(
                    "test", 
                    recv,
                ),
            );
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok");
                    // let mut acceptCount = 2;
                    // let mut maxReadErrors = 3;
                    match listener.accept() {
                        Ok((mut socket, addr)) => {
                            info!("TCP server | accept connection - ok\n\t{:?}", addr);
                            for _ in 0..count {
                                for value in &testData {
                                    let point = value.toPoint("test");
                                    send.send(point.clone()).unwrap();
                                    match jds.read() {
                                        Ok(bytes) => {
                                            warn!("TCP server | send bytes: {:?}", bytes);
                                            match socket.write(&bytes) {
                                                Ok(_) => {
                                                    sent.lock().unwrap().push(point);
                                                },
                                                Err(err) => {
                                                    warn!("TCP server | socket.wrtite error: {:?}", err);
                                                },
                                            }
                                        },
                                        Err(err) => {
                                            error!("TCP server | error: {:?}", err);
                                        },
                                    }
                                }
                            }
                            info!("TCP server | all sent: {:?}", sent.lock().unwrap().len());
                            let received = multiqueue.lock().unwrap().received();
                            while received.lock().unwrap().len() < count {
                                thread::sleep(Duration::from_micros(100));
                            }
                            // while received.len() < count {}
                        },
                        Err(err) => {
                            warn!("incoming connection - error: {:?}", err);
                        },
                    }
                    // while acceptCount > 0 {
                    //     // acceptCount -= 1;
                    //     acceptCount = -1;
                    // }
                },
                Err(err) => {
                    // connectExit.send(true).unwrap();
                    // okRef.store(false, Ordering::SeqCst);
                    panic!("Preparing test TCP server - error: {:?}", err);
                },
            };
        });
        info!("TCP server | Started");
    }
}

#![allow(non_snake_case)]
#[cfg(test)]


mod tests {
    use log::{info, debug, warn, error, trace};
    use std::{sync::{Once, Arc, Mutex}, thread, time::{Duration, Instant}, net::TcpListener, io::Write};
    use testing::{session::test_session::TestSession, entities::test_value::Value, stuff::{random_test_values::RandomTestValues, max_test_duration::TestDuration}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::tcp_client_config::TcpClientConfig, core_::{
            net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, point::point_type::{PointType, ToPoint} 
        }, services::{service::Service, services::Services, tcp_client::tcp_client::TcpClient}, tcp::steam_read::StreamRead, tests::unit::services::tcp_client::mock_multiqueue::MockMultiqueue 
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
        let selfId = "test TcpClient READ";
        println!("{}", selfId);
        let path = "./src/tests/unit/services/tcp_client/tcp_client.yaml";
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let mut conf = TcpClientConfig::read(path);
        let addr = "127.0.0.1:".to_owned() + &TestSession::free_tcp_port_str();
        conf.address = addr.parse().unwrap();

        let iterations = 100;
        let testData = RandomTestValues::new(
            selfId, 
            vec![
                Value::Int(i64::MIN),
                Value::Int(i64::MAX),
                Value::Int(-7),
                Value::Int(0),
                Value::Int(12),
                Value::Float(f64::MAX),
                Value::Float(f64::MIN),
                Value::Float(f64::MIN_POSITIVE),
                Value::Float(-f64::MIN_POSITIVE),
                Value::Float(0.0),
                Value::Float(1.33),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(false),
                Value::Bool(true),
                Value::String("test1".to_string()),
                Value::String("test1test1test1test1test1test1test1test1test1test1test1test1test1test1test1".to_string()),
                Value::String("test2".to_string()),
                Value::String("test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2".to_string()),
            ], 
            iterations, 
        );
        let testData: Vec<Value> = testData.collect();
        let totalCount = testData.len();

        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let multiQueue = Arc::new(Mutex::new(MockMultiqueue::new(Some(totalCount))));
        let tcpClient = Arc::new(Mutex::new(TcpClient::new(selfId, conf, services.clone())));
        let multiQueueServiceId = "MultiQueue";
        let tcpClientServiceId = "TcpClient";
        services.lock().unwrap().insert(tcpClientServiceId, tcpClient.clone());
        services.lock().unwrap().insert(multiQueueServiceId, multiQueue.clone());

        let sent = Arc::new(Mutex::new(vec![]));
        debug!("Lock services...");
        let services = services.lock().unwrap();
        debug!("Lock services - ok");
        debug!("Lock service {}...", tcpClientServiceId);
        let tcpClient = services.get(tcpClientServiceId);
        debug!("Lock service {} - ok", tcpClientServiceId);
        drop(services);
        debug!("Running service {}...", multiQueueServiceId);
        let handle = multiQueue.lock().unwrap().run().unwrap();
        debug!("Running service {} - ok", multiQueueServiceId);
        debug!("Running service {}...", tcpClientServiceId);
        tcpClient.lock().unwrap().run().unwrap();
        debug!("Running service {} - ok", tcpClientServiceId);
        mockTcpServer(addr.to_string(), iterations, testData.clone(), sent.clone(), multiQueue.clone());
        thread::sleep(Duration::from_micros(100));
        
        let timer = Instant::now();
        debug!("Test - setup - ok");
        handle.join().unwrap();
        // let waitDuration = Duration::from_millis(100);
        // let mut waitAttempts = testDuration.as_micros() / waitDuration.as_micros();
        // while multiQueue.lock().unwrap().received().lock().unwrap().len() < totalCount {
        //     debug!("waiting while all data beeng received {}/{}...", multiQueue.lock().unwrap().received().lock().unwrap().len(), totalCount);
        //     thread::sleep(waitDuration);
        //     waitAttempts -= 1;
        //     assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", multiQueue.lock().unwrap().received().lock().unwrap().len(), totalCount, timer.elapsed(), testDuration);
        // }
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
            debug!("\nresult({}): {:?}\ntarget({}): {:?}", received.len(), result, sent.len(), target);
            assert!(result.name() == target.name(), "\nresult: {:?}\ntarget: {:?}", result, target);
            assert!(result.status() == target.status(), "\nresult: {:?}\ntarget: {:?}", result, target);
            assert!(result.timestamp() == target.timestamp(), "\nresult: {:?}\ntarget: {:?}", result, target);
            assert!(result.cmp_value(&target), "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }
    ///
    /// TcpServer setup
    fn mockTcpServer(addr: String, count: usize, testData: Vec<Value>, sent: Arc<Mutex<Vec<PointType>>>, multiqueue: Arc<Mutex<MockMultiqueue>>) {
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
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
                    match listener.accept() {
                        Ok((mut socket, addr)) => {
                            info!("TCP server | accept connection - ok\n\t{:?}", addr);
                            for value in &testData {
                                let point = value.to_point(0, "test");
                                send.send(point.clone()).unwrap();
                                match jds.read() {
                                    Ok(bytes) => {
                                        trace!("TCP server | send bytes: {:?}", bytes);
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
                            info!("TCP server | all sent: {:?}", sent.lock().unwrap().len());
                            let received = multiqueue.lock().unwrap().received();
                            while received.lock().unwrap().len() < count {
                                thread::sleep(Duration::from_millis(100));
                            }
                        },
                        Err(err) => {
                            warn!("incoming connection - error: {:?}", err);
                        },
                    }
                },
                Err(err) => {
                    panic!("Preparing test TCP server - error: {:?}", err);
                },
            };
        });
        info!("TCP server | Started");
    }
}

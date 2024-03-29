#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug, warn};
    use std::{sync::{Once, Arc, Mutex}, thread::{self, JoinHandle}, time::{Duration, Instant}, net::TcpListener, io::BufReader};
    use testing::{session::test_session::TestSession, entities::test_value::Value, stuff::{random_test_values::RandomTestValues, max_test_duration::TestDuration}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::tcp_client_config::TcpClientConfig, core_::{
            net::{connection_status::ConnectionStatus, protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize}}, point::point_type::{PointType, ToPoint} 
        }, services::{services::Services, tcp_client::tcp_client::TcpClient}, tests::unit::services::tcp_client::mock_multiqueue::MockMultiqueue
    }; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {
    
    }
    
    #[test]
    fn test_TcpClient_write() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test TcpClient WRITE";
        println!("\n{}", self_id);
        let path = "./src/tests/unit/services/tcp_client/tcp_client.yaml";
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let mut conf = TcpClientConfig::read(path);
        let addr = "127.0.0.1:".to_owned() + &TestSession::free_tcp_port_str();
        conf.address = addr.parse().unwrap();

        let iterations = 100;
        let test_data = RandomTestValues::new(
            self_id, 
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
        let test_data: Vec<Value> = test_data.collect();

        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let multiQueue = Arc::new(Mutex::new(MockMultiqueue::new(None)));
        let tcpClient = Arc::new(Mutex::new(TcpClient::new(self_id, conf, services.clone())));
        let multiQueueServiceId = "MultiQueue";
        let tcpClientServiceId = "TcpClient";
        services.lock().unwrap().insert(tcpClientServiceId, tcpClient.clone());
        services.lock().unwrap().insert(multiQueueServiceId, multiQueue.clone());

        let mut sent = vec![];
        let received = Arc::new(Mutex::new(vec![]));


        let handle = mockTcpServer(addr.to_string(), iterations, received.clone());
        thread::sleep(Duration::from_micros(100));

        debug!("Lock services...");
        let services = services.lock().unwrap();
        debug!("Lock services - ok");
        debug!("Lock service {}...", tcpClientServiceId);
        let tcpClient = services.get(tcpClientServiceId);
        debug!("Lock service {} - ok", tcpClientServiceId);
        drop(services);
        debug!("Running service {}...", tcpClientServiceId);
        tcpClient.lock().unwrap().run().unwrap();
        debug!("Running service {} - ok", tcpClientServiceId);
        let timer = Instant::now();
        let send = tcpClient.lock().unwrap().get_link("link");
        debug!("Test - setup - ok");
        debug!("Sending points...");
        for value in test_data {
            let point = value.to_point(0, "teset");
            send.send(point.clone()).unwrap();
            sent.push(point);
        }
        handle.join().unwrap();
        // let waitDuration = Duration::from_millis(10);
        // let mut waitAttempts = test_duration.as_micros() / waitDuration.as_micros();
        // while received.lock().unwrap().len() < count {
        //     debug!("waiting while all data beeng received {}/{}...", received.lock().unwrap().len(), count);
        //     thread::sleep(waitDuration);
        //     waitAttempts -= 1;
        //     assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), test_duration);
        // }
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", iterations);
        println!("sent events: {:?}", sent.len());
        let received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent.len() == iterations, "sent: {:?}\ntarget: {:?}", sent.len(), iterations);
        assert!(received.len() == iterations, "received: {:?}\ntarget: {:?}", received.len(), iterations);
        test_duration.exit();
    }
    ///
    /// TcpServer setup
    fn mockTcpServer(addr: String, count: usize, received: Arc<Mutex<Vec<PointType>>>) -> JoinHandle<()> {
        let sent = 0;
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok");
                    let mut acceptCount = 2;
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
                                let mut receivedCount = 0;
                                loop {
                                    match jds.read(&mut tcpStream) {
                                        ConnectionStatus::Active(point) => {
                                            match point {
                                                Ok(point) => {
                                                    received.lock().unwrap().push(point);
                                                    receivedCount += 1;
                                                    if receivedCount >= count {
                                                        acceptCount = -1;
                                                        break;
                                                    }
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
                                info!("TCP server | all received: {:?}", sent);
                                // while received.lock().unwrap().len() < count {
                                //     thread::sleep(Duration::from_micros(100));
                                // }
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
        })
    }    
}

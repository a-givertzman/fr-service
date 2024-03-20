#[cfg(test)]

mod tcp_client {
    use log::{info, debug, warn, error, trace};
    use std::{sync::{Once, Arc, Mutex}, thread, time::{Duration, Instant}, net::TcpListener, io::Write};
    use testing::{entities::test_value::Value, session::test_session::TestSession, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::tcp_client_config::TcpClientConfig, core_::{
            net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, point::point_type::{PointType, ToPoint} 
        }, services::{service::service::Service, services::Services, tcp_client::tcp_client::TcpClient}, tcp::steam_read::StreamRead, tests::unit::services::tcp_client::mock_multiqueue::MockMultiqueue 
    }; 
    ///
    ///     
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
    fn init_each() -> () {}
    ///
    /// 
    #[test]
    fn read() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test TcpClient READ";
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
        let total_count = test_data.len();

        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let multi_queue = Arc::new(Mutex::new(MockMultiqueue::new(Some(total_count))));
        let tcp_client = Arc::new(Mutex::new(TcpClient::new(self_id, conf, services.clone())));
        let multi_queue_service_id = "MultiQueue";
        let tcp_client_service_id = "TcpClient";
        services.lock().unwrap().insert(tcp_client_service_id, tcp_client.clone());
        services.lock().unwrap().insert(multi_queue_service_id, multi_queue.clone());

        let sent = Arc::new(Mutex::new(vec![]));
        debug!("Lock services...");
        let services = services.lock().unwrap();
        debug!("Lock services - ok");
        debug!("Lock service {}...", tcp_client_service_id);
        let tcp_client = services.get(tcp_client_service_id);
        debug!("Lock service {} - ok", tcp_client_service_id);
        drop(services);
        debug!("Running service {}...", multi_queue_service_id);
        let handle = multi_queue.lock().unwrap().run().unwrap();
        debug!("Running service {} - ok", multi_queue_service_id);
        debug!("Running service {}...", tcp_client_service_id);
        tcp_client.lock().unwrap().run().unwrap();
        debug!("Running service {} - ok", tcp_client_service_id);
        mock_tcp_server(addr.to_string(), iterations, test_data.clone(), sent.clone(), multi_queue.clone());
        thread::sleep(Duration::from_micros(100));
        let timer = Instant::now();
        debug!("Test - setup - ok");
        handle.wait().unwrap();
        // let waitDuration = Duration::from_millis(100);
        // let mut waitAttempts = test_duration.as_micros() / waitDuration.as_micros();
        // while multiQueue.lock().unwrap().received().lock().unwrap().len() < totalCount {
        //     debug!("waiting while all data beeng received {}/{}...", multiQueue.lock().unwrap().received().lock().unwrap().len(), totalCount);
        //     thread::sleep(waitDuration);
        //     waitAttempts -= 1;
        //     assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", multiQueue.lock().unwrap().received().lock().unwrap().len(), totalCount, timer.elapsed(), test_duration);
        // }
        let mut sent = sent.lock().unwrap();
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", total_count);
        println!("sent events: {:?}", sent.len());
        let mq = multi_queue.lock().unwrap();
        let received = mq.received();
        let mut received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent.len() == total_count, "sent: {:?}\ntarget: {:?}", sent.len(), total_count);
        assert!(received.len() == total_count, "received: {:?}\ntarget: {:?}", received.len(), total_count);
        while &sent.len() > &0 {
            let target = sent.pop().unwrap();
            let result = received.pop().unwrap();
            debug!("\nresult({}): {:?}\ntarget({}): {:?}", received.len(), result, sent.len(), target);
            assert!(result.name() == target.name(), "\nresult: {:?}\ntarget: {:?}", result, target);
            assert!(result.status() == target.status(), "\nresult: {:?}\ntarget: {:?}", result, target);
            assert!(result.timestamp() == target.timestamp(), "\nresult: {:?}\ntarget: {:?}", result, target);
            assert!(result.cmp_value(&target), "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
    ///
    /// TcpServer setup
    fn mock_tcp_server(addr: String, count: usize, test_data: Vec<Value>, sent: Arc<Mutex<Vec<PointType>>>, multiqueue: Arc<Mutex<MockMultiqueue>>) {
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
                            for value in &test_data {
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

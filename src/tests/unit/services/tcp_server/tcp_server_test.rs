#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, thread};
    use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::test_stuff::{max_test_duration::MaxTestDuration, inc_test_values::IncTestValues, test_value::Value}}, conf::tcp_server_config::TcpServerConfig, services::{tcp_server::tcp_server::TcpServer, services::Services, service::Service}, tests::unit::services::tcp_server::mock_tcp_client::MockTcpClient}; 
    
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
    fn test_tcp_server_send() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test TcpServer");
        let selfId = "test";
        let maxTestDuration = MaxTestDuration::new(selfId, Duration::from_secs(10));
        maxTestDuration.run().unwrap();

        let iterations = 100;
        let testData = IncTestValues::new(
            selfId, 
            0, 
            iterations, 
        );
        let testData: Vec<Value> = testData.collect();
        let totalCount = testData.len();

        
        let conf = r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: 127.0.0.1:8080
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.queue
        "#;
        let conf = serde_yaml::from_str(conf).unwrap();
        let conf = TcpServerConfig::fromYamlValue(&conf);
        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let tcpServer = Arc::new(Mutex::new(TcpServer::new(selfId, conf, services.clone())));
        let mockTcpClient = Arc::new(Mutex::new(MockTcpClient::new(
            selfId,
            "MultiQueue.queue",
            services.clone(),
            testData,
            Some(iterations),
        )));
        services.lock().unwrap().insert("TcpServer", tcpServer.clone());
        let handle = tcpServer.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(1000));

        let received = mockTcpClient.lock().unwrap().received();
        let received = received.lock().unwrap();
        let target = totalCount;
        let result = received.len();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        for (index, point) in received.iter().enumerate() {
            let result = point.asInt().value;
            let target = index as i64;
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }


        tcpServer.lock().unwrap().exit();
        handle.join().unwrap();
        maxTestDuration.exit();
    }
}

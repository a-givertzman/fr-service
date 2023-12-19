#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, thread};
    use crate::{
        tests::unit::services::tcp_server::emulated_tcp_client::EmulatedTcpClient,
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::{test_stuff::{max_test_duration::MaxTestDuration, inc_test_values::IncTestValues, test_value::Value}, test_session::TestSession}}, 
        conf::{tcp_server_config::TcpServerConfig, multi_queue_config::MultiQueueConfig}, 
        services::{tcp_server::tcp_server::TcpServer, services::Services, service::Service, task::task_test_producer::TaskTestProducer, multi_queue::multi_queue::MultiQueue}, 
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

        let tcpPort = TestSession::freeTcpPortStr();
        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: 127.0.0.1:{}
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.queue
        "#, tcpPort);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::fromYamlValue(&conf);
        let tcpServer = Arc::new(Mutex::new(TcpServer::new(selfId, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcpServer.clone());

        let mqConf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
        "#;
        let mqConf = serde_yaml::from_str(mqConf).unwrap();
        let mqConf = MultiQueueConfig::fromYamlValue(&mqConf);
        let mqService = Arc::new(Mutex::new(MultiQueue::new(selfId, mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            selfId,
            "MultiQueue.queue",
            services.clone(),
            testData.clone(),
        )));
        services.lock().unwrap().insert("TaskTestProducer", producer.clone());
        let emulatedTcpClient = Arc::new(Mutex::new(EmulatedTcpClient::new(
            selfId,
            &format!("127.0.0.1:{}", tcpPort),
            testData,
            Some(iterations),
        )));
        let handle = tcpServer.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(1000));

        let received = emulatedTcpClient.lock().unwrap().received();
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


// pub struct 
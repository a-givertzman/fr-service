#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use std::{sync::{Once, Arc, Mutex}, time::Duration, thread};
    use crate::{
        tests::unit::services::tcp_server::emulated_tcp_client::EmulatedTcpClient,
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::{test_stuff::{max_test_duration::MaxTestDuration, inc_test_values::IncTestValues, test_value::Value, wait::WaitTread}, test_session::TestSession}}, 
        conf::{tcp_server_config::TcpServerConfig, multi_queue_config::MultiQueueConfig}, 
        services::{tcp_server::tcp_server::TcpServer, services::Services, service::Service, task::{task_test_producer::TaskTestProducer, task_test_receiver::TaskTestReceiver}, multi_queue::multi_queue::MultiQueue}, 
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
    fn test_TcpServer_send() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        println!("test TcpServer | Send");
        let selfId = "test";
        let maxTestDuration = MaxTestDuration::new(selfId, Duration::from_secs(20));
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
        let tcpAddr = format!("127.0.0.1:{}", tcpPort);
        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcpAddr);
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
            "MultiQueue.in-queue",
            Duration::ZERO,
            services.clone(),
            testData.clone(),
        )));
        services.lock().unwrap().insert("TaskTestProducer", producer.clone());
        let emulatedTcpClient = Arc::new(Mutex::new(EmulatedTcpClient::new(
            selfId,
            &tcpAddr,
            vec![],
            Some(iterations),
            None,
            vec![],
        )));
        let mqServiceHandle = mqService.lock().unwrap().run().unwrap();
        let tcpServerHandle = tcpServer.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let emulatedTcpClientHandle = emulatedTcpClient.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let producerHandle = producer.lock().unwrap().run().unwrap();
        producerHandle.wait().unwrap();
        emulatedTcpClient.lock().unwrap().waitAllReceived();
        
        let received = emulatedTcpClient.lock().unwrap().received();
        let mut received = received.lock().unwrap();
        let target = totalCount;
        let result = received.len();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        for value in testData {
            let result = received.remove(0).asInt().value;
            let target = value.asInt();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        
        emulatedTcpClient.lock().unwrap().exit();
        tcpServer.lock().unwrap().exit();
        mqService.lock().unwrap().exit();
        emulatedTcpClientHandle.wait().unwrap();
        tcpServerHandle.wait().unwrap();
        mqServiceHandle.wait().unwrap();
        maxTestDuration.exit();
    }

    #[test]
    fn test_TcpServer_receive() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        println!("test TcpServer | Receive");
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
        let tcpAddr = format!("127.0.0.1:{}", tcpPort);
        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcpAddr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::fromYamlValue(&conf);
        let tcpServer = Arc::new(Mutex::new(TcpServer::new(selfId, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcpServer.clone());

        let mqConf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
                    - TaskTestReceiver.queue
        "#;
        let mqConf = serde_yaml::from_str(mqConf).unwrap();
        let mqConf = MultiQueueConfig::fromYamlValue(&mqConf);
        let mqService = Arc::new(Mutex::new(MultiQueue::new(selfId, mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            selfId,
            "queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        let emulatedTcpClient = Arc::new(Mutex::new(EmulatedTcpClient::new(
            selfId,
            &tcpAddr,
            testData.clone(),
            Some(0),
            None,
            vec![],
        )));
        let mqServiceHandle = mqService.lock().unwrap().run().unwrap();
        let tcpServerHandle = tcpServer.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let emulatedTcpClientHandle = emulatedTcpClient.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let receiverHandle = receiver.lock().unwrap().run().unwrap();
        receiverHandle.wait().unwrap();
        
        let received = receiver.lock().unwrap().received();
        let mut received = received.lock().unwrap();
        let target = totalCount;
        let result = received.len();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        for value in testData {
            let result = received.remove(0).asInt().value;
            let target = value.asInt();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        
        emulatedTcpClient.lock().unwrap().exit();
        tcpServer.lock().unwrap().exit();
        mqService.lock().unwrap().exit();
        emulatedTcpClientHandle.wait().unwrap();
        tcpServerHandle.wait().unwrap();
        mqServiceHandle.wait().unwrap();
        maxTestDuration.exit();
    }
}


// pub struct 
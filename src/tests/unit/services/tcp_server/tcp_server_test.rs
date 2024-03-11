#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use std::{sync::{Once, Arc, Mutex}, time::Duration, thread};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, inc_test_values::IncTestValues, wait::WaitTread}, session::test_session::TestSession};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        tests::unit::services::tcp_server::{emulated_tcp_client_recv::EmulatedTcpClientRecv, emulated_tcp_client_send::EmulatedTcpClientSend},
        conf::{tcp_server_config::TcpServerConfig, multi_queue_config::MultiQueueConfig}, 
        services::{tcp_server::tcp_server::TcpServer, services::Services, service::service::Service, task::{task_test_producer::TaskTestProducer, task_test_receiver::TaskTestReceiver}, multi_queue::multi_queue::MultiQueue}, 
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
    fn test_TcpServer_send() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test TcpServer | Send";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(20));
        test_duration.run().unwrap();

        let iterations = 100;
        let test_data = IncTestValues::new(
            self_id, 
            0, 
            iterations, 
        );
        let test_data: Vec<Value> = test_data.collect();
        let totalCount = test_data.len();

        let tcpPort = TestSession::free_tcp_port_str();
        let tcpAddr = format!("127.0.0.1:{}", tcpPort);
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                auth: none      # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                auth-secret: 
                    pass: /home/scada/.ssh/ #/ auth-ssh: path: ...
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcpAddr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::from_yaml(&conf);
        let tcpServer = Arc::new(Mutex::new(TcpServer::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcpServer.clone());

        let mqConf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
        "#;
        let mqConf = serde_yaml::from_str(mqConf).unwrap();
        let mqConf = MultiQueueConfig::from_yaml(&mqConf);
        let mqService = Arc::new(Mutex::new(MultiQueue::new(self_id, mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            self_id,
            "MultiQueue.in-queue",
            Duration::ZERO,
            services.clone(),
            test_data.clone(),
        )));
        services.lock().unwrap().insert("TaskTestProducer", producer.clone());
        let emulatedTcpClient = Arc::new(Mutex::new(EmulatedTcpClientRecv::new(
            self_id,
            &tcpAddr,
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
        for value in test_data {
            let result = received.remove(0).as_int().value;
            let target = value.as_int();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        
        emulatedTcpClient.lock().unwrap().exit();
        tcpServer.lock().unwrap().exit();
        mqService.lock().unwrap().exit();
        emulatedTcpClientHandle.wait().unwrap();
        tcpServerHandle.wait().unwrap();
        mqServiceHandle.wait().unwrap();
        test_duration.exit();
    }

    #[test]
    fn test_TcpServer_receive() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test TcpServer | Receive";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();

        let iterations = 100;
        let test_data = IncTestValues::new(
            self_id, 
            0, 
            iterations, 
        );
        let test_data: Vec<Value> = test_data.collect();
        let totalCount = test_data.len();

        let tcpPort = TestSession::free_tcp_port_str();
        let tcpAddr = format!("127.0.0.1:{}", tcpPort);
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                auth: none      # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcpAddr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::from_yaml(&conf);
        let tcpServer = Arc::new(Mutex::new(TcpServer::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcpServer.clone());

        let mqConf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
                    - TaskTestReceiver.queue
        "#;
        let mqConf = serde_yaml::from_str(mqConf).unwrap();
        let mqConf = MultiQueueConfig::from_yaml(&mqConf);
        let mqService = Arc::new(Mutex::new(MultiQueue::new(self_id, mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            self_id,
            "queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        let emulatedTcpClient = Arc::new(Mutex::new(EmulatedTcpClientSend::new(
            self_id,
            &tcpAddr,
            test_data.clone(),
            vec![],
            false,
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
        for value in test_data {
            let result = received.remove(0).as_int().value;
            let target = value.as_int();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        
        emulatedTcpClient.lock().unwrap().exit();
        tcpServer.lock().unwrap().exit();
        mqService.lock().unwrap().exit();
        emulatedTcpClientHandle.wait().unwrap();
        tcpServerHandle.wait().unwrap();
        mqServiceHandle.wait().unwrap();
        test_duration.exit();
    }
}

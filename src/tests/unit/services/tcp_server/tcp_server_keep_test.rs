#[cfg(test)]

mod tcp_server {
    use std::{sync::{Once, Arc, Mutex}, time::Duration, thread};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, inc_test_values::IncTestValues, wait::WaitTread}, session::test_session::TestSession};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        tests::unit::services::tcp_server::{emulated_tcp_client_recv::EmulatedTcpClientRecv, emulated_tcp_client_send::EmulatedTcpClientSend},
        conf::{tcp_server_config::TcpServerConfig, multi_queue_config::MultiQueueConfig}, 
        services::{server::tcp_server::TcpServer, services::Services, service::service::Service, task::{task_test_producer::TaskTestProducer, task_test_receiver::TaskTestReceiver}, multi_queue::multi_queue::MultiQueue}, 
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
    fn keep_send() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test TcpServer keep lost connection | Send";
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
        let total_count = test_data.len();

        let tcp_port = TestSession::free_tcp_port_str();
        let tcp_addr = format!("127.0.0.1:{}", tcp_port);
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
        "#, tcp_addr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::from_yaml(&conf);
        let tcp_server = Arc::new(Mutex::new(TcpServer::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcp_server.clone());

        let mq_conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
        "#;
        let mq_conf = serde_yaml::from_str(mq_conf).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&mq_conf);
        let mq_service = Arc::new(Mutex::new(MultiQueue::new(self_id, mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());

        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            self_id,
            "MultiQueue.in-queue",
            Duration::from_millis(100),
            services.clone(),
            test_data.clone(),
        )));
        services.lock().unwrap().insert("TaskTestProducer", producer.clone());
        let emulated_tcp_client_recv = Arc::new(Mutex::new(EmulatedTcpClientRecv::new(
            self_id,
            &tcp_addr,
            Some(iterations),
            Some(test_data.last().unwrap().clone()),
            vec![25, 50, 75],
        )));
        let mq_service_handle = mq_service.lock().unwrap().run().unwrap();
        let tcp_server_handle = tcp_server.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let emulated_tcp_client_recv_handle = emulated_tcp_client_recv.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let producer_handle = producer.lock().unwrap().run().unwrap();
        emulated_tcp_client_recv.lock().unwrap().waitMarkerReceived();
        
        let received = emulated_tcp_client_recv.lock().unwrap().received();
        let received = received.lock().unwrap();
        let target = 0.75;
        let result = (received.len() as f32) / (total_count as f32);
        // println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", total_count);
        println!("sent events: {:?}", producer.lock().unwrap().sent().lock().unwrap().len());
        println!("recv events: {:?} ({}%)", received.len(), result * 100.0);
        assert!(result >= target, "\nresult: {:?}\ntarget: {:?}", result, target);
        
        emulated_tcp_client_recv.lock().unwrap().exit();
        producer.lock().unwrap().exit();
        tcp_server.lock().unwrap().exit();
        mq_service.lock().unwrap().exit();
        emulated_tcp_client_recv_handle.wait().unwrap();
        producer_handle.wait().unwrap();
        tcp_server_handle.wait().unwrap();
        mq_service_handle.wait().unwrap();
        test_duration.exit();
    }

    #[test]
    fn keep_receive() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test TcpServer keep lost connection | Receive";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(30));
        test_duration.run().unwrap();

        let iterations = 100;
        let test_data = IncTestValues::new(
            self_id, 
            0, 
            iterations, 
        );
        let test_data: Vec<Value> = test_data.collect();
        let total_count = test_data.len();

        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let tcp_port = TestSession::free_tcp_port_str();
        let tcp_addr = format!("127.0.0.1:{}", tcp_port);
        let conf = format!(r#"
            service TcpServer:
                cycle: 1 ms
                reconnect: 1 s  # default 3 s
                address: {}
                auth: none      # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                in queue link:
                    max-length: 10000
                out queue: MultiQueue.in-queue
        "#, tcp_addr);
        let conf = serde_yaml::from_str(&conf).unwrap();
        let conf = TcpServerConfig::from_yaml(&conf);
        let tcp_server = Arc::new(Mutex::new(TcpServer::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("TcpServer", tcp_server.clone());

        let mq_conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
                    - TaskTestReceiver.queue
        "#;
        let mq_conf = serde_yaml::from_str(mq_conf).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&mq_conf);
        let mq_service = Arc::new(Mutex::new(MultiQueue::new(self_id, mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());

        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            self_id,
            "queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        let emulated_tcp_client = Arc::new(Mutex::new(EmulatedTcpClientSend::new(
            self_id,
            "/test/Jds/",
            &tcp_addr,
            test_data.clone(),
            vec![25, 50, 75],
            true,
        )));
        let mq_service_handle = mq_service.lock().unwrap().run().unwrap();
        let tcp_server_handle = tcp_server.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let emulated_tcp_client_handle = emulated_tcp_client.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(100));
        let receiver_handle = receiver.lock().unwrap().run().unwrap();
        receiver_handle.wait().unwrap();
        emulated_tcp_client.lock().unwrap().exit();
        emulated_tcp_client_handle.wait().unwrap();
        
        let received = receiver.lock().unwrap().received();
        let mut received = received.lock().unwrap();
        let target = total_count;
        let result = received.len();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        for value in test_data {
            let result = received.remove(0).as_int().value;
            let target = value.as_int();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        tcp_server.lock().unwrap().exit();
        mq_service.lock().unwrap().exit();
        tcp_server_handle.wait().unwrap();
        mq_service_handle.wait().unwrap();
        test_duration.exit();
    }
}


// pub struct 
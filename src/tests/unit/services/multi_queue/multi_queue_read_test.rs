#[cfg(test)]

mod multi_queue {
    use log::debug;
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::multi_queue_config::MultiQueueConfig, services::{multi_queue::multi_queue::MultiQueue, services::Services, service::service::Service}, 
        tests::unit::services::multi_queue::{mock_recv_service::MockRecvService, mock_send_service::MockSendService},
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
    /// Test MultiQueue for - static link
    /// - action: read
    #[test]
    fn static_read() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "multi_queue_read_test";
        println!("\n{}", self_id);
        //
        // can be changed
        let iterations = 10;
        let test_data = RandomTestValues::new(
            self_id, 
            vec![
                Value::Int(i64::MIN),
                Value::Int(i64::MAX),
                Value::Int(-7),
                Value::Int(0),
                Value::Int(12),
                Value::Real(f32::MAX),
                Value::Real(f32::MIN),
                Value::Real(f32::MIN_POSITIVE),
                Value::Real(-f32::MIN_POSITIVE),
                Value::Real(0.0),
                Value::Real(1.33),
                Value::Double(f64::MAX),
                Value::Double(f64::MIN),
                Value::Double(f64::MIN_POSITIVE),
                Value::Double(-f64::MIN_POSITIVE),
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
        let test_data_len = test_data.len();
        let count = 30;
        let total_count = count * test_data.len();
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let mut conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
        "#.to_string();
        for i in 0..count {
            conf = format!("{}\n                    - MockRecvService{}.in-queue", conf, i)
        }
        let conf = serde_yaml::from_str(&conf).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&conf);
        debug!("mqConf: {:?}", mq_conf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mq_service = Arc::new(Mutex::new(MultiQueue::new("test", mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());

        let mut recv_handles = vec![];
        let mut recv_services = vec![];
        let timer = Instant::now();
        let send_service = Arc::new(Mutex::new(MockSendService::new(
            format!("test"),
            "MultiQueue.in-queue",
            services.clone(),
            test_data.clone(),
            None,
        )));
        services.lock().unwrap().insert("MockRecvService", send_service.clone());
        for i in 0..count {
            let recv_service = Arc::new(Mutex::new(MockRecvService::new(
                format!("tread{}", i),
                "in-queue",
                Some(iterations),
            )));
            services.lock().unwrap().insert(&format!("MockRecvService{}", i), recv_service.clone());
            recv_services.push(recv_service);
        }
        mq_service.lock().unwrap().run().unwrap();
        for service in &mut recv_services {
            let handle = service.lock().unwrap().run().unwrap();
            recv_handles.push(handle);
        }
        send_service.lock().unwrap().run().unwrap();
        for mut thd in recv_handles {
            thd.wait().unwrap();
        }
        println!("\nelapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", total_count);
        println!("sent events: {:?}\n", count * send_service.lock().unwrap().sent().lock().unwrap().len());
        let mut received = vec![];
        let target = test_data_len;
        for recv_service in &recv_services {
            let len = recv_service.lock().unwrap().received().lock().unwrap().len();
            assert!(len == target, "\nresult: {:?}\ntarget: {:?}", len, target);
            received.push(len);
        }
        println!("recv events: {} {:?}", received.iter().sum::<usize>(), received);
        for service in recv_services {
            service.lock().unwrap().exit();
        }
        test_duration.exit();
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }
}

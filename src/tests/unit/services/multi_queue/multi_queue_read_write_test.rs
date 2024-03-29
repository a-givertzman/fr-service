#[cfg(test)]

mod multi_queue {
    use log::debug;
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::multi_queue_config::MultiQueueConfig, 
        services::{multi_queue::multi_queue::MultiQueue, services::Services, service::service::Service}, 
        tests::unit::services::multi_queue::mock_recv_send_service::MockRecvSendService,
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
    /// Test MultiQueue for static link
    /// - action: read-write
    #[test]
    fn read_write() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let iterations = 10;
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
        let test_data_len = test_data.len();
        let count = 3;
        let total_count = count * test_data_len;
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let mut conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
        "#.to_string();
        for i in 0..count {
            conf = format!("{}\n                    - MockRecvSendService{}.in-queue", conf, i)
        }
        let conf = serde_yaml::from_str(&conf).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&conf);
        debug!("mqConf: {:?}", mq_conf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mq_service = Arc::new(Mutex::new(MultiQueue::new("test", mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());
        let timer = Instant::now();
        let mut rs_services = vec![];
        for i in 0..count {
            let rs_service = Arc::new(Mutex::new(MockRecvSendService::new(
                format!("tread{}", i),
                "in-queue",//MultiQueue.
                "MultiQueue.in-queue",
                services.clone(),
                test_data.clone(),
                Some(total_count),
            )));
            services.lock().unwrap().insert(&format!("MockRecvSendService{}", i), rs_service.clone());
            rs_services.push(rs_service);
        }
        mq_service.lock().unwrap().run().unwrap();
        let mut recv_handles = vec![];
        for service in &mut rs_services {
            let handle = service.lock().unwrap().run().unwrap();
            recv_handles.push(handle);
        }
        for thd in recv_handles {
            thd.wait().unwrap();
        }
        println!("\nelapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", total_count);
        for service in &rs_services {
            println!("sent events: {:?}\n", service.lock().unwrap().sent().lock().unwrap().len());
        }
        let mut received = vec![];
        let target = total_count;
        for recv_service in &rs_services {
            let len = recv_service.lock().unwrap().received().lock().unwrap().len();
            assert!(len == target, "\nresult: {:?}\ntarget: {:?}", len, target);
            received.push(len);
        }
        println!("recv events: {} {:?}", received.iter().sum::<usize>(), received);
        for service in rs_services {
            service.lock().unwrap().exit();
        }
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}

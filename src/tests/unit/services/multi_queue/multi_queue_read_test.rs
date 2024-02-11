#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::multi_queue_config::MultiQueueConfig, services::{multi_queue::multi_queue::MultiQueue, services::Services, service::Service}, 
        tests::unit::services::multi_queue::{mock_recv_service::MockRecvService, mock_send_service::MockSendService},
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
    fn test_MultiQueue_static_single() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test_multi_queue - Static subscriptions - Single send";
        println!("{}", selfId);

        let iterations = 10;
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
        let testDataLen = testData.len();
        let count = 30;
        let totalCount = count * testData.len();
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
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
        let mqConf = MultiQueueConfig::fromYamlValue(&conf);
        debug!("mqConf: {:?}", mqConf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mqService = Arc::new(Mutex::new(MultiQueue::new("test", mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let mut recvHandles = vec![];
        let mut recvServices = vec![];
        let timer = Instant::now();
        let sendService = Arc::new(Mutex::new(MockSendService::new(
            format!("test"),
            "in-queue",//MultiQueue.
            "MultiQueue.in-queue",
            services.clone(),
            testData.clone(),
            None,
        )));
        services.lock().unwrap().insert("MockRecvService", sendService.clone());
        for i in 0..count {
            let recvService = Arc::new(Mutex::new(MockRecvService::new(
                format!("tread{}", i),
                "in-queue",
                Some(iterations),
            )));
            services.lock().unwrap().insert(&format!("MockRecvService{}", i), recvService.clone());
            recvServices.push(recvService);
        }
        mqService.lock().unwrap().run().unwrap();
        for service in &mut recvServices {
            let handle = service.lock().unwrap().run().unwrap();
            recvHandles.push(handle);
        }
        sendService.lock().unwrap().run().unwrap();
        for thd in recvHandles {
            let thdId = format!("{:?}-{:?}", thd.thread().id(), thd.thread().name());
            info!("Waiting for service: {:?}...", thdId);
            thd.join().unwrap();
            info!("Waiting for thread: {:?} - finished", thdId);
        }
        println!("\nelapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", totalCount);
        println!("sent events: {:?}\n", count * sendService.lock().unwrap().sent().lock().unwrap().len());
        let mut received = vec![];
        let target = testDataLen;
        for recvService in &recvServices {
            let len = recvService.lock().unwrap().received().lock().unwrap().len();
            assert!(len == target, "\nresult: {:?}\ntarget: {:?}", len, target);
            received.push(len);
        }
        println!("recv events: {} {:?}", received.iter().sum::<usize>(), received);

        for service in recvServices {
            service.lock().unwrap().exit();
        }
        testDuration.exit();
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }
}

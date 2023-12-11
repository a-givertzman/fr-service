#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}};
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::test_stuff::{test_value::Value, max_test_duration::MaxTestDuration, random_test_values::RandomTestValues}}, 
        conf::multi_queue_config::MultiQueueConfig, 
        services::{multi_queue::multi_queue::MultiQueue, services::Services, service::Service}, 
        tests::unit::services::multi_queue::mock_rs_service::MockRecvSendService,
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
    fn test_multi_queue_static_single() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_multi_queue - Static subscriptions - Single send");
        let selfId = "test";
        let iterations = 10;
        let testData = RandomTestValues::new(
            selfId, 
            vec![
                Value::Int(7),
                Value::Float(1.3),
                Value::Bool(true),
                Value::Bool(false),
                Value::String("test1".to_string()),
                Value::String("test2".to_string()),
            ], 
            iterations, 
        );
        let testData: Vec<Value> = testData.collect();
        let testDataLen = testData.len();
        let count = 3;
        let totalCount = count * testData.len();
        let maxTestDuration = MaxTestDuration::new(selfId, Duration::from_secs(10));
        maxTestDuration.run().unwrap();
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
        let mqConf = MultiQueueConfig::fromYamlValue(&conf);
        // let mqConf = MultiQueueConfig::read(path);
        debug!("mqConf: {:?}", mqConf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mqService = Arc::new(Mutex::new(MultiQueue::new("test", mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let mut rsServices = vec![];
        let timer = Instant::now();
        for i in 0..count {
            let rsService = Arc::new(Mutex::new(MockRecvSendService::new(
                format!("tread{}", i),
                "in-queue",//MultiQueue.
                "MultiQueue.in-queue",
                services.clone(),
                testData.clone(),
            )));
            services.lock().unwrap().insert(&format!("MockRecvSendService{}", i), rsService.clone());
            rsServices.push(rsService);
        }
        mqService.lock().unwrap().run().unwrap();
        let mut recvHandles = vec![];
        for service in &mut rsServices {
            let handle = service.lock().unwrap().run().unwrap();
            recvHandles.push(handle);
        }
        for thd in recvHandles {
            let thdId = format!("{:?}-{:?}", thd.thread().id(), thd.thread().name());
            info!("Waiting for service: {:?}...", thdId);
            thd.join().unwrap();
            info!("Waiting for thread: {:?} - finished", thdId);
        }
        println!("\nelapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", totalCount);
        for service in &rsServices {
            println!("sent events: {:?}\n", service.lock().unwrap().sent().lock().unwrap().len());
        }
        let mut received = vec![];
        let target = testDataLen;
        for recvService in &rsServices {
            let len = recvService.lock().unwrap().received().lock().unwrap().len();
            assert!(len == target, "\nresult: {:?}\ntarget: {:?}", len, target);
            received.push(len);
        }
        println!("recv events: {} {:?}", received.iter().sum::<usize>(), received);

        for service in rsServices {
            service.lock().unwrap().exit();
        }
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        maxTestDuration.exit();
    }
}

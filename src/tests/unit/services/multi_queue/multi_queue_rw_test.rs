#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, thread};
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::test_stuff::test_value::Value}, 
        conf::multi_queue_config::MultiQueueConfig, 
        services::{multi_queue::multi_queue::MultiQueue, services::Services, service::Service}, 
        tests::unit::services::multi_queue::{mock_recv_service::MockRecvService, mock_rs_service::MockRecvSendService},
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
        let testData = Arc::new(Mutex::new(vec![
            Value::Int(7),
            Value::Float(1.3),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_string()),
            Value::String("test2".to_string()),
        ]));
        let testDataLen = testData.lock().unwrap().len();
        let count = 3;
        let totalCount = count * testData.lock().unwrap().len();
        let maxTestDuration = Duration::from_secs(10);
        
        // let path = "./src/tests/unit/services/multi_queue/multi_queue.yaml";
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

        let mut threads = vec![];
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
        for service in &mut rsServices {
            let handle = service.lock().unwrap().run().unwrap();
            threads.push(handle);
        }
        let waitDuration = Duration::from_micros(10);
        let mut waitAttempts = maxTestDuration.as_micros() / waitDuration.as_micros();
        let mut received = usize::MAX;
        let mut allReceivedPrev = vec![];
        while received != totalCount {
            let mut allReceived = vec![];
            for service in &rsServices {
                let r = service.lock().unwrap().received().lock().unwrap().len();
                allReceived.push(r);
                if allReceived != allReceivedPrev {
                    debug!("waiting while all data beeng received {:?}/{}...", allReceived, totalCount);
                    allReceivedPrev = allReceived.clone();
                }
            }
            received = allReceived.iter().sum::<usize>().clone();
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", received, totalCount, timer.elapsed(), maxTestDuration);
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
        for thd in threads {
            let thdId = format!("{:?}-{:?}", thd.thread().id(), thd.thread().name());
            info!("Waiting for service: {:?}...", thdId);
            thd.join().unwrap();
            info!("Waiting for thread: {:?} - finished", thdId);
        }
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }
}

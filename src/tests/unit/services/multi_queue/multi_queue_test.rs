#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, thread};
    use crate::{core_::debug::debug_session::{DebugSession, LogLevel, Backtrace}, conf::multi_queue_config::MultiQueueConfig, services::{multi_queue::multi_queue::MultiQueue, services::Services, service::Service}, tests::unit::services::multi_queue::mock_service::MockService}; 
    
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
    fn test_multi_queue() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_multi_queue");

        let count = 3;
        let path = "./src/tests/unit/services/multi_queue/multi_queue.yaml";
        let mqConf = MultiQueueConfig::read(path);
        debug!("mqConf: {:?}", mqConf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mq = MultiQueue::new("test", mqConf, services.clone());

        let testData = Arc::new(Mutex::new(vec![

        ]));

        let mut threads = vec![];
        let mut testServices = vec![];
        for i in 0..count {
            let thdTestData = testData.clone();
            let thdServices = services.clone();
            let mut service = MockService::new(
                format!("tread{}", i),
                "MultiQueue.in-queue",
                "queue",
                thdServices,
                thdTestData
            );
            // let handle = thread::Builder::new().name(format!("test thread #{}", i)).spawn(move || {
            //     info!("Preparing thread {} - ok", i);
            // }).unwrap();
            let handle = service.run().unwrap();
            testServices.push(service);
            threads.push(handle);
        }
        for service in testServices {
            service.exit();
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

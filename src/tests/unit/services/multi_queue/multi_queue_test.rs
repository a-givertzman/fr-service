#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, thread};
    use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::test_stuff::test_value::Value}, conf::multi_queue_config::MultiQueueConfig, services::{multi_queue::multi_queue::MultiQueue, services::Services, service::Service}, tests::unit::services::multi_queue::mock_service::MockService}; 
    
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
        let testData = Arc::new(Mutex::new(vec![
            Value::Int(7),
            Value::Float(1.3),
            Value::Bool(true),
            Value::Bool(false),
            Value::String("test1".to_string()),
            Value::String("test2".to_string()),
        ]));

        let count = 3;
        let totalCount = count * testData.lock().unwrap().len();
        let maxTestDuration = Duration::from_secs(10);
        
        let path = "./src/tests/unit/services/multi_queue/multi_queue.yaml";
        let mqConf = MultiQueueConfig::read(path);
        debug!("mqConf: {:?}", mqConf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mqService = Arc::new(Mutex::new(MultiQueue::new("test", mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let mut threads = vec![];
        let mut testServices = vec![];
        let timer = Instant::now();
        // let mut recvService = MockService::new(
        //     format!("tread{}", i),
        //     "in-queue",//MultiQueue.
        //     "MultiQueue.in-queue",
        //     thdServices,
        //     thdTestData
        // );
        for i in 0..count {
            let thdTestData = testData.clone();
            let thdServices = services.clone();
            let mut service = MockService::new(
                format!("tread{}", i),
                "in-queue",//MultiQueue.
                "MultiQueue.in-queue",
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
        for service in &mut testServices {
            service.run().unwrap();
        }
        let waitDuration = Duration::from_millis(10);
        let mut waitAttempts = maxTestDuration.as_micros() / waitDuration.as_micros();
        let mut received = usize::MAX;
        while received != totalCount {
            let mut allReceived = vec![];
            for service in &testServices {
                let r = service.received().lock().unwrap().len();
                debug!("waiting while all data beeng received {}/{}...", r, totalCount);
                allReceived.push(r);
            }
            received = allReceived.iter().min().unwrap().clone();
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", received, totalCount, timer.elapsed(), maxTestDuration);
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

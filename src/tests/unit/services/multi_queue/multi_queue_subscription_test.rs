#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, debug, error};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, thread::{self, JoinHandle}, any::Any};
    use crate::{
        core_::{
            debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
            testing::test_stuff::{test_value::Value, random_test_values::RandomTestValues, max_test_duration::MaxTestDuration},
        }, 
        conf::multi_queue_config::MultiQueueConfig, services::{multi_queue::multi_queue::MultiQueue, services::Services, service::Service}, 
        tests::unit::services::multi_queue::{mock_tcp_server::MockTcpServer, mock_recv_send_service::MockRecvSendService},
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
    fn test_multi_queue_subscribtions() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_multi_queue - Static subscriptions - Single send");

        let selfId = "test";
        let count = 3;              // count of the MockRecvSendService & MockTcpServer instances
        let iterations = 1000;      // test data length
        let staticTestData = RandomTestValues::new(
            selfId, 
            vec![
                Value::Int(12),
            ], 
            iterations, 
        );
        let staticTestData: Vec<Value> = staticTestData.collect();
        let staticTestDataLen = staticTestData.len();
        let dynamicTestData = RandomTestValues::new(
            selfId, 
            vec![
                Value::Int(12),
            ], 
            iterations, 
        );
        let dynamicTestData: Vec<Value> = dynamicTestData.collect();
        let dynamicTestDataLen = dynamicTestData.len();
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
        debug!("mqConf: {:?}", mqConf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mqService = Arc::new(Mutex::new(MultiQueue::new("test", mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());
        let mut handles = vec![];
        let mut rsServices = vec![];
        for i in 0..count {
            let rsService = Arc::new(Mutex::new(MockRecvSendService::new(
                format!("tread{}", i),
                "in-queue",     //MultiQueue.
                "MultiQueue.in-queue",
                services.clone(),
                staticTestData.clone(),
                Some(staticTestDataLen * count),
            )));
            services.lock().unwrap().insert(&format!("MockRecvSendService{}", i), rsService.clone());
            rsServices.push(rsService);
        }
        let mqHandle = mqService.lock().unwrap().run().unwrap();
        for rsService in &rsServices {
            let h = rsService.lock().unwrap().run().unwrap();
            handles.push(h);
        }
        for thd in handles {
            waitForThread(thd).unwrap();
        }
        println!("All MockRecvSendService threads - finished");
        // let mut tcpServerServices = vec![];
        // for i in 0..count {
        //     let tcpServerService = Arc::new(Mutex::new(MockTcpServer::new(
        //         format!("tread{}", i),
        //         "MultiQueue.in-queue",
        //         services.clone(),
        //         dynamicTestData.clone(),
        //         Some(iterations),
        //     )));
        //     services.lock().unwrap().insert(&format!("MockTcpServer{}", i), tcpServerService.clone());
        //     tcpServerServices.push(tcpServerService.clone());
        //     thread::sleep(Duration::from_millis(100));
        //     let thd = tcpServerService.lock().unwrap().run().unwrap();
        //     waitForThread(thd).unwrap();
        //     for rsService in &rsServices {
        //         let result = rsService.lock().unwrap().received().lock().unwrap().len();
        //         assert!(result == dynamicTestDataLen, "\nresult: {:?}\ntarget: {:?}", result, dynamicTestDataLen);
        //     }
        //     for tcpServerService in &tcpServerServices {
        //         let result = tcpServerService.lock().unwrap().received().lock().unwrap().len();
        //         assert!(result == dynamicTestDataLen, "\nresult: {:?}\ntarget: {:?}", result, dynamicTestDataLen);
        //     }
        //     tcpServerServices.push(tcpServerService.clone());
        // }
        for rsService in rsServices {
            rsService.lock().unwrap().exit();
        }
        // for tcpServerService in tcpServerServices {
        //     tcpServerService.lock().unwrap().exit();
        // }
        mqService.lock().unwrap().exit();
        mqHandle.join().unwrap();
        maxTestDuration.exit();
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }
    ///
    /// 
    fn waitForThread(thd: JoinHandle<()>) -> Result<(), Box<dyn Any + Send>>{
        let thdId = format!("{:?}-{:?}", thd.thread().id(), thd.thread().name());
        info!("Waiting for service: {:?}...", thdId);
        let r = thd.join();
        match &r {
            Ok(_) => {
                info!("Waiting for thread: '{}' - finished", thdId);
            },
            Err(err) => {
                error!("Waiting for thread '{}' error: {:?}", thdId, err);                
            },
        }
        r
    }
}

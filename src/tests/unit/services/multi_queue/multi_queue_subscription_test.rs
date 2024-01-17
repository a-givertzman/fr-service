#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::debug;
    use std::{sync::{Once, Arc, Mutex}, time::Duration, thread::{self}, collections::HashMap};
    use crate::{
        core_::{
            debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
            testing::test_stuff::{test_value::Value, random_test_values::RandomTestValues, max_test_duration::TestDuration, wait::WaitTread}, point::point_type::PointType,
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
    fn test_MultiQueue_subscribtions() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test_multi_queue - Static subscriptions - Single send";
        println!("{}", selfId);

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
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
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
                None,   //Some(staticTestDataLen * count),
            )));
            services.lock().unwrap().insert(&format!("MockRecvSendService{}", i), rsService.clone());
            rsServices.push(rsService);
        }
        let mqHandle = mqService.lock().unwrap().run().unwrap();
        for rsService in &rsServices {
            let h = rsService.lock().unwrap().run().unwrap();
            handles.push(h);
        }
        println!("All MockRecvSendService threads - finished");
        let mut tcpServerServices: Vec<Arc<Mutex<MockTcpServer>>> = vec![];
        let mut dynamicTarget: HashMap<i32, usize> = HashMap::new();
        for i in 0..count {
            let pointContent = format!("dynamic{}", i);
            let dynamicTestData = RandomTestValues::new(
                selfId, 
                vec![
                    Value::String(String::from(&pointContent)),
                ], 
                iterations, 
            );
            let dynamicTestData: Vec<Value> = dynamicTestData.collect();
            let tcpServerService = Arc::new(Mutex::new(MockTcpServer::new(
                format!("tread{}", i),
                "MultiQueue.in-queue",
                services.clone(),
                dynamicTestData.clone(),
                None,
            )));
            services.lock().unwrap().insert(&format!("MockTcpServer{}", i), tcpServerService.clone());
            let thdHandle = tcpServerService.lock().unwrap().run().unwrap();
            thdHandle.wait().unwrap();
            thread::sleep(Duration::from_millis(100));
            let target = 0;
            let result = pointsCount(tcpServerService.lock().unwrap().received().lock().unwrap().iter(), &pointContent);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            for rsService in &rsServices {
                let result = rsService.lock().unwrap().received().lock().unwrap().len();
                println!("Static service Received( {} ): {}", rsService.lock().unwrap().id(), result);
                // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            }
            for (index, tcpServerService) in tcpServerServices.iter().enumerate() {
                let result = pointsCount(tcpServerService.lock().unwrap().received().lock().unwrap().iter(), &pointContent);
                println!("Dynamic service Received( {} ): {}", tcpServerService.lock().unwrap().id(), result);
                // let result = tcpServerService.lock().unwrap().received().lock().unwrap().len();
                match dynamicTarget.get_mut(&(index as i32)) {
                    Some(value) => {
                        *value += iterations;
                    },
                    None => {
                        panic!("index {} - not found", index)
                    },
                };
                let target = dynamicTarget.get(&(index as i32)).unwrap();
                assert!(&result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            }
            println!("Dynamic service Received( {} ): {}", tcpServerService.lock().unwrap().id(), result);
            tcpServerServices.push(tcpServerService.clone());
            dynamicTarget.insert(i, 0);
        }
        for rsService in rsServices {
            rsService.lock().unwrap().exit();
        }
        for tcpServerService in tcpServerServices {
            tcpServerService.lock().unwrap().exit();
        }
        for thd in handles {
            thd.wait().unwrap();
        }
        mqService.lock().unwrap().exit();
        mqHandle.wait().unwrap();
        testDuration.exit();
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }
    ///
    /// 
    fn pointsCount<'a, T>(points: T, value: &str) -> usize 
        where T: Iterator<Item = &'a PointType>{
        let mut result = 0;
        for point in points {
            match point {
                PointType::Bool(_point) => {},
                PointType::Int(_point) => {},
                PointType::Float(_point) => {},
                PointType::String(point) => {
                    result += 1;
                    if point.value == value {
                    }
                },
            }
        }
        result
    }    
}

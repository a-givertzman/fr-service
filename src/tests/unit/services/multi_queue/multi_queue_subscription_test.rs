#[cfg(test)]

mod tests {
    use log::debug;
    use std::{sync::{Once, Arc, Mutex}, time::Duration, thread::{self}, collections::HashMap};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::{
        entities::test_value::Value, 
        stuff::{random_test_values::RandomTestValues, max_test_duration::TestDuration, wait::WaitTread},
    };
    use crate::{
        core_::point::point_type::PointType, 
        conf::multi_queue_config::MultiQueueConfig, services::{multi_queue::multi_queue::MultiQueue, services::Services, service::service::Service}, 
        tests::unit::services::multi_queue::{mock_tcp_server::MockTcpServer, mock_recv_send_service::MockRecvSendService},
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
    fn test_multi_queue_subscribtions() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test_multi_queue - Static subscriptions - Single send";
        println!("{}", self_id);

        let count = 3;              // count of the MockRecvSendService & MockTcpServer instances
        let iterations = 1000;      // test data length
        let static_test_data = RandomTestValues::new(
            self_id, 
            vec![
                Value::Int(12),
            ], 
            iterations, 
        );
        let static_test_data: Vec<Value> = static_test_data.collect();
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
        let mut handles = vec![];
        let mut rs_services = vec![];
        for i in 0..count {
            let rs_service = Arc::new(Mutex::new(MockRecvSendService::new(
                format!("tread{}", i),
                "in-queue",     //MultiQueue.
                "MultiQueue.in-queue",
                services.clone(),
                static_test_data.clone(),
                None,   //Some(staticTestDataLen * count),
            )));
            services.lock().unwrap().insert(&format!("MockRecvSendService{}", i), rs_service.clone());
            rs_services.push(rs_service);
        }
        let mq_handle = mq_service.lock().unwrap().run().unwrap();
        for rs_service in &rs_services {
            let h = rs_service.lock().unwrap().run().unwrap();
            handles.push(h);
        }
        println!("All MockRecvSendService threads - finished");
        let mut tcp_server_services: Vec<Arc<Mutex<MockTcpServer>>> = vec![];
        let mut dynamic_target: HashMap<i32, usize> = HashMap::new();
        for i in 0..count {
            let point_content = format!("dynamic{}", i);
            let dynamic_test_data = RandomTestValues::new(
                self_id, 
                vec![
                    Value::String(String::from(&point_content)),
                ], 
                iterations, 
            );
            let dynamic_test_data: Vec<Value> = dynamic_test_data.collect();
            let tcp_server_service = Arc::new(Mutex::new(MockTcpServer::new(
                format!("tread{}", i),
                "MultiQueue.in-queue",
                services.clone(),
                dynamic_test_data.clone(),
                None,
            )));
            services.lock().unwrap().insert(&format!("MockTcpServer{}", i), tcp_server_service.clone());
            let thd_handle = tcp_server_service.lock().unwrap().run().unwrap();
            thd_handle.wait().unwrap();
            thread::sleep(Duration::from_millis(100));
            let target = 0;
            let result = points_count(tcp_server_service.lock().unwrap().received().lock().unwrap().iter(), &point_content);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            for rs_service in &rs_services {
                let result = rs_service.lock().unwrap().received().lock().unwrap().len();
                println!("Static service Received( {} ): {}", rs_service.lock().unwrap().id(), result);
                // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            }
            for (index, tcp_server_service) in tcp_server_services.iter().enumerate() {
                let result = points_count(tcp_server_service.lock().unwrap().received().lock().unwrap().iter(), &point_content);
                println!("Dynamic service Received( {} ): {}", tcp_server_service.lock().unwrap().id(), result);
                // let result = tcpServerService.lock().unwrap().received().lock().unwrap().len();
                match dynamic_target.get_mut(&(index as i32)) {
                    Some(value) => {
                        *value += iterations;
                    },
                    None => {
                        panic!("index {} - not found", index)
                    },
                };
                let target = dynamic_target.get(&(index as i32)).unwrap();
                assert!(&result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            }
            println!("Dynamic service Received( {} ): {}", tcp_server_service.lock().unwrap().id(), result);
            tcp_server_services.push(tcp_server_service.clone());
            dynamic_target.insert(i, 0);
        }
        for rs_srvice in rs_services {
            rs_srvice.lock().unwrap().exit();
        }
        for tcp_server_service in tcp_server_services {
            tcp_server_service.lock().unwrap().exit();
        }
        for thd in handles {
            thd.wait().unwrap();
        }
        mq_service.lock().unwrap().exit();
        mq_handle.wait().unwrap();
        test_duration.exit();
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }
    ///
    /// 
    fn points_count<'a, T>(points: T, value: &str) -> usize 
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

#[cfg(test)]

mod profinet_client {
    use chrono::Utc;
    use log::{debug, warn};
    use std::{sync::{Arc, Mutex, Once}, thread, time::Duration};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{conf::{multi_queue_config::MultiQueueConfig, point_config::point_name::PointName, profinet_client_config::profinet_client_config::ProfinetClientConfig}, core_::{aprox_eq::aprox_eq::AproxEq, cot::cot::Cot, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status}, services::{multi_queue::multi_queue::MultiQueue, profinet_client::profinet_client::ProfinetClient, service::service::Service, services::Services}}; 
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
    /// 
    #[test]
    #[ignore = "Integration test"]
    fn basic() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue: queue
        "#.to_string();
        let conf = serde_yaml::from_str(&conf).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&conf);
        let mq_service = Arc::new(Mutex::new(MultiQueue::new(self_id, mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());
        let path = "./src/tests/unit/services/profinet_client/profinet_client.yaml";
        let conf = ProfinetClientConfig::read(path);
        debug!("config: {:?}", &conf);
        debug!("config points:");
        let client = Arc::new(Mutex::new(ProfinetClient::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("ProfinetClient", client.clone());
        let mq_service_handle = mq_service.lock().unwrap().run().unwrap();
        let client_handle = client.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(2000));
        let tx_id = PointTxId::fromStr(self_id);
        let test_data = [
            Value::Int(1),
            Value::Int(2),
            Value::Int(3),
            Value::Float(0.00101),
            Value::Float(0.00201),
            Value::Float(0.10201),
            Value::Float(9.10201),
        ];
        let send = mq_service.lock().unwrap().get_link("in-queue");
        let (_, recv) = mq_service.lock().unwrap().subscribe(self_id, &[]);
        for value in test_data {
            let point = match value {
                Value::Bool(value) => panic!("{} | Bool does not supported: {:?}", self_id, value),
                Value::Int(value) => {
                    PointType::Int(Point::new(tx_id, &PointName::new("/Ied01/db999/", "Capacitor.Capacity").full(), value, Status::Ok, Cot::Act, Utc::now()))
                },
                Value::Float(value) => {
                    PointType::Float(Point::new(tx_id, &PointName::new("/Ied01/db899/", "Drive.Speed").full(), value, Status::Ok, Cot::Act, Utc::now()))
                },
                Value::String(value) => panic!("{} | String does not supported: {:?}", self_id, value),
            };
            if let Err(err) = send.send(point.clone()) {
                warn!("{} | Send error: {:#?}", self_id, err);
            }
            match recv.recv_timeout(Duration::from_secs(3)) {
                Ok(received_point) => {
                    if received_point.cot() == Cot::Inf {
                        match received_point {
                            PointType::Bool(value) => {
                                panic!("{} | Bool does not supported: {:?}", self_id, value)
                            },
                            PointType::Int(received_point) => {
                                let result = received_point.value;
                                let target = point.as_int().value;
                                assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
                            },
                            PointType::Float(received_point) => {
                                let result = received_point.value;
                                let target = point.as_float().value;
                                assert!(result.aprox_eq(target, 3), "\nresult: {:?}\ntarget: {:?}", result, target);
                            },
                            PointType::String(value) => {
                                panic!("{} | Bool does not supported: {:?}", self_id, value)
                            },
                        }
                    }
                },
                Err(err) => {
                    warn!("{} | Receive changed value error: {:#?}", self_id, err);
                },
            }
        }
        // thread::sleep(Duration::from_millis(3000));
        client.lock().unwrap().exit();
        mq_service.lock().unwrap().exit();
        client_handle.wait().unwrap();
        mq_service_handle.wait().unwrap();
        test_duration.exit();
    }
}

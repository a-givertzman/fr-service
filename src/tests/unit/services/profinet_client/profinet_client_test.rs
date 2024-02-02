#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::{Arc, Mutex, Once}, thread, time::{Duration, Instant}};
    use crate::{conf::{multi_queue_config::MultiQueueConfig, point_config::{point_config::PointConfig, point_config_type::PointConfigType}, profinet_client_config::profinet_client_config::ProfinetClientConfig}, core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        testing::test_stuff::{max_test_duration::TestDuration, wait::WaitTread},
    }, services::{multi_queue::multi_queue::MultiQueue, profinet_client::profinet_client::ProfinetClient, service::Service, services::Services}}; 
    
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
    fn profinet_client() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test ProfinetClient";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue: queue
        "#.to_string();
        let conf = serde_yaml::from_str(&conf).unwrap();
        let mqConf = MultiQueueConfig::fromYamlValue(&conf);
        let mqService = Arc::new(Mutex::new(MultiQueue::new(selfId, mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let path = "./src/tests/unit/services/profinet_client/profinet_client.yaml";
        let conf = ProfinetClientConfig::read(path);
        debug!("config: {:?}", &conf);
        debug!("config points:");

        let client = Arc::new(Mutex::new(ProfinetClient::new(selfId, conf, services.clone())));
        services.lock().unwrap().insert("ProfinetClient", client.clone());

        mqService.lock().unwrap().run().unwrap();
        let clientHandle = client.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(3000));
        client.lock().unwrap().exit();
        clientHandle.wait().unwrap();
        // let targetPoints = [
        //     PointConfig { name: String::from("Drive.Speed"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("Drive.OutputVoltage"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("Drive.DCVoltage"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("Drive.Current"), _type: PointConfigType::Float, history: Some(1), alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("Drive.Torque"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("Drive.positionFromMru"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("Drive.positionFromHoist"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("Capacitor.Capacity"), _type: PointConfigType::Int, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("ChargeIn.On"), _type: PointConfigType::Bool, history: None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: String::from("ChargeOut.On"), _type: PointConfigType::Bool, history: None, alarm: None, address: None, filters: None, comment: None },
        // ];
        // let configPoints = config.points();
        // for point in &configPoints {
        //     println!("\t {:?}", point);
        // }
        // for target in &targetPoints {
        //     let result = configPoints.iter().find(|point| {
        //         point.name == target.name
        //     });
        //     assert!(result.is_some(), "result points does not contains '{}'", target.name);
        //     let result = result.unwrap();
        //     assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        // }
        // let result = config.points().len();
        // let target = targetPoints.len();
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }
}

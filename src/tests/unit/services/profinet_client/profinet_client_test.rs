#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::debug;
    use std::{sync::{Arc, Mutex, Once}, thread, time::Duration};
    use testing::stuff::{max_test_duration::TestDuration, wait::WaitTread};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{conf::{multi_queue_config::MultiQueueConfig, profinet_client_config::profinet_client_config::ProfinetClientConfig}, services::{multi_queue::multi_queue::MultiQueue, profinet_client::profinet_client::ProfinetClient, service::service::Service, services::Services}}; 
    
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
    fn profinet_client() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test ProfinetClient";
        println!("{}", self_id);
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
        let mqConf = MultiQueueConfig::from_yaml(&conf);
        let mqService = Arc::new(Mutex::new(MultiQueue::new(self_id, mqConf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mqService.clone());

        let path = "./src/tests/unit/services/profinet_client/profinet_client.yaml";
        let conf = ProfinetClientConfig::read(path);
        debug!("config: {:?}", &conf);
        debug!("config points:");

        let client = Arc::new(Mutex::new(ProfinetClient::new(self_id, conf, services.clone())));
        services.lock().unwrap().insert("ProfinetClient", client.clone());

        mqService.lock().unwrap().run().unwrap();
        let clientHandle = client.lock().unwrap().run().unwrap();
        thread::sleep(Duration::from_millis(3000));
        client.lock().unwrap().exit();
        mqService.lock().unwrap().exit();
        clientHandle.wait().unwrap();
        // let targetPoints = [
        //     PointConfig { name: format!("Drive.Speed"), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("Drive.OutputVoltage"), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("Drive.DCVoltage"), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("Drive.Current"), _type: PointConfigType::Float, history: Some(1), alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("Drive.Torque"), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("Drive.positionFromMru"), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("Drive.positionFromHoist"), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("Capacitor.Capacity"), _type: PointConfigType::Int, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("ChargeIn.On"), _type: PointConfigType::Bool, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        //     PointConfig { name: format!("ChargeOut.On"), _type: PointConfigType::Bool, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
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
        test_duration.exit();
    }
}

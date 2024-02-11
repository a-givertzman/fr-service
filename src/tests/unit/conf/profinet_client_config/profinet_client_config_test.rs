#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::debug;
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::conf::{point_config::{point_config::PointConfig, point_config_type::PointConfigType}, profinet_client_config::profinet_client_config::ProfinetClientConfig}; 
    
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
    fn profinet_clientConfig() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test ProfinetClientConfig";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let path = "./src/tests/unit/conf/profinet_client_config/profinet_client.yaml";
        let config = ProfinetClientConfig::read(path);
        debug!("config: {:?}", &config);
        debug!("config points:");
        let targetPoints = [
            PointConfig { name: String::from("Drive.Speed"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.OutputVoltage"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.DCVoltage"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.Current"), _type: PointConfigType::Float, history: Some(1), alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.Torque"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.positionFromMru"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.positionFromHoist"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Capacitor.Capacity"), _type: PointConfigType::Int, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("ChargeIn.On"), _type: PointConfigType::Bool, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("ChargeOut.On"), _type: PointConfigType::Bool, history: None, alarm: None, address: None, filters: None, comment: None },
        ];
        let configPoints = config.points();
        for point in &configPoints {
            println!("\t {:?}", point);
        }
        for target in &targetPoints {
            let result = configPoints.iter().find(|point| {
                point.name == target.name
            });
            assert!(result.is_some(), "result points does not contains '{}'", target.name);
            let result = result.unwrap();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        let result = config.points().len();
        let target = targetPoints.len();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }
}

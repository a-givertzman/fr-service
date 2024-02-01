#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use crate::{conf::{point_config::{point_config::PointConfig, point_config_type::PointConfigType}, profinet_client_config::profinet_client_config::ProfinetClientConfig}, core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        testing::test_stuff::max_test_duration::TestDuration,
    }}; 
    
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
        let path = "./src/tests/unit/services/profinet_client/profinet_client.yaml";
        let config = ProfinetClientConfig::read(path);
        debug!("config: {:?}", &config);
        debug!("config points:");

        let mut targetPoints = [
            PointConfig { name: String::from("Drive.Speed"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.OutputVoltage"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.DCVoltage"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.Current"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.Torque"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.positionFromMru"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Drive.positionFromHoist"), _type: PointConfigType::Float, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("Capacitor.Capacity"), _type: PointConfigType::Int, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("ChargeIn.On"), _type: PointConfigType::Bool, history: None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: String::from("ChargeOut.On"), _type: PointConfigType::Bool, history: None, alarm: None, address: None, filters: None, comment: None },
        ];
        for point in config.points() {
            println!("\t {:?}", point);
            let contains = targetPoints.iter().any(|p| {
                p.name == point.name
            });
            assert!(contains == true, "\nresult: {:?}\ntarget: {:?}", contains, true);
        }

        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }
}

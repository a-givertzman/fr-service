#[cfg(test)]

mod tests {
    use log::debug;
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::conf::{point_config::{point_config::PointConfig, point_config_history::PointConfigHistory, point_config_type::PointConfigType}, profinet_client_config::profinet_client_config::ProfinetClientConfig}; 
    
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
    fn profinet_client_config() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_name = "Ied01";
        let self_id = "test ProfinetClientConfig";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let path = "./src/tests/unit/conf/profinet_client_config/profinet_client.yaml";
        let config = ProfinetClientConfig::read(path);
        let target_points = [
            // 222
            PointConfig { name: format!("/{}/db222/Drive.Speed", self_name), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db222/Drive.OutputVoltage", self_name), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db222/Drive.DCVoltage", self_name), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db222/Drive.Current", self_name), _type: PointConfigType::Float, history: PointConfigHistory::Read, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db222/Drive.Torque", self_name), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            // 999
            PointConfig { name: format!("/{}/db999/Drive.positionFromMru", self_name), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db999/Drive.positionFromHoist", self_name), _type: PointConfigType::Float, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db999/Capacitor.Capacity", self_name), _type: PointConfigType::Int, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db999/ChargeIn.On", self_name), _type: PointConfigType::Bool, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
            PointConfig { name: format!("/{}/db999/ChargeOut.On", self_name), _type: PointConfigType::Bool, history: PointConfigHistory::None, alarm: None, address: None, filters: None, comment: None },
        ];
        debug!("result config: {:?}", &config);
        debug!("result points:");
        let config_points = config.points();
        for point in &config_points {
            println!("\t {:?}", point);
        }
        for target in &target_points {
            let result = config_points.iter().find(|point| {
                point.name == target.name
            });
            assert!(result.is_some(), "result points does not contains '{}'", target.name);
            let result = result.unwrap();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        let result = config.points().len();
        let target = target_points.len();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
}

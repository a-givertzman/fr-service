#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use crate::{core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        testing::test_stuff::max_test_duration::TestDuration, point::point_type::PointType,
    }, conf::{point_config::{PointConfig, PointConfigType, PointConfigAddress}, conf_tree::ConfTree}}; 
    
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
    fn test_point_config() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test PointConfig";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let testData = [
            (r#"
                    PointName0:
                        type: bool      # Bool / Int / Float / String / Json
                        history: 0      # 0 / 1
                        alarm: 0        # 0..15
                        address:
                            offset: 0   # 0..65535
                            bit: 0      # 0..255
                        comment: Test Point Bool"#, 
                PointConfig { 
                    name: String::from("PointName0"),
                    _type: PointConfigType::Bool, 
                    history: Some(0), alarm: Some(0), 
                    address: PointConfigAddress { offset: Some(0), bit: Some(0) }, 
                    comment: Some(String::from("Test Point Bool")),
                },
            ),
            (r#"
                    PointName1:
                        type: Int       # Bool / Int / Float / String / Json
                        history: 1      # 0 / 1
                        address:
                            offset: 0   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: String::from("PointName1"),
                    _type: PointConfigType::Int, 
                    history: Some(1), alarm: None, 
                    address: PointConfigAddress { offset: Some(0), bit: None }, 
                    comment: Some(String::from("Test Point")),
                },
            ),
            (r#"
                    PointName2:
                        type: Int       # Bool / Int / Float / String / Json
                        alarm: 4        # 0..15
                        address:
                            offset: 0   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: String::from("PointName2"),
                    _type: PointConfigType::Int, 
                    history: None, alarm: Some(4), 
                    address: PointConfigAddress { offset: Some(0), bit: None }, 
                    comment: Some(String::from("Test Point")),
                },
            ),
            (r#"
                    PointName3:
                        type: Int       # Bool / Int / Float / String / Json
                        address:
                            offset: 12   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: String::from("PointName3"),
                    _type: PointConfigType::Int, 
                    history: None, alarm: None, 
                    address: PointConfigAddress { offset: Some(12), bit: None }, 
                    comment: Some(String::from("Test Point")),
                },
            ),
        ];
        for (conf, target) in testData {
            let conf = serde_yaml::from_str(conf).unwrap();
            let result = PointConfig::fromYamlValue(&conf);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }
}

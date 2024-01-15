#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use crate::{core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        testing::test_stuff::max_test_duration::TestDuration, point::point_type::PointType,
    }, conf::point_config::{PointConfig, PointConfigType, PointConfigAddress}}; 
    
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
                    PointName:
                        type: bool      # bool / int / float / string / json
                        history: 0      # 0 / 1
                        alarm: 0        # 0..15
                        address:
                            offset: 0..65535
                            bit: 0..255
                        comment: Test Point Bool"#, 
                PointConfig { 
                    _type: PointConfigType::Bool, 
                    history: Some(0), alarm: Some(0), 
                    address: PointConfigAddress { offset: Some(0), bit: Some(0) }, 
                    comment: Some(String::from("")),
                },
            )
        ];
        for (conf, target) in testData {
            let conf = serde_yaml::from_str(conf).unwrap();
            let result = PointConfig::fromYamlValue(&conf);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }
}

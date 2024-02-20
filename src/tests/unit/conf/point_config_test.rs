#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration; 
    use crate::conf::point_config::{point_config::PointConfig, point_config_address::PointConfigAddress, point_config_filters::PointConfigFilter, point_config_history::PointConfigHistory, point_config_type::PointConfigType}; 
    
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
    fn test_point_config_serialize() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test PointConfig deserialize";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let testData = [
            (r#"
                    Point.Name.0:
                        type: Bool      # Bool / Int / Float / String / Json
                        alarm: 0        # 0..15
                        address:
                            offset: 0   # 0..65535
                            bit: 0      # 0..255
                        filters:
                            threshold: 5.0    # 5 threshold
                        comment: Test Point Bool"#, 
                PointConfig { 
                    name: format!("Point.Name.0"),
                    _type: PointConfigType::Bool, 
                    history: PointConfigHistory::None, alarm: Some(0), 
                    address: Some(PointConfigAddress { offset: Some(0), bit: Some(0) }), 
                    filters: Some(PointConfigFilter { threshold: 5.0, factor: None }),
                    comment: Some(format!("Test Point Bool")),
                },
            ),
            (r#"
                    Point.Name.0:
                        type: Bool      # Bool / Int / Float / String / Json
                        alarm: 0        # 0..15
                        address:
                            offset: 0   # 0..65535
                            bit: 0      # 0..255
                        filters:
                            threshold: 5.0    # 5 threshold
                            factor: 0.1
                        comment: Test Point Bool"#, 
                PointConfig { 
                    name: format!("Point.Name.0"),
                    _type: PointConfigType::Bool, 
                    history: PointConfigHistory::None, alarm: Some(0), 
                    address: Some(PointConfigAddress { offset: Some(0), bit: Some(0) }), 
                    filters: Some(PointConfigFilter { threshold: 5.0, factor: Some(0.1) }),
                    comment: Some(format!("Test Point Bool")),
                },
            ),
            (r#"
                    PointName1:
                        type: Int       # Bool / Int / Float / String / Json
                        history: r      # ommit - None / r - Read / w - Write / rw - ReadWrite
                        address:
                            offset: 0   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: format!("PointName1"),
                    _type: PointConfigType::Int, 
                    history: PointConfigHistory::Read, alarm: None, 
                    address: Some(PointConfigAddress { offset: Some(0), bit: None }), 
                    filters: None,
                    comment: Some(format!("Test Point")),
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
                    name: format!("PointName2"),
                    _type: PointConfigType::Int, 
                    history: PointConfigHistory::None, alarm: Some(4), 
                    address: Some(PointConfigAddress { offset: Some(0), bit: None }), 
                    filters: None,
                    comment: Some(format!("Test Point")),
                },
            ),
            (r#"
                    PointName3:
                        type: Int       # Bool / Int / Float / String / Json
                        history: w      # ommit - None / r - Read / w - Write / rw - ReadWrite
                        address:
                            offset: 12   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: format!("PointName3"),
                    _type: PointConfigType::Int, 
                    history: PointConfigHistory::Write, alarm: None, 
                    address: Some(PointConfigAddress { offset: Some(12), bit: None }), 
                    filters: None,
                    comment: Some(format!("Test Point")),
                },
            ),
            (r#"
                    PointName4:
                        type: Int       # Bool / Int / Float / String / Json
                        history: rw     # ommit - None / r - Read / w - Write / rw - ReadWrite
                        address:
                            offset: 12   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: format!("PointName4"),
                    _type: PointConfigType::Int, 
                    history: PointConfigHistory::ReadWrite, alarm: None, 
                    address: Some(PointConfigAddress { offset: Some(12), bit: None }), 
                    filters: None,
                    comment: Some(format!("Test Point")),
                },
            ),            
        ];
        for (target, conf) in testData {
            let target: serde_yaml::Value = serde_yaml::from_str(target).unwrap();
            let result = conf.asYaml();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }

    
    #[test]
    fn test_point_config_deserialize() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test PointConfig serialize";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let testData = [
            (r#"
                    PointName0:
                        type: bool      # Bool / Int / Float / String / Json
                        history: rw      # None / Read / Write
                        alarm: 0        # 0..15
                        address:
                            offset: 0   # 0..65535
                            bit: 0      # 0..255
                        filters:
                            threshold: 5    # 5% threshold
                        comment: Test Point Bool"#, 
                PointConfig { 
                    name: format!("{}/PointName0", selfId),
                    _type: PointConfigType::Bool, 
                    history: PointConfigHistory::ReadWrite, alarm: Some(0), 
                    address: Some(PointConfigAddress { offset: Some(0), bit: Some(0) }), 
                    filters: Some(PointConfigFilter { threshold: 5.0, factor: None }),
                    comment: Some(format!("Test Point Bool")),
                },
            ),
            (r#"
                    PointName1:
                        type: Int       # Bool / Int / Float / String / Json
                        history: w      # None / Read / Write
                        address:
                            offset: 0   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: format!("{}/PointName1", selfId),
                    _type: PointConfigType::Int, 
                    history: PointConfigHistory::Write, alarm: None, 
                    address: Some(PointConfigAddress { offset: Some(0), bit: None }), 
                    filters: None,
                    comment: Some(format!("Test Point")),
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
                    name: format!("{}/PointName2", selfId),
                    _type: PointConfigType::Int, 
                    history: PointConfigHistory::None, alarm: Some(4), 
                    address: Some(PointConfigAddress { offset: Some(0), bit: None }), 
                    filters: None,
                    comment: Some(format!("Test Point")),
                },
            ),
            (r#"
                    PointName3:
                        type: Int       # Bool / Int / Float / String / Json
                        address:
                            offset: 12   # 0..65535
                        comment: Test Point"#, 
                PointConfig { 
                    name: format!("{}/PointName3", selfId),
                    _type: PointConfigType::Int, 
                    history: PointConfigHistory::None, alarm: None, 
                    address: Some(PointConfigAddress { offset: Some(12), bit: None }), 
                    filters: None,
                    comment: Some(format!("Test Point")),
                },
            ),
        ];
        for (conf, target) in testData {
            let conf = serde_yaml::from_str(conf).unwrap();
            let result = PointConfig::fromYamlValue(selfId, &conf);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }
}

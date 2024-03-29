#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::debug;
    use std::sync::Once;
    use indexmap::IndexMap;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::{
        point_config::{point_config::PointConfig, point_config_history::PointConfigHistory, point_config_type::PointConfigType},
        fn_::{fn_conf_keywd::FnConfPointType, fn_conf_kind::FnConfKind, fn_config::FnConfig, fn_point_config::FnPointConfig},
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
    fn test_fn_config_point() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test FnConfig | point";
        println!("\n{}", self_id);
        let testPoint1 = PointConfig {
            name: format!("/{}/CraneMovement.BoomUp", self_id),
            _type: PointConfigType::Int,
            history: PointConfigHistory::None,
            alarm: None,
            address: None,
            filters: None,
            comment: Some("Some indication".to_string()),
        };
        let testPoint2 = PointConfig {
            name: format!("/{}/CraneMovement.BoomDown", self_id),
            _type: PointConfigType::Float,
            history: PointConfigHistory::Read,
            alarm: None,
            address: None,
            filters: None,
            comment: Some("Some indication".to_string()),
        };
        let test_data = [
            (
                vec![],
                r#"let newVar:
                    input: const '13.55'
                "#, 
                FnConfKind::Var(
                    FnConfig { name: "newVar".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        ("input".to_string(), FnConfKind::Const( FnConfig { name: "13.55".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::new() })),
                    ]) }
                ),
            ),
            (
                vec![testPoint1.clone(), testPoint2.clone()],
                r#"
                    fn ToMultiQueue:
                        in1 point CraneMovement.BoomUp: 
                            type: 'Int'
                            comment: 'Some indication'
                            input:
                                const float 0.05
                        in2 point CraneMovement.BoomDown: 
                            type: 'Float'
                            history: r
                            comment: 'Some indication'
                            input:
                                const float 0.07
                "#,
                FnConfKind::Fn( FnConfig { name: "ToMultiQueue".to_string(), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                    ("in1".to_string(), FnConfKind::PointConf( FnPointConfig { 
                        conf: testPoint1,
                        input: Box::new(FnConfKind::Const( FnConfig { name: "0.05".to_string(), type_: FnConfPointType::Float, inputs: IndexMap::new()} )),
                    })),
                    ("in2".to_string(), FnConfKind::PointConf( FnPointConfig { 
                            conf: testPoint2,
                            input: Box::new(FnConfKind::Const( FnConfig { name: "0.07".to_string(), type_: FnConfPointType::Float, inputs: IndexMap::new()} )),
                        })),
                ]) } ),
            ),
        ];
        for (pointsTarget, value, target) in test_data {
            debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
            debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
            let mut vars = vec![];
            let fnConfig = FnConfig::from_yaml(self_id, &conf, &mut vars);
            let points = fnConfig.points();
            debug!("\tfnConfig: {:?}", fnConfig);
            debug!("\tpoints: {:?}", points);
            assert_eq!(fnConfig, target);
            assert_eq!(points, pointsTarget);
        }
    }
}

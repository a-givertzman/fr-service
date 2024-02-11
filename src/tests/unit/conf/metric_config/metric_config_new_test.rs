#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{info, debug};
    use std::sync::Once;
    use indexmap::IndexMap;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, metric_config::MetricConfig, fn_conf_keywd::FnConfPointType};
    
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
    fn test_metric_config_new_valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_metric_config_new_valid");
        // let (initial, switches) = initEach();
        let testData = [
            // (
            //     r#"fn SqlMetric:
            //         table: "table_name"
            //         sql: "select * from {table}"
            //         inputs:
            //             input: const '13.55'
            //     "#, 
            //     FnConfig { fnType: FnConfigType::Var, name: "newVar".to_string(), inputs: IndexMap::from([
            //         ("input".to_string(), FnConfig { fnType: FnConfigType::Const, name: "13.55".to_string(), inputs: IndexMap::new() }),
            //     ]) }
            // ),
            (
                r#"fn SqlMetric:
                    initial: 0.123      # начальное значение
                    table: table_name
                    sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"    
                    inputs:
                        input1:
                            let VarName2:
                                input fn functionName:
                                    initial: VarName2
                                    input11 fn functionName:
                                        input1: const someValue
                                        input2: point int '/path/Point.Name/'
                                        input fn functionName:
                                            input: point bool '/path/Point.Name/'
                        input2:
                            const 1
                "#, 
                MetricConfig { 
                    name: String::from("SqlMetric"), 
                    table: String::from("table_name"), 
                    sql: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
                    initial: 0.123, 
                    vars: vec![String::from("VarName2")],
                    inputs: IndexMap::from([
                        (String::from("input1"), FnConfKind::Var( FnConfig { 
                            name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                (String::from("input"), FnConfKind::Fn( FnConfig { 
                                    name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                        (String::from("initial"), FnConfKind::Var( FnConfig { name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                        (String::from("input11"), FnConfKind::Fn( FnConfig { 
                                            name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                (String::from("input1"), FnConfKind::Const( FnConfig { name: String::from("someValue"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                                (String::from("input2"), FnConfKind::Point( FnConfig { name: String::from("/path/Point.Name/"), type_: FnConfPointType::Int, inputs: IndexMap::new() } )), 
                                                (String::from("input"), FnConfKind::Fn( FnConfig { 
                                                    name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                        (String::from("input"), FnConfKind::Point( FnConfig { name: String::from("/path/Point.Name/"), type_: FnConfPointType::Bool, inputs: IndexMap::new() } )),
                                                    ])
                                                } )), 
                                            ]),
                                        } )),
                                    ]), 
                                } )),
                            ]),
                        } )), 
                        (String::from("input2"), FnConfKind::Const( FnConfig { name: String::from("1"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                    ]), 
                },
            ),
        ];
        for (value, target) in testData {
            debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
    
            debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
            // let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
            // debug!("\tfnKeyword: {:?}", fnKeyword);
            let mut vars = vec![];
            let fnConfig = MetricConfig::fromYamlValue(&conf, &mut vars);
            debug!("\tfnConfig: {:?}", fnConfig);
            assert_eq!(fnConfig, target);
        }
    }
}
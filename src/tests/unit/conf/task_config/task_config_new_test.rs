#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use indexmap::IndexMap;
    use log::{info, debug};
    use std::{sync::Once, time::Duration};
    
    use crate::{
        core_::debug::debug_session::*, 
        conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, fn_conf_keywd::FnConfPointType, task_config::TaskConfig}, 
    };
    
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
    fn test_fn_config_new_valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_config_new_valid");
        // let (initial, switches) = initEach();
        let testData = [
            // (
            //     r#"metric sqlSelectMetric:
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
                r#"task task1:
                    cycle: 100 ms
                    recv-queue: recv-queue
                    metric sqlSelectMetric:
                        initial: 0.123      # начальное значение
                        table: table_name
                        sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"    
                        input1 let VarName2:
                            input fn functionName:
                                initial: VarName2
                                input fn functionName:
                                    input1: const someValue
                                    input2: point float '/path/Point.Name'
                                    input fn functionName:
                                        input: point bool '/path/Point.Name'
                        input2:
                            const int 1
                "#, 
                TaskConfig {
                    name: String::from("task1"),
                    cycle: Some(Duration::from_millis(100)),
                    recvQueue: String::from("recv-queue"),
                    vars: vec![String::from("VarName2")],
                    nodes: IndexMap::from([                    
                        (String::from("sqlSelectMetric-1"), FnConfig { 
                                fnKind: FnConfKind::Metric,
                                type_: FnConfPointType::Unknown,
                                name: String::from("sqlSelectMetric"), 
                                // vars: vec![String::from("VarName2")],
                                inputs: IndexMap::from([
                                    (String::from("initial"), FnConfig { fnKind: FnConfKind::Param, name: String::from("0.123"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                    (String::from("table"), FnConfig { fnKind: FnConfKind::Param, name: String::from("table_name"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                    (String::from("sql"), FnConfig { fnKind: FnConfKind::Param, name: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                    (String::from("input1"), FnConfig { 
                                        fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                            (String::from("input"), FnConfig { 
                                                fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                    (String::from("initial"), FnConfig { fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                                    (String::from("input"), FnConfig { 
                                                        fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                            (String::from("input1"), FnConfig { fnKind: FnConfKind::Const, name: String::from("someValue"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                                            (String::from("input2"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name"), type_: FnConfPointType::Float, inputs: IndexMap::new() }), 
                                                            (String::from("input"), FnConfig { 
                                                                fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                                    (String::from("input"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name"), type_: FnConfPointType::Bool, inputs: IndexMap::new() }),
                                                                ])
                                                            }), 
                                                        ]) 
                                                    }),
                                                ]) 
                                            })
                                        ]) 
                                    }), 
                                    (String::from("input2"), FnConfig { fnKind: FnConfKind::Const, name: String::from("1"), type_: FnConfPointType::Int, inputs: IndexMap::new() })
                                ]), 
                            }
                        ),
                    ])
                }
            ),
            (
                r#"task task1:
                    cycle: 100 ms
                    recv-queue: recv-queue
                    let VarName2:
                        input fn functionName:
                            initial: VarName2
                            input fn functionName:
                                input1: const someValue
                                input2: point float '/path/Point.Name'
                                input fn functionName:
                                    input: point bool '/path/Point.Name'
                "#, 
                TaskConfig {
                    name: String::from("task1"),
                    cycle: Some(Duration::from_millis(100)),
                    recvQueue: String::from("recv-queue"),
                    vars: vec![String::from("VarName2")],
                    nodes: IndexMap::from([                    
                        (String::from("VarName2-1"), FnConfig { 
                            fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                (String::from("input"), FnConfig { 
                                    fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                        (String::from("initial"), FnConfig { fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                        (String::from("input"), FnConfig { 
                                            fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                (String::from("input1"), FnConfig { fnKind: FnConfKind::Const, name: String::from("someValue"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                                (String::from("input2"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name"), type_: FnConfPointType::Float, inputs: IndexMap::new() }), 
                                                (String::from("input"), FnConfig { 
                                                    fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                        (String::from("input"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name"), type_: FnConfPointType::Bool, inputs: IndexMap::new() }),
                                                    ])
                                                }), 
                                            ]) 
                                        }),
                                    ]) 
                                })
                            ]) 
                        }), 
                    ])
                }
            ),        
        ];
        for (value, target) in testData {
            debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
    
            debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
            // let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
            // debug!("\tfnKeyword: {:?}", fnKeyword);
            // let mut vars = vec![];
            let fnConfig = TaskConfig::fromYamlValue(&conf);
            debug!("\tfnConfig: {:?}", fnConfig);
            assert_eq!(fnConfig, target);
        }
    }
}
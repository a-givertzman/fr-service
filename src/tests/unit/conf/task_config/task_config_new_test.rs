#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use indexmap::IndexMap;
    use log::{info, debug};
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::{
        task_config::TaskConfig,
        fn_::{fn_config::FnConfig, fn_conf_kind::FnConfKind, fn_conf_keywd::FnConfPointType},
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
    fn test_fn_config_new_valid() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        info!("test_task_config_new_valid");
        // let (initial, switches) = init_each();
        let test_data = [
            // (
            //     r#"metric SqlMetric:
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
                    in queue recv-queue:
                        max-length: 10000
                    fn SqlMetric:
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
                    name: format!("task1"),
                    cycle: Some(Duration::from_millis(100)),
                    rx: format!("recv-queue"),
                    rxMaxLength: 10000,
                    vars: vec![format!("VarName2")],
                    nodes: IndexMap::from([                    
                        (format!("SqlMetric-1"), FnConfKind::Fn( FnConfig {
                                type_: FnConfPointType::Unknown,
                                name: format!("SqlMetric"), 
                                // vars: vec![format!("VarName2")],
                                inputs: IndexMap::from([
                                    (format!("initial"), FnConfKind::Param( format!("0.123") )),
                                    (format!("table"), FnConfKind::Param( format!("table_name") )),
                                    (format!("sql"), FnConfKind::Param( String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';") )),
                                    (format!("input1"), FnConfKind::Var( FnConfig { 
                                        name: format!("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                            (format!("input"), FnConfKind::Fn( FnConfig { 
                                                name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                    (format!("initial"), FnConfKind::Var( FnConfig { name: format!("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                                    (format!("input"), FnConfKind::Fn( FnConfig { 
                                                        name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                            (format!("input1"), FnConfKind::Const( FnConfig { name: format!("someValue"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                                            (format!("input2"), FnConfKind::Point( FnConfig { name: format!("/path/Point.Name"), type_: FnConfPointType::Float, inputs: IndexMap::new() } )), 
                                                            (format!("input"), FnConfKind::Fn( FnConfig { 
                                                                name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                                    (format!("input"), FnConfKind::Point( FnConfig { name: format!("/path/Point.Name"), type_: FnConfPointType::Bool, inputs: IndexMap::new() } )),
                                                                ])
                                                            } )), 
                                                        ]) 
                                                    } )),
                                                ]),
                                            } )),
                                        ]),
                                    } )), 
                                    (format!("input2"), FnConfKind::Const( FnConfig { name: format!("1"), type_: FnConfPointType::Int, inputs: IndexMap::new() } )),
                                ]), 
                            } ),
                        ),
                    ]),
                },
            ),
            (
                r#"task task1:
                    cycle: 100 ms
                    in queue recv-queue:
                        max-length: 10000
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
                    name: format!("task1"),
                    cycle: Some(Duration::from_millis(100)),
                    rx: format!("recv-queue"),
                    rxMaxLength: 10000,
                    vars: vec![format!("VarName2")],
                    nodes: IndexMap::from([                    
                        (format!("VarName2-1"), FnConfKind::Var( FnConfig { 
                            name: format!("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                (format!("input"), FnConfKind::Fn( FnConfig { 
                                    name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                        (format!("initial"), FnConfKind::Var( FnConfig { name: format!("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                        (format!("input"), FnConfKind::Fn( FnConfig { 
                                            name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                (format!("input1"), FnConfKind::Const( FnConfig { name: format!("someValue"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                                (format!("input2"), FnConfKind::Point( FnConfig { name: format!("/path/Point.Name"), type_: FnConfPointType::Float, inputs: IndexMap::new() } )), 
                                                (format!("input"), FnConfKind::Fn( FnConfig { 
                                                    name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                        (format!("input"), FnConfKind::Point( FnConfig { name: format!("/path/Point.Name"), type_: FnConfPointType::Bool, inputs: IndexMap::new() } )),
                                                    ])
                                                } )), 
                                            ]) 
                                        } )),
                                    ]) 
                                } ))
                            ]) 
                        } )), 
                    ])
                }
            ),        
        ];
        for (value, target) in test_data {
            debug!("test value: {:?}", value);
            let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
    
            debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
            // let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
            // debug!("\tfnKeyword: {:?}", fnKeyword);
            // let mut vars = vec![];
            let fnConfig = TaskConfig::from_yaml(&conf);
            debug!("\tfnConfig: {:?}", fnConfig);
            assert_eq!(fnConfig, target);
        }
    }
}
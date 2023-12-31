#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use indexmap::IndexMap;
    use log::{trace, info};
    use std::{sync::Once, env, time::Duration};
    
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
    fn test_fn_config_read_valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_config_read_valid");
        let target = TaskConfig {
            name: String::from("task1"),
            cycle: Some(Duration::from_millis(100)),
            rx: String::from("recv-queue"),
            rxMaxLength: 10000,
            vars: vec![String::from("VarName2")],
            nodes: IndexMap::from([                    
                (String::from("SqlMetric-1"), FnConfig { 
                        name: String::from("SqlMetric"), 
                        fnKind: FnConfKind::Metric,
                        type_: FnConfPointType::Unknown,
                        // table: String::from("table_name"), 
                        // sql: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
                        // initial: 0.123, 
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
                            (String::from("input2"), FnConfig { fnKind: FnConfKind::Const, name: String::from("1"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() })
                        ]), 
                    }
                ),
            ])
        };
        
        // let (initial, switches) = initEach();
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/conf/task_config/task_config_test.yaml";
        let metricConfig = TaskConfig::read(path);
        trace!("fnConfig: {:?}", metricConfig);
        assert_eq!(metricConfig, target);
    }
}    

#[cfg(test)]

mod task_config_read {
    use indexmap::IndexMap;
    use log::{trace, info};
    use std::{sync::Once, env, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::{
        conf_subscribe::ConfSubscribe, fn_::{fn_conf_keywd::FnConfPointType, fn_conf_kind::FnConfKind, fn_config::FnConfig}, task_config::TaskConfig
    };
    ///
    ///     
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
    fn init_each() -> () {}
    ///
    ///     
    #[test]
    fn valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        info!("test");
        let target = TaskConfig {
            name: format!("Task1"),
            cycle: Some(Duration::from_millis(100)),
            rx: format!("recv-queue"),
            rx_max_length: 10000,
            subscribe: ConfSubscribe::new(serde_yaml::Value::Null),
            vars: vec![format!("VarName2")],
            nodes: IndexMap::from([                    
                (format!("SqlMetric-1"), FnConfKind::Fn( FnConfig { 
                        name: format!("SqlMetric"), 
                        type_: FnConfPointType::Unknown,
                        // table: format!("table_name"), 
                        // sql: format!("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
                        // initial: 0.123, 
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
                                                    (format!("input2"), FnConfKind::Point( FnConfig { name: format!("/path/Point.Name"), type_: FnConfPointType::Real, inputs: IndexMap::new() } )), 
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
                            (format!("input2"), FnConfKind::Const( FnConfig { name: format!("1"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } ))
                        ]), 
                    } )
                ),
            ])
        };
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/conf/task_config/task_config_test.yaml";
        let metric_config = TaskConfig::read(path);
        trace!("fnConfig: {:?}", metric_config);
        assert_eq!(metric_config, target);
    }
}    

#![allow(non_snake_case)]
#[cfg(test)]

mod tests{
    use log::trace;
    use std::{sync::Once, env};
    use indexmap::IndexMap;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::fn_::{fn_config::FnConfig, fn_conf_kind::FnConfKind, fn_conf_keywd::FnConfPointType, metric_config::MetricConfig};
    
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
    fn test_fn_config_read_valid() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test MetricConfig | read valid";
        println!("\n{}", self_id);
        let target = MetricConfig { 
            name: format!("SqlMetric"), 
            table: format!("table_name"), 
            sql: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
            initial: 0.123, 
            vars: vec![format!("VarName2")],
            inputs: IndexMap::from([
                (format!("input1"), FnConfKind::Var( FnConfig { 
                    name: format!("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                        (format!("input"), FnConfKind::Fn( FnConfig { 
                            name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                (format!("initial"), FnConfKind::Var( FnConfig { name: format!("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                (format!("input"), FnConfKind::Fn( FnConfig { 
                                    name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                        (format!("input1"), FnConfKind::Const( FnConfig { name: format!("someValue"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } )),
                                        (format!("input2"), FnConfKind::Point( FnConfig { name: format!("/path/Point.Name/"), type_: FnConfPointType::Float, inputs: IndexMap::new() } )), 
                                        (format!("input"), FnConfKind::Fn( FnConfig { 
                                            name: format!("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                                (format!("input"), FnConfKind::Point( FnConfig { name: format!("/path/Point.Name/"), type_: FnConfPointType::Bool, inputs: IndexMap::new() } )),
                                            ]),
                                        } )), 
                                    ]),
                                } )),
                            ]),
                        } )),
                    ]),
                } )), 
                (format!("input2"), FnConfKind::Const( FnConfig { name: format!("1"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() } ))
            ]), 
        };
        
        // let (initial, switches) = init_each();
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/conf/metric_config/metric_config_test.yaml";
        let metricConfig = MetricConfig::read(self_id, path);
        trace!("fnConfig: {:?}", metricConfig);
        assert_eq!(metricConfig, target);
    }
}    

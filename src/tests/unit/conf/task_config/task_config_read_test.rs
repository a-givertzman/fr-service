#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, info};
use std::{sync::Once, env, collections::HashMap};

use crate::core_::{
    debug::debug_session::{DebugSession, LogLevel}, 
    conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType, metric_config::MetricConfig, task_config::{TaskConfig, TaskConfNode}}, 
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
    DebugSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_task_config_read_valid");
    let target = TaskConfig {
        name: String::from("task1"),
        cycle: 100,
        apiQueue: String::from("queueApi"),
        vars: vec![String::from("VarName2")],
        nodes: HashMap::from([                    
            (String::from("sqlSelectMetric-1"), FnConfig { 
                    name: String::from("sqlSelectMetric"), 
                    fnKind: FnConfKind::Metric,
                    type_: FnConfPointType::Unknown,
                    // table: String::from("table_name"), 
                    // sql: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
                    // initial: 0.123, 
                    // vars: vec![String::from("VarName2")],
                    inputs: HashMap::from([
                        (String::from("initial"), FnConfig { fnKind: FnConfKind::Param, name: String::from("0.123"), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                        (String::from("table"), FnConfig { fnKind: FnConfKind::Param, name: String::from("table_name"), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                        (String::from("sql"), FnConfig { fnKind: FnConfKind::Param, name: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                        (String::from("input1"), FnConfig { 
                            fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                                (String::from("input"), FnConfig { 
                                    fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                                        (String::from("initial"), FnConfig { fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                                        (String::from("input"), FnConfig { 
                                            fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                                                (String::from("input1"), FnConfig { fnKind: FnConfKind::Const, name: String::from("someValue"), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                                                (String::from("input2"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name/"), type_: FnConfPointType::Float, inputs: HashMap::new() }), 
                                                (String::from("input"), FnConfig { 
                                                    fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                                                        (String::from("input"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name/"), type_: FnConfPointType::Bool, inputs: HashMap::new() }),
                                                    ])
                                                }), 
                                            ]) 
                                        }),
                                    ]) 
                                })
                            ]) 
                        }), 
                        (String::from("input2"), FnConfig { fnKind: FnConfKind::Const, name: String::from("1"), type_: FnConfPointType::Unknown, inputs: HashMap::new() })
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

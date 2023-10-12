#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, info};
use std::{sync::Once, env, collections::HashMap};

use crate::core_::{conf::{fn_config::FnConfig, fn_config_type::FnConfigType, metric_config::MetricConfig, task_config::{TaskConfig, TaskNode}}, debug::debug_session::{DebugSession, LogLevel}};

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
    info!("test_fn_config_read_valid");
    let target = TaskConfig {
        name: String::from("task1"),
        cycle: 100,
        vars: vec![String::from("VarName2")],
        nodes: HashMap::from([                    
            (String::from("sqlSelectMetric-1"), TaskNode::Metric(                    
                MetricConfig { 
                    name: String::from("sqlSelectMetric"), 
                    table: String::from("table_name"), 
                    sql: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
                    initial: 0.123, 
                    vars: vec![String::from("VarName2")],
                    inputs: HashMap::from([
                        (String::from("input1"), FnConfig { 
                            fnType: FnConfigType::Var, name: String::from("VarName2"), inputs: HashMap::from([
                                (String::from("input"), FnConfig { 
                                    fnType: FnConfigType::Fn, name: String::from("functionName"), inputs: HashMap::from([
                                        (String::from("initial"), FnConfig { fnType: FnConfigType::Var, name: String::from("VarName2"), inputs: HashMap::new() }),
                                        (String::from("input"), FnConfig { 
                                            fnType: FnConfigType::Fn, name: String::from("functionName"), inputs: HashMap::from([
                                                (String::from("input1"), FnConfig { fnType: FnConfigType::Const, name: String::from("someValue"), inputs: HashMap::new() }),
                                                (String::from("input2"), FnConfig { fnType: FnConfigType::Point, name: String::from("/path/Point.Name/"), inputs: HashMap::new() }), 
                                                (String::from("input"), FnConfig { 
                                                    fnType: FnConfigType::Fn, name: String::from("functionName"), inputs: HashMap::from([
                                                        (String::from("input"), FnConfig { fnType: FnConfigType::Point, name: String::from("/path/Point.Name/"), inputs: HashMap::new() }),
                                                    ])
                                                }), 
                                            ]) 
                                        }),
                                    ]) 
                                })
                            ]) 
                        }), 
                        (String::from("input2"), FnConfig { fnType: FnConfigType::Const, name: String::from("1"), inputs: HashMap::new() })
                    ]), 
                }
            )),
        ])
    };
    
    // let (initial, switches) = initEach();
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/tests/unit/task_config/task_config_test.yaml";
    let metricConfig = TaskConfig::read(path);
    trace!("fnConfig: {:?}", metricConfig);
    assert_eq!(metricConfig, target);
}


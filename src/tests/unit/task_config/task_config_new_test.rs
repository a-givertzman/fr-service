#![allow(non_snake_case)]
#[cfg(test)]
use log::{info, debug};
use std::{sync::Once, collections::HashMap};

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
fn test_fn_config_new_valid() {
    DebugSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_metric_config_new_valid");
    // let (initial, switches) = initEach();
    let testData = [
        // (
        //     r#"metric sqlSelectMetric:
        //         table: "table_name"
        //         sql: "select * from {table}"
        //         inputs:
        //             input: const '13.55'
        //     "#, 
        //     FnConfig { fnType: FnConfigType::Var, name: "newVar".to_string(), inputs: HashMap::from([
        //         ("input".to_string(), FnConfig { fnType: FnConfigType::Const, name: "13.55".to_string(), inputs: HashMap::new() }),
        //     ]) }
        // ),
        (
            r#"task task1:
                cycle: 100
                metric sqlSelectMetric:
                    initial: 0.123      # начальное значение
                    table: table_name
                    sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"    
                    inputs:
                        input1:
                            let VarName2:
                                input: 
                                    fn functionName:
                                        initial: VarName2
                                        input: 
                                            fn functionName:
                                                input1: const someValue
                                                input2: point '/path/Point.Name/'
                                                input: 
                                                    fn functionName:
                                                        input: point '/path/Point.Name/'
                        input2:
                            const 1
            "#, 
            TaskConfig {
                name: String::from("task1"),
                cycle: 100,
                vars: vec![String::from("VarName2")],
                nodes: HashMap::from([                    
                    (String::from("task1-1"), TaskNode::Metric(                    
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
#![allow(non_snake_case)]
use log::debug;
#[cfg(test)]
use log::info;
use std::{sync::Once, collections::HashMap};

use crate::core_::{conf::{fn_config::FnConfig, fn_config_type::FnConfigType, metric_config::MetricConfig}, debug::debug_session::{DebugSession, LogLevel}};

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
            r#"metric sqlSelectMetric:
                initial: "0"      # начальное значение
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
            MetricConfig { 
                name: String::from("metric sqlSelectMetric"), 
                table: String::from("table_name"), 
                sql: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
                initial: String::from("0"), 
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
        ),
        // (
        //     serde_yaml::from_str(r#"let newVar:
        //         input1:
        //             fn count:
        //                 inputConst1: const '11.3'
        //                 inputConst2: const '12.7'"
        //         input2:
        //             fn count:
        //                 inputConst1: const '13.3'
        //                 inputConst2: const '14.7'"#
        //     ), 
        //     FnConfig { fnType: FnConfigType::Var, name: "newVar".to_string(), inputs: HashMap::from([
        //         ("input1".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "count".to_string(), inputs: HashMap::from([
        //             ("inputConst1".to_string(), FnConfig { fnType: FnConfigType::Const, name: "11.3".to_string(), inputs: HashMap::new() }),
        //             ("inputConst2".to_string(), FnConfig { fnType: FnConfigType::Const, name: "12.7".to_string(), inputs: HashMap::new() }),
        //         ]) }),
        //         ("input2".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "count".to_string(), inputs: HashMap::from([
        //             ("inputConst1".to_string(), FnConfig { fnType: FnConfigType::Const, name: "13.3".to_string(), inputs: HashMap::new() }),
        //             ("inputConst2".to_string(), FnConfig { fnType: FnConfigType::Const, name: "14.7".to_string(), inputs: HashMap::new() }),
        //         ]) }),
        //     ]) }
        // ),
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
#![allow(non_snake_case)]
use indexmap::IndexMap;
#[cfg(test)]
use log::{trace, info};
use std::{sync::Once, env};

use crate::core_::{
    debug::debug_session::*, 
    conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType, metric_config::MetricConfig},
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
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_fn_config_read_valid");
    let target = MetricConfig { 
        name: String::from("sqlSelectMetric"), 
        table: String::from("table_name"), 
        sql: String::from("UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"), 
        initial: 0.123, 
        vars: vec![String::from("VarName2")],
        inputs: IndexMap::from([
            (String::from("input1"), FnConfig { 
                fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                    (String::from("input"), FnConfig { 
                        fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                            (String::from("initial"), FnConfig { fnKind: FnConfKind::Var, name: String::from("VarName2"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                            (String::from("input"), FnConfig { 
                                fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                    (String::from("input1"), FnConfig { fnKind: FnConfKind::Const, name: String::from("someValue"), type_: FnConfPointType::Unknown, inputs: IndexMap::new() }),
                                    (String::from("input2"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name/"), type_: FnConfPointType::Float, inputs: IndexMap::new() }), 
                                    (String::from("input"), FnConfig { 
                                        fnKind: FnConfKind::Fn, name: String::from("functionName"), type_: FnConfPointType::Unknown, inputs: IndexMap::from([
                                            (String::from("input"), FnConfig { fnKind: FnConfKind::Point, name: String::from("/path/Point.Name/"), type_: FnConfPointType::Bool, inputs: IndexMap::new() }),
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
    };
    
    // let (initial, switches) = initEach();
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/tests/unit/conf/metric_config/metric_config_test.yaml";
    let metricConfig = MetricConfig::read(path);
    trace!("fnConfig: {:?}", metricConfig);
    assert_eq!(metricConfig, target);
}


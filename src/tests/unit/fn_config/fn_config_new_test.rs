#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, collections::HashMap};

use crate::core_::{conf::{fn_config::FnConfig, fn_config_type::FnConfigType}, debug::debug_session::{DebugSession, LogLevel}};

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
    info!("test_fn_config_new_valid");
    let testData = [
        (
            r#"let newVar:
                input: const '13.55'
            "#, 
            FnConfig { fnType: FnConfigType::Var, name: "newVar".to_string(), inputs: HashMap::from([
                ("input".to_string(), FnConfig { fnType: FnConfigType::Const, name: "13.55".to_string(), inputs: HashMap::new() }),
            ]) }
        ),
        (
            r#"let newVar:
                input fn count:
                    inputConst1: const '13.3'
                    inputConst2: const '13.7'
            "#, 
            FnConfig { fnType: FnConfigType::Var, name: "newVar".to_string(), inputs: HashMap::from([
                ("input".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "count".to_string(), inputs: HashMap::from([
                    ("inputConst1".to_string(), FnConfig { fnType: FnConfigType::Const, name: "13.3".to_string(), inputs: HashMap::new() }),
                    ("inputConst2".to_string(), FnConfig { fnType: FnConfigType::Const, name: "13.7".to_string(), inputs: HashMap::new() }),
                ]) }),
            ]) }
        ),
        (
            r#"let newVar:
                input1 fn count:
                    inputConst1: const '11.3'
                    inputConst2: const '12.7'"
                input2 fn count:
                    inputConst1: const '13.3'
                    inputConst2: const '14.7'
            "#, 
            FnConfig { fnType: FnConfigType::Var, name: "newVar".to_string(), inputs: HashMap::from([
                ("input1".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "count".to_string(), inputs: HashMap::from([
                    ("inputConst1".to_string(), FnConfig { fnType: FnConfigType::Const, name: "11.3".to_string(), inputs: HashMap::new() }),
                    ("inputConst2".to_string(), FnConfig { fnType: FnConfigType::Const, name: "12.7".to_string(), inputs: HashMap::new() }),
                ]) }),
                ("input2".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "count".to_string(), inputs: HashMap::from([
                    ("inputConst1".to_string(), FnConfig { fnType: FnConfigType::Const, name: "13.3".to_string(), inputs: HashMap::new() }),
                    ("inputConst2".to_string(), FnConfig { fnType: FnConfigType::Const, name: "14.7".to_string(), inputs: HashMap::new() }),
                ]) }),
            ]) }
        ),
        (
            r#"let VarName2:
                input fn functionName1:
                    initial: VarName2
                    input fn functionName2:
                        input1: const someValue
                        input2: point '/path/Point.Name/'
                        input3 fn functionName3:
                                input: point '/path/Point.Name/'
            "#,
            FnConfig { fnType: FnConfigType::Var, name: "VarName2".to_string(), inputs: HashMap::from([
                ("input".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "functionName1".to_string(), inputs: HashMap::from([
                    ("initial".to_string(), FnConfig { fnType: FnConfigType::Var, name: "VarName2".to_string(), inputs: HashMap::new() }),
                    ("input".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "functionName2".to_string(), inputs: HashMap::from([
                        ("input1".to_string(), FnConfig { fnType: FnConfigType::Const, name: "someValue".to_string(), inputs: HashMap::new() }),
                        ("input2".to_string(), FnConfig { fnType: FnConfigType::Point, name: "/path/Point.Name/".to_string(), inputs: HashMap::new() }),

                        ("input3".to_string(), FnConfig { fnType: FnConfigType::Fn, name: "functionName3".to_string(), inputs: HashMap::from([
                            ("input".to_string(), FnConfig { fnType: FnConfigType::Point, name: "/path/Point.Name/".to_string(), inputs: HashMap::new() }),
                        ]) }),
                    ])}),
                ]) }),
            ]) }
        ),
    ];
    for (value, target) in testData {
        debug!("test value: {:?}", value);
        let conf: serde_yaml::Value = serde_yaml::from_str(value).unwrap();
        debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
        let mut vars = vec![];
        let fnConfig = FnConfig::fromYamlValue(&conf, &mut vars);
        debug!("\tfnConfig: {:?}", fnConfig);
        assert_eq!(fnConfig, target);
    }
}
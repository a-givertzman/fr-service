#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, collections::HashMap};

use crate::core_::{
    debug::debug_session::{DebugSession, LogLevel}, 
    conf::{fn_config::FnConfig, fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType}, 
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
            FnConfig { fnKind: FnConfKind::Var, name: "newVar".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                ("input".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "13.55".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
            ]) }
        ),
        (
            r#"let newVar:
                input fn count:
                    inputConst1: const '13.3'
                    inputConst2: const '13.7'
            "#, 
            FnConfig { fnKind: FnConfKind::Var, name: "newVar".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                ("input".to_string(), FnConfig { fnKind: FnConfKind::Fn, name: "count".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                    ("inputConst1".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "13.3".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                    ("inputConst2".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "13.7".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                ]) }),
            ]) }
        ),
        (
            r#"let newVar:
                input1 fn count:
                    inputConst1: const '11.3'
                    inputConst2: const '12.7'"
                input2 fn count:
                    inputConst1: const float '13.3'
                    inputConst2: const int '147'
            "#, 
            FnConfig { fnKind: FnConfKind::Var, name: "newVar".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                ("input1".to_string(), FnConfig { fnKind: FnConfKind::Fn, name: "count".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                    ("inputConst1".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "11.3".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                    ("inputConst2".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "12.7".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                ]) }),
                ("input2".to_string(), FnConfig { fnKind: FnConfKind::Fn, name: "count".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                    ("inputConst1".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "13.3".to_string(), type_: FnConfPointType::Float, inputs: HashMap::new() }),
                    ("inputConst2".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "147".to_string(), type_: FnConfPointType::Int, inputs: HashMap::new() }),
                ]) }),
            ]) }
        ),
        (
            r#"let VarName2:
                input fn functionName1:
                    initial: VarName2
                    input fn functionName2:
                        input1: const someValue
                        input2: point int '/path/Point.Name/'
                        input3 fn functionName3:
                                input: point bool '/path/Point.Name/'
            "#,
            FnConfig { fnKind: FnConfKind::Var, name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                ("input".to_string(), FnConfig { fnKind: FnConfKind::Fn, name: "functionName1".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                    ("initial".to_string(), FnConfig { fnKind: FnConfKind::Var, name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                    ("input".to_string(), FnConfig { fnKind: FnConfKind::Fn, name: "functionName2".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                        ("input1".to_string(), FnConfig { fnKind: FnConfKind::Const, name: "someValue".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() }),
                        ("input2".to_string(), FnConfig { fnKind: FnConfKind::Point, name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Int, inputs: HashMap::new() }),

                        ("input3".to_string(), FnConfig { fnKind: FnConfKind::Fn, name: "functionName3".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                            ("input".to_string(), FnConfig { fnKind: FnConfKind::Point, name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Bool, inputs: HashMap::new() }),
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

#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, info};
use std::{sync::Once, env, collections::HashMap};

use crate::core_::{
    debug::debug_session::*,
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
fn test_fn_config_read_valid() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_fn_config_read_valid");
    let target = FnConfig { 
        fnKind: FnConfKind::Var, name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
            ("input".to_string(), FnConfig { 
                fnKind: FnConfKind::Fn, name: "functionName".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                    ("initial".to_string(), FnConfig { 
                        fnKind: FnConfKind::Var, name: "VarName2".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::new() 
                    }),
                    ("input".to_string(), FnConfig { 
                        fnKind: FnConfKind::Fn, name: "functionName".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                            ("input".to_string(), FnConfig { 
                                fnKind: FnConfKind::Fn, name: "functionName".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([
                                    ("input".to_string(), FnConfig { 
                                        fnKind: FnConfKind::Point, name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Bool, inputs: HashMap::from([]) 
                                    }),
                                ]) 
                            }),
                            ("input2".to_string(), FnConfig { 
                                fnKind: FnConfKind::Point, name: "/path/Point.Name/".to_string(), type_: FnConfPointType::Float, inputs: HashMap::from([]) 
                            }),
                            ("input1".to_string(), FnConfig { 
                                fnKind: FnConfKind::Const, name: "someValue".to_string(), type_: FnConfPointType::Unknown, inputs: HashMap::from([]) 
                            }),
                        ]) 
                    }),
                ]) 
            }),
        ]) 
    };
    
    // let (initial, switches) = initEach();
    trace!("dir: {:?}", env::current_dir());
    let path= "./src/tests/unit/conf/fn_config/fn_config_test.yaml";
    let fnConfig = FnConfig::read(path);
    trace!("fnConfig: {:?}", fnConfig);
    assert_eq!(fnConfig, target);
}


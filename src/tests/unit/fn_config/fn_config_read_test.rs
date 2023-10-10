#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, info};
use std::{sync::Once, env, collections::HashMap};

use crate::core_::{nested_function::{fn_config::FnConfig, fn_config_type::FnConfigType}, debug::debug_session::{DebugSession, LogLevel}};

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
    let target = FnConfig { 
        fnType: FnConfigType::Var, name: "VarName2".to_string(), inputs: HashMap::from([
            ("input".to_string(), FnConfig { 
                fnType: FnConfigType::Fn, name: "functionName".to_string(), inputs: HashMap::from([
                    ("initial".to_string(), FnConfig { 
                        fnType: FnConfigType::Var, name: "VarName2".to_string(), inputs: HashMap::new() 
                    }),
                    ("input".to_string(), FnConfig { 
                        fnType: FnConfigType::Fn, name: "functionName".to_string(), inputs: HashMap::from([
                            ("input".to_string(), FnConfig { 
                                fnType: FnConfigType::Fn, name: "functionName".to_string(), inputs: HashMap::from([
                                    ("input".to_string(), FnConfig { 
                                        fnType: FnConfigType::Point, name: "/path/Point.Name/".to_string(), inputs: HashMap::from([]) 
                                    }),
                                ]) 
                            }),
                            ("input2".to_string(), FnConfig { 
                                fnType: FnConfigType::Point, name: "/path/Point.Name/".to_string(), inputs: HashMap::from([]) 
                            }),
                            ("input1".to_string(), FnConfig { 
                                fnType: FnConfigType::Const, name: "someValue".to_string(), inputs: HashMap::from([]) 
                            }),
                        ]) 
                    }),
                ]) 
            }),
        ]) 
    };
    
    // let (initial, switches) = initEach();
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/tests/unit/fn_config/fn_config_test.yaml";
    let fnConfig = FnConfig::read(path);
    trace!("fnConfig: {:?}", fnConfig);
    assert_eq!(fnConfig, target);
}


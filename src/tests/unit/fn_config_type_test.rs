#![allow(non_snake_case)]
use log::debug;
#[cfg(test)]
use log::{trace, info};
use std::{sync::Once, env, str::FromStr, collections::HashMap};

use crate::{
    tests::unit::init::{TestSession, LogLevel},
    core::nested_function::fn_config::{FnConfig, FnConfigType, FnVarConfig, FnConstConfig, FnPointConfig},
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

// #[test]
// fn test_create_valid_simple() {
//     TestSession::init(LogLevel::Trace);
//     initOnce();
//     initEach();
//     info!("test_create_valid_simple");
//     // let (initial, switches) = initEach();
//     let testData = [
//         (serde_yaml::from_str("var newVar"), FnConfigType::Point(FnPointConfig{ value: String::from("newVar") })),
//         (serde_yaml::from_str("const '12.5'"), FnConfigType::Point(FnPointConfig{ value: String::from("12.5") })),
//         (serde_yaml::from_str("point '/path/Point.Name'"), FnConfigType::Point(FnPointConfig{ value: String::from("/path/Point.Name") })),
//     ];
//     for (value, target) in testData {
//         debug!("test value: {:?}", value);
//         let conf: serde_yaml::Value = value.unwrap();
//         // let conf = testData.get("/").unwrap();

//         debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
//         let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
//         debug!("\tfnKeyword: {:?}", fnKeyword);
//         // let fnConfigType = FnConfigType::new(&fnKeyword,  &conf).unwrap();
//         // debug!("\tfnConfigType: {:?}", fnConfigType);
//         // assert_eq!(fnConfigType, target);
//     }
// }

#[test]
fn test_create_valid_fn() {
    TestSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_create_valid_fn");
    // let (initial, switches) = initEach();
    let testData = [
        (serde_yaml::from_str(r#"let newVar:
            input:
                fn count:
                    inputConst1: const '13.5'
                    inputConst2: const '13.5'"#), FnConfig{ fnType: None, name: "".to_string(), inputs: HashMap::new() }),
        // (serde_yaml::from_str("fn count:\
        //     inputTrip:\
        //         fn trip:\
        //             input: point '/path/Point.Name'"), FnConfigType::Fn(FnConfig{ inputs: HashMap::new() })),
    ];
    for (value, target) in testData {
        debug!("test value: {:?}", value);
        let conf: serde_yaml::Value = value.unwrap();
        // let conf = testData.get("/").unwrap();

        debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
        // let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
        // debug!("\tfnKeyword: {:?}", fnKeyword);
        let fnConfig = FnConfig::new(&conf);
        debug!("\tfnConfig: {:?}", fnConfig);
        // assert_eq!(fnConfigType, target);
    }
}
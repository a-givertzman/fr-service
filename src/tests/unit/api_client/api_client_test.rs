#![allow(non_snake_case)]
#[cfg(test)]

use log::{warn, info, debug};
use std::{sync::Once, time::{Duration, Instant}};
use crate::core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, conf::api_client_config::ApiClientConfig}; 

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
fn test_task_cycle() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    println!("");
    info!("test_task_cycle");
    let path = "./src/tests/unit/api_client/api_client.yaml";
    let conf = ApiClientConfig::read(path);
    // assert!(false)
    // assert!(result == target, "result: {:?}\ntarget: {:?}", result, target);
}
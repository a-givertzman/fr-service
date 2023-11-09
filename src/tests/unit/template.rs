#![allow(non_snake_case)]
#[cfg(test)]

use log::{warn, info, debug};
use std::{sync::Once, time::{Duration, Instant}};
use crate::core_::debug::debug_session::{DebugSession, LogLevel, Backtrace}; 

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
    assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
}
#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, str::FromStr};

use crate::{
    core_::debug::debug_session::*,
    conf::{fn_conf_keywd::FnConfKeywd, conf_duration::{ConfDuration, ConfDurationUnit}}, 
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
fn test_create_valid() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    println!("test_create_valid");
    // let (initial, switches) = initEach();
    let testData = vec![
        ("111 ns"   , ConfDuration::new(111, ConfDurationUnit::Nanos)),
        ("112ns"    , ConfDuration::new(112, ConfDurationUnit::Nanos)),
        ("12  us"   , ConfDuration::new(12, ConfDurationUnit::Micros)),
        ("13 us"    , ConfDuration::new(13, ConfDurationUnit::Micros)),
        ("11  ms"   , ConfDuration::new(11, ConfDurationUnit::Millis)),
        ("10ms"     , ConfDuration::new(10, ConfDurationUnit::Millis)),
        ("5   s"    , ConfDuration::new(5, ConfDurationUnit::Secs)),
        ("4s"       , ConfDuration::new(4, ConfDurationUnit::Secs)),
        ("3"        , ConfDuration::new(3, ConfDurationUnit::Secs)),
        ("2   m"    , ConfDuration::new(2, ConfDurationUnit::Mins)),
        ("7m"       , ConfDuration::new(7, ConfDurationUnit::Mins)),
        ("8   h"    , ConfDuration::new(8, ConfDurationUnit::Hours)),
        ("9h"       , ConfDuration::new(9, ConfDurationUnit::Hours)),
    ];
    for (value, target) in testData {
        let confDuration = ConfDuration::from_str(value).unwrap();
        debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, confDuration, target);
        assert_eq!(confDuration, target);
    }
}

#[test]
fn test_create_invalid() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_create_invalid");
    // let (initial, switches) = initEach();
    let testData: Vec<(&str, Result<&str, ()>)> = vec![
        ("1nsec", Err(())),
        ("12usec", Err(())),
        ("3msec", Err(())),
        ("12sec", Err(())),
        ("1min", Err(())),
        ("2hour", Err(())),
        ("1.1ns", Err(())),
        ("12.2us", Err(())),
        ("3.1ms", Err(())),
        ("12.2s", Err(())),
        ("3.5m", Err(())),
        ("1.5h", Err(())),
    ];
    for (value, target) in testData {
        let confDuration = FnConfKeywd::from_str(value);
        debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, confDuration, target);
        assert_eq!(confDuration.is_err(), true);
    }
}

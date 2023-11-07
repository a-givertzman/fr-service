#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, str::FromStr};

use crate::core_::{conf::{conf_keywd::{ConfKeywd, FnConfKeywdValue, FnConfPointType}, conf_duration::{ConfDuration, ConfDurationUnit}}, debug::debug_session::*};

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
        ("11  ms"   , ConfDuration::new(11, ConfDurationUnit::Milles)),
        ("10ms"     , ConfDuration::new(10, ConfDurationUnit::Milles)),
        ("5   s"    , ConfDuration::new(5, ConfDurationUnit::Secs)),
        ("4s"       , ConfDuration::new(4, ConfDurationUnit::Secs)),
        ("3"        , ConfDuration::new(3, ConfDurationUnit::Secs)),
        ("2   m"    , ConfDuration::new(2, ConfDurationUnit::Mins)),
        ("7m"       , ConfDuration::new(7, ConfDurationUnit::Mins)),
        ("8   h"    , ConfDuration::new(8, ConfDurationUnit::Hours)),
        ("9h"       , ConfDuration::new(9, ConfDurationUnit::Hours)),
    ];
    for (value, target) in testData {
        let fnConfigType = ConfDuration::from_str(value).unwrap();
        debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, fnConfigType, target);
        assert_eq!(fnConfigType, target);
    }
}

// #[test]
fn test_create_invalid() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_create_invalid");
    // let (initial, switches) = initEach();
    let testData: Vec<(&str, Result<&str, ()>)> = vec![
        ("fn:name", Err(())),
        ("fn\nname", Err(())),
        ("fn: name", Err(())),
        ("fn :name", Err(())),
        ("fn : name", Err(())),
        ("Fn name", Err(())),
        ("FN name", Err(())),
        ("fnName", Err(())),
        ("fn_name", Err(())),
        ("let:name", Err(())),
        ("Let name", Err(())),
        ("LET name", Err(())),
        ("letName", Err(())),
        ("let_name", Err(())),
        ("const:name", Err(())),
        ("Const name", Err(())),
        ("CONST name", Err(())),
        ("constName", Err(())),
        ("const_name", Err(())),
        ("point:name", Err(())),
        ("Point name", Err(())),
        ("POINT name", Err(())),
        ("pointName", Err(())),
        ("point_name", Err(())),
    ];
    for (value, target) in testData {
        let fnConfigType = ConfKeywd::from_str(value);
        debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, fnConfigType, target);
        assert_eq!(fnConfigType.is_err(), true);
    }
}

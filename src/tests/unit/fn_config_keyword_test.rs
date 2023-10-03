#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, debug, info};
use std::{sync::Once, str::FromStr};

use crate::{
    tests::unit::init::{TestSession, LogLevel},
    core::nested_function::fn_conf_keywd::FnConfKeywd,
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
    TestSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_create_valid");
    // let (initial, switches) = initEach();
    let testData = vec![
        ("fn name", FnConfKeywd::Fn(String::from("name"))),
        ("fn  name", FnConfKeywd::Fn(String::from("name"))),
        ("fn   name", FnConfKeywd::Fn(String::from("name"))),
        ("fn\tname", FnConfKeywd::Fn(String::from("name"))),
        ("let name", FnConfKeywd::Var(String::from("name"))),
        ("const name", FnConfKeywd::Const(String::from("name"))),
        ("point /path/Point.Name", FnConfKeywd::Point(String::from("/path/Point.Name"))),
        ("point '/path/Point.Name'", FnConfKeywd::Point(String::from("/path/Point.Name"))),
        ("point \"/path/Point.Name\"", FnConfKeywd::Point(String::from("/path/Point.Name"))),
    ];
    for (value, target) in testData {
        let fnConfigType = FnConfKeywd::from_str(value).unwrap();
        debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, fnConfigType, target);
        assert_eq!(fnConfigType, target);
    }
}

#[test]
fn test_create_invalid() {
    TestSession::init(LogLevel::Trace);
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
        let fnConfigType = FnConfKeywd::from_str(value);
        debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, fnConfigType, target);
        assert_eq!(fnConfigType.is_err(), true);
    }
}

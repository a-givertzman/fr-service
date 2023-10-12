#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, str::FromStr};

use crate::core_::{conf::conf_keywd::{ConfKeywd, FnConfKeywdValue}, debug::debug_session::{DebugSession, LogLevel}};

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
    DebugSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    println!("test_create_valid");
    // let (initial, switches) = initEach();
    let testData = vec![
        ("input1 fn fnName", ConfKeywd::Fn( FnConfKeywdValue {input: String::from("input1"), name: String::from("fnName")} )),
        ("fn name", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), name: String::from("name")} )),
        ("fn  name", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), name: String::from("name")} )),
        ("fn   name", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), name: String::from("name")} )),
        ("fn\tname", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), name: String::from("name")} )),
        ("let name", ConfKeywd::Var( FnConfKeywdValue {input: String::new(), name: String::from("name")} )),
        ("input1 const", ConfKeywd::Const( FnConfKeywdValue {input: String::from("input1"), name: String::new()} )),
        ("const name", ConfKeywd::Const( FnConfKeywdValue {input: String::new(), name: String::from("name")} )),
        ("input2 const name", ConfKeywd::Const( FnConfKeywdValue {input: String::from("input2"), name: String::from("name")} )),
        ("point /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::new(), name: String::from("/path/Point.Name")} )),
        ("point '/path/Point.Name'", ConfKeywd::Point( FnConfKeywdValue {input: String::new(), name: String::from("/path/Point.Name")} )),
        ("point \"/path/Point.Name\"", ConfKeywd::Point( FnConfKeywdValue {input: String::new(), name: String::from("/path/Point.Name")} )),
        ("input1 point /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input1"), name: String::from("/path/Point.Name")} )),
        ("input2 point '/path/Point.Name'", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input2"), name: String::from("/path/Point.Name")} )),
        ("input3 point \"/path/Point.Name\"", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input3"), name: String::from("/path/Point.Name")} )),
    ];
    for (value, target) in testData {
        let fnConfigType = ConfKeywd::from_str(value).unwrap();
        debug!("value: {:?}   |   fnConfigType: {:?}   |   target: {:?}", value, fnConfigType, target);
        assert_eq!(fnConfigType, target);
    }
}

#[test]
fn test_create_invalid() {
    DebugSession::init(LogLevel::Trace);
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

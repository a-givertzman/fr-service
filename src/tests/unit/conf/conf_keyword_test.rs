#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, str::FromStr};

use crate::core_::{conf::conf_keywd::{ConfKeywd, FnConfKeywdValue, FnConfPointType}, debug::debug_session::{DebugSession, LogLevel}};

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
        ("input1 fn fnName", ConfKeywd::Fn( FnConfKeywdValue {input: String::from("input1"), type_: FnConfPointType::Unknown, data: String::from("fnName")} )),
        ("fn name", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("name")} )),
        ("fn  name", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("name")} )),
        ("fn   name", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("name")} )),
        ("fn\tname", ConfKeywd::Fn( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("name")} )),
        ("let name", ConfKeywd::Var( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("name")} )),
        ("input1 const", ConfKeywd::Const( FnConfKeywdValue {input: String::from("input1"), type_: FnConfPointType::Unknown, data: String::new()} )),
        ("const name", ConfKeywd::Const( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("name")} )),
        ("input2 const name", ConfKeywd::Const( FnConfKeywdValue {input: String::from("input2"), type_: FnConfPointType::Unknown, data: String::from("name")} )),
        ("point /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("/path/Point.Name")} )),
        ("point '/path/Point.Name'", ConfKeywd::Point( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("/path/Point.Name")} )),
        ("point \"/path/Point.Name\"", ConfKeywd::Point( FnConfKeywdValue {input: String::new(), type_: FnConfPointType::Unknown, data: String::from("/path/Point.Name")} )),
        ("input1 point /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input1"), type_: FnConfPointType::Unknown, data: String::from("/path/Point.Name")} )),
        ("input2 point '/path/Point.Name'", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input2"), type_: FnConfPointType::Unknown, data: String::from("/path/Point.Name")} )),
        ("input3 point \"/path/Point.Name\"", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input3"), type_: FnConfPointType::Unknown, data: String::from("/path/Point.Name")} )),
        ("input4 point bool /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input4"), type_: FnConfPointType::Bool, data: String::from("/path/Point.Name")} )),
        ("input5 point int /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input5"), type_: FnConfPointType::Int, data: String::from("/path/Point.Name")} )),
        ("input6 point float /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input6"), type_: FnConfPointType::Float, data: String::from("/path/Point.Name")} )),
        ("input7 point string /path/Point.Name", ConfKeywd::Point( FnConfKeywdValue {input: String::from("input7"), type_: FnConfPointType::String, data: String::from("/path/Point.Name")} )),
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

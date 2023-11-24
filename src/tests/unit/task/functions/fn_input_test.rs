#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
    point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
    services::task::nested_function::{fn_::FnInOut, fn_input::FnInput},
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
fn initEach(initial: PointType) -> FnInOutRef {
    fn boxFnInput(input: FnInput) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    Rc::new(RefCell::new(
        boxFnInput(
            FnInput::new("test", initial)
        )
    ))
}


#[test]
fn test_int() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    info!("test_int");
    let input = initEach(0.toPoint("int"));
    let testData = vec![
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        5,
        4,
        3,
        2,
        1,
        0,
        0,
    ];
    for value in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asInt().value, value);
    }        
}

#[test]
fn test_bool() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    info!("test_bool");
    let input = initEach(false.toPoint("bool"));
    let testData = vec![
        false,
        false,
        false,
        true,
        false,
        true,
        true,
        false,
        false,
        false,
        true,
        true,
        false,
        false,
        false,
        false,
    ];
    for value in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asBool().value.0, value);
    }        
}

#[test]
fn test_float() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    info!("test_float");
    let input = initEach(0.0.toPoint("float"));
    let testData = vec![
        0.0,
        1.0,
        2.0,
        4.0,
        3.0,
        5.0,
        6.0,
        3.0,
        2.0,
        3.0,
        4.0,
        4.0,
        3.0,
        2.0,
        1.0,
        0.0,
    ];
    for value in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asFloat().value, value);
    }        
}


#[test]
fn test_string() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    info!("test_string");
    let input = initEach("0".toPoint("string"));
    let testData = vec![
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "5",
        "4",
        "3",
        "2",
        "1",
        "0",
        "0",
    ];
    for value in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asString().value, value);
    }        
}
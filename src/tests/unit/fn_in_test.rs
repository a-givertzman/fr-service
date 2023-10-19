#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{core_::debug::debug_session::{DebugSession, LogLevel}, services::task::nested_function::fn_::FnIn};

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
fn test_int() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_int");
    let mut input = FnIn::new(0);
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
        input.add(value);
        // debug!("input: {:?}", &input);
        let state = input.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, value);
    }        
}

#[test]
fn test_bool() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_bool");
    let input = Rc::new(RefCell::new(FnIn::new(false)));
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
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, value);
    }        
}

#[test]
fn test_float() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_float");
    let input = Rc::new(RefCell::new(FnIn::new(0.0)));
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
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, value);
    }        
}


#[test]
fn test_string() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_string");
    let mut input = FnIn::new("0");
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
        input.add(value);
        // debug!("input: {:?}", &input);
        let state = input.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, value);
    }        
}
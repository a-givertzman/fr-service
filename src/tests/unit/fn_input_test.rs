#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel}, 
    point::{point_type::PointType, point::Point}}, 
    services::task::nested_function::{fn_::{FnIn, FnOut, FnInOut}, fn_input::FnInput},
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
fn initEach(initial: PointType) -> Rc<RefCell<Box<dyn FnInOut>>> {
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
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_int");
    let mut input = initEach(PointType::Int(Point::newInt("int", 0)));
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
        let point = PointType::Int(Point::newInt("test", value));
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        let targetState = PointType::Int(Point::newInt("tesr", value));
        assert_eq!(state, targetState);
    }        
}

#[test]
fn test_bool() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_bool");
    let mut input = initEach(PointType::Bool(Point::newBool("bool", false)));
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
        let point = PointType::Bool(Point::newBool("test", value));
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        let targetState = PointType::Bool(Point::newBool("tesr", value));
        assert_eq!(state, targetState);
    }        
}

#[test]
fn test_float() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_float");
    let mut input = initEach(PointType::Float(Point::newFloat("float", 0.0)));
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
        let point = PointType::Float(Point::newFloat("test", value));
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        let targetState = PointType::Float(Point::newFloat("tesr", value));
        assert_eq!(state, targetState);
    }        
}


#[test]
fn test_string() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_string");
    let mut input = initEach(PointType::String(Point::newString("string", "0")));
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
        let point = PointType::String(Point::newString("test", value));
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        let targetState = PointType::String(Point::newString("tesr", value));
        assert_eq!(state, targetState);
    }        
}
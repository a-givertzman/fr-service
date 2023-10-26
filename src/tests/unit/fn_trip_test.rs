#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
    services::task::nested_function::{fn_::{FnInOut, FnOut}, fn_input::FnInput, fn_trip::FnTripGe}
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
fn test_single_int() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    info!("test_single");

    // let (initial, switches) = initEach();
    let setpoint = 4.0;
    let input = initEach(0.toPoint("int"));
    let mut fnTrip = FnTripGe::new(
        "test",
        false, 
        input.clone(),
        setpoint,
    );
    let testData = vec![
        (0, false),
        (1, false),
        (2, false),
        (3, false),
        (4, false),
        (5, true),
        (7, true),
        (6, true),
        (5, true),
        (3, false),
        (2, false),
        (1, false),
        (0, false),
        (0, false),
    ];
    for (value, targetState) in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asBool().value.0, targetState);
    }        
}

#[test]
fn test_multiple_int() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    info!("test_single");

    // let (initial, switches) = initEach();
    let setpoint = 4.0;
    let input = initEach(0.toPoint("int"));
    let mut fnTrip = FnTripGe::new(
        "test",
        false, 
        input.clone(),
        setpoint,
    );
    let testData = vec![
        (0, false),
        (1, false),
        (2, false),
        (5, true),
        (3, false),
        (5, true),
        (6, true),
        (3, false),
        (2, false),
        (3, false),
        (5, true),
        (5, true),
        (3, false),
        (2, false),
        (1, false),
        (0, false),
    ];
    for (value, targetState) in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asBool().value.0, targetState);
    }        
}

#[test]
fn test_multiple_float() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    info!("test_single");

    // let (initial, switches) = initEach();
    let setpoint = 4.0;
    let input = initEach(0.0.toPoint("float"));
    let mut fnTrip = FnTripGe::new(
        "test",
        false, 
        input.clone(),
        setpoint,
    );
    let testData = vec![
        (0.0, false),
        (1.0, false),
        (2.0, false),
        (5.0, true),
        (3.0, false),
        (5.0, true),
        (6.0, true),
        (3.0, false),
        (2.0, false),
        (3.0, false),
        (5.0, true),
        (5.0, true),
        (3.0, false),
        (2.0, false),
        (1.0, false),
        (0.0, false),
    ];
    for (value, targetState) in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asBool().value.0, targetState);
    }        
}
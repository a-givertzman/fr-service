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
    Rc::new(RefCell::new(Box::new(
        FnInput::new("test", initial)
    )))
}


#[test]
fn test_single_int() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    info!("test_single");

    // let (initial, switches) = initEach();
    let input1 = initEach(0.toPoint("point1"));
    let input2 = initEach(0.toPoint("point2"));
    let mut fnTrip = FnTripGe::new(
        "test",
        false, 
        input1.clone(),
        input2.clone(),
    );
    let testData = vec![
        (-1, 0, false),
        (0, 1, false),
        (-2, -1, false),
        (0, 1, false),
        (0, 0, true),
        (2, 1, true),
        (i64::MAX, 5, true),
        (3, 4, false),
        (2, 3, false),
        (1, 2, false),
        (0, 1, false),
        (-1, 0, false),
    ];
    for (value1, value2, targetState) in testData {
        let point1 = value1.toPoint("point1");
        let point2 = value2.toPoint("point2");
        input1.borrow_mut().add(point1);
        input2.borrow_mut().add(point2);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value1: {:?}  >=  value2: {:?}  |   state: {:?}", value1, value2, state);
        assert_eq!(state.asBool().value.0, targetState);
    }        
}

#[test]
fn test_multiple_int() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    info!("test_single");

    // let (initial, switches) = initEach();
    let input1 = initEach(0.toPoint("point1"));
    let input2 = initEach(0.toPoint("point2"));
    let mut fnTrip = FnTripGe::new(
        "test",
        false, 
        input1.clone(),
        input2.clone(),
    );
    let testData = vec![
        (-1, 0, false),
        (0, 1, false),
        (1, 2, false),
        (3, 3, true),
        (2, 3, false),
        (5, 3, true),
        (6, 3, true),
        (2, 3, false),
        (1, 2, false),
        (2, 3, false),
        (4, 4, true),
        (5, 4, true),
        (3, 4, false),
        (2, 3, false),
        (1, 2, false),
        (0, 1, false),
    ];
    for (value1, value2, targetState) in testData {
        let point1 = value1.toPoint("point1");
        let point2 = value2.toPoint("point2");
        input1.borrow_mut().add(point1);
        input2.borrow_mut().add(point2);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value1: {:?}  >=  value2: {:?}  |   state: {:?}", value1, value2, state);
        assert_eq!(state.asBool().value.0, targetState);
    }        
}

#[test]
fn test_multiple_float() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    info!("test_single");

    // let (initial, switches) = initEach();
    let input1 = initEach(0.0.toPoint("point1"));
    let input2 = initEach(0.0.toPoint("point2"));
    let mut fnTrip = FnTripGe::new(
        "test",
        false, 
        input1.clone(),
        input2.clone(),
    );
    let testData = vec![
        (-0.1, 0.0, false),
        (1.0, 1.1, false),
        (2.0, 2.2, false),
        (5.0, 5.0, true),
        (3.0, 3.1, false),
        (5.0, 5.0, true),
        (5.1, 5.0, true),
        (4.9, 5.0, false),
        (4.8, 5.0, false),
        (4.7, 5.0, false),
        (5.1, 5.0, true),
        (5.2, 5.0, true),
        (2.0, 3.0, false),
        (1.0, 2.0, false),
        (0.0, 1.0, false),
        (-0.1, 0.0, false),
    ];
    for (value1, value2, targetState) in testData {
        let point1 = value1.toPoint("point1");
        let point2 = value2.toPoint("point2");
        input1.borrow_mut().add(point1);
        input2.borrow_mut().add(point2);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value1: {:?}  >=  value2: {:?}  |   state: {:?}", value1, value2, state);

        assert_eq!(state.asBool().value.0, targetState);
    }        
}
#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::core::{nested_function::{fn_trip::FnTripGe, fn_in::FnIn, fn_::FnInput, fn_::FnOutput, }, debug::debug_session::{DebugSession, LogLevel}};

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
fn test_single_int() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_single");

    // let (initial, switches) = initEach();
    let setpoint = 4;
    let input = Rc::new(RefCell::new(FnIn::new(0)));
    let mut fnTrip = FnTripGe::new(
        false, 
        input.clone(),
        setpoint,
    );
    let testData = vec![
        (0, false),
        (1, false),
        (2, false),
        (3, false),
        (4, true),
        (5, true),
        (6, true),
        (5, true),
        (4, true),
        (3, false),
        (2, false),
        (1, false),
        (0, false),
        (0, false),
    ];
    for (value, targetState) in testData {
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, targetState);
    }        
}

#[test]
fn test_multiple_int() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_single");

    // let (initial, switches) = initEach();
    let setpoint = 4;
    let input = Rc::new(RefCell::new(FnIn::new(0)));
    let mut fnTrip = FnTripGe::new(
        false, 
        input.clone(),
        setpoint,
    );
    let testData = vec![
        (0, false),
        (1, false),
        (2, false),
        (4, true),
        (3, false),
        (5, true),
        (6, true),
        (3, false),
        (2, false),
        (3, false),
        (4, true),
        (4, true),
        (3, false),
        (2, false),
        (1, false),
        (0, false),
    ];
    for (value, targetState) in testData {
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, targetState);
    }        
}

#[test]
fn test_multiple_float() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_single");

    // let (initial, switches) = initEach();
    let setpoint = 4.0;
    let input = Rc::new(RefCell::new(FnIn::new(0.0)));
    let mut fnTrip = FnTripGe::new(
        false, 
        input.clone(),
        setpoint,
    );
    let testData = vec![
        (0.0, false),
        (1.0, false),
        (2.0, false),
        (4.0, true),
        (3.0, false),
        (5.0, true),
        (6.0, true),
        (3.0, false),
        (2.0, false),
        (3.0, false),
        (4.0, true),
        (4.0, true),
        (3.0, false),
        (2.0, false),
        (1.0, false),
        (0.0, false),
    ];
    for (value, targetState) in testData {
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let state = fnTrip.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, targetState);
    }        
}
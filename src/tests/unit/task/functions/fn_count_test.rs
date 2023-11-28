#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
    point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
    services::task::nested_function::{fn_::{FnInOut, FnOut}, 
    fn_count::{FnCount, self}, fn_input::FnInput},
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
    fn_count::resetCount();
    Rc::new(RefCell::new(Box::new(
        FnInput::new("test", initial)
    )))
}


#[test]
fn test_single() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    info!("test_single");
    let input = initEach(false.toPoint("bool"));
    let mut fnCount = FnCount::new(
        "test",
        0.0, 
        input.clone(),
    );
    let testData = vec![
        (false, 0),
        (false, 0),
        (true, 1),
        (false, 1),
        (false, 1),
        (true, 2),
        (false, 2),
        (true, 3),
        (false, 3),
        (false, 3),
        (true, 4),
        (true, 5),
        (false, 5),
        (false, 5),
    ];
    for (value, targetState) in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asFloat().value, targetState as f64);
    }        
}
// 

#[test]
fn test_multiple() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    info!("test_multiple");
    let input = initEach(false.toPoint("bool"));
    let mut fnCount = FnCount::new(
        "test",
        0.0, 
        input.clone(),
    );
    let testData = vec![
        (false, 0),
        (false, 0),
        (true, 1),
        (false, 1),
        (false, 1),
        (true, 2),
        (false, 2),
        (true, 3),
        (false, 3),
        (false, 3),
        (true, 4),
        (true, 5),
        (false, 5),
        (false, 5),
    ];
    for (value, targetState) in testData {
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asFloat().value, targetState as f64);
    }        
}

#[test]
fn test_multiple_reset() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    info!("test_multiple_reset");
    let input = initEach(false.toPoint("bool"));
    let mut fnCount = FnCount::new(
        "test",
        0.0, 
        input.clone(),
    );
    let testData = vec![
        (false, 0, false),
        (false, 0, false),
        (true, 1, false),
        (false, 1, false),
        (false, 1, false),
        (true, 2, false),
        (false, 0, true),
        (true, 1, false),
        (false, 1, false),
        (false, 1, false),
        (true, 2, false),
        (true, 3, false),
        (false, 0, true),
        (false, 0, false),
    ];
    for (value, targetState, reset) in testData {
        if reset {
            fnCount.reset();
        }
        let point = value.toPoint("test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.asFloat().value, targetState as f64);
    }        
}

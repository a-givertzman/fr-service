#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel}, 
    point::{point_type::PointType, point::Point}}, 
    services::task::nested_function::{fn_::{FnInOut, FnOut}, 
    fn_count::FnCount, fn_input::FnInput},
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
fn test_single() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_single");
    let mut input = initEach(PointType::Bool(Point::newBool("bool", false)));
    let mut fnCount = FnCount::new(
        0, 
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
        (true, 4),
        (false, 4),
        (false, 4),
    ];
    for (value, targetState) in testData {
        let point = PointType::Bool(Point::newBool("test", value));
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        let targetState = PointType::Int(Point::newInt("tesr", targetState));
        assert_eq!(state, targetState);
    }        
}
// 

#[test]
fn test_multiple() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_multiple");
    // let (initial, switches) = initEach();
    let mut input = initEach(PointType::Bool(Point::newBool("bool", false)));
    let mut fnCount = FnCount::new(
        0, 
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
        (true, 4),
        (false, 4),
        (false, 4),
    ];
    for (value, targetState) in testData {
        let point = PointType::Bool(Point::newBool("test", value));
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        let targetState = PointType::Int(Point::newInt("tesr", targetState));
        assert_eq!(state, targetState);
    }        
}

#[test]
fn test_multiple_reset() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_multiple_reset");
    let mut input = initEach(PointType::Bool(Point::newBool("bool", false)));
    let mut fnCount = FnCount::new(
        0, 
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
        (true, 2, false),
        (false, 0, true),
        (false, 0, false),
    ];
    for (value, targetState, reset) in testData {
        if reset {
            fnCount.reset();
        }
        let point = PointType::Bool(Point::newBool("test", value));
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        let targetState = PointType::Int(Point::newInt("tesr", targetState));
        assert_eq!(state, targetState);
    }        
}

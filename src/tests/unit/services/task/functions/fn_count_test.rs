#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::{
    core_::{point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
    services::task::nested_function::{fn_::FnOut, 
    fn_count::{FnCount, self}, fn_input::FnInput, reset_counter::AtomicReset},
};

// Note this useful idiom: importing names from outer (for mod tests) scope.
// use super::*;

static INIT: Once = Once::new();

///
/// once called initialisation
fn init_once() {
    INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        }
    )
}


///
/// returns:
///  - ...
fn init_each(initial: PointType) -> FnInOutRef {
    fn_count::COUNT.reset();
    Rc::new(RefCell::new(Box::new(
        FnInput::new("test", initial)
    )))
}


#[test]
fn test_single() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_single");
    let input = init_each(false.to_point(0, "bool"));
    let mut fnCount = FnCount::new(
        "test",
        0.0, 
        input.clone(),
    );
    let test_data = vec![
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
    for (value, targetState) in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_double().value, targetState as f64);
    }        
}
// 

#[test]
fn test_multiple() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_multiple");
    let input = init_each(false.to_point(0, "bool"));
    let mut fnCount = FnCount::new(
        "test",
        0.0, 
        input.clone(),
    );
    let test_data = vec![
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
    for (value, targetState) in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_double().value, targetState as f64);
    }        
}

#[test]
fn test_multiple_reset() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_multiple_reset");
    let input = init_each(false.to_point(0, "bool"));
    let mut fnCount = FnCount::new(
        "test",
        0.0, 
        input.clone(),
    );
    let test_data = vec![
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
    for (value, targetState, reset) in test_data {
        if reset {
            fnCount.reset();
        }
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_double().value, targetState as f64);
    }        
}

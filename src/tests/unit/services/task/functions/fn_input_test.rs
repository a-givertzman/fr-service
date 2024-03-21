#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::{
    core_::{point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
    services::task::nested_function::fn_input::FnInput,
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
    Rc::new(RefCell::new(
        Box::new(
            FnInput::new("test", initial)
        )
    ))
}


#[test]
fn test_int() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_int");
    let input = init_each(0.to_point(0, "int"));
    let test_data = vec![
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
    for value in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_int().value, value);
    }        
}

#[test]
fn test_bool() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_bool");
    let input = init_each(false.to_point(0, "bool"));
    let test_data = vec![
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
    for value in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_bool().value.0, value);
    }        
}

#[test]
fn test_float() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_float");
    let input = init_each(0.0.to_point(0, "float"));
    let test_data = vec![
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
    for value in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_double().value, value);
    }        
}


#[test]
fn test_string() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_string");
    let input = init_each("0".to_point(0, "string"));
    let test_data = vec![
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
    for value in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = input.borrow_mut().out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_string().value, value);
    }        
}
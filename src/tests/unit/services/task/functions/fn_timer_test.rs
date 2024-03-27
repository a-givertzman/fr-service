#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, time::{Instant, Duration}, thread,rc::Rc, cell::RefCell};
use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use crate::{
    conf::fn_::fn_conf_keywd::FnConfPointType, core_::{aprox_eq::aprox_eq::AproxEq, point::point_type::{PointType, ToPoint}, 
    types::fn_in_out_ref::FnInOutRef}, services::task::nested_function::{fn_::FnOut, fn_input::FnInput, fn_timer::FnTimer}
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
fn init_each(initial: PointType, type_: FnConfPointType) -> FnInOutRef {
    Rc::new(RefCell::new(
        Box::new(
            FnInput::new("test", initial, type_)
        )
    ))
}


#[test]
fn test_elapsed_repeat_false() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_elapsed_repeat_false");
    let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
    let mut fnTimer = FnTimer::new(
        "id", 
        0,
        input.clone(),
        false,
    );
    let test_data = vec![
        (false, 0),
        (false, 0),
        (true, 1),
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
    let mut start: Option<Instant> = None;
    let mut target: f64;
    let mut elapsed: f64 = 0.0;
    let mut elapsedTotal: f64 = 0.0;
    let mut done = false;
    for (value, _) in test_data {
        if !done {
            if value {
                if start.is_none() {
                    start = Some(Instant::now());
                } else {
                    elapsed = start.unwrap().elapsed().as_secs_f64();
                }
            } else {
                if start.is_some() {
                    elapsed = 0.0;
                    elapsedTotal += start.unwrap().elapsed().as_secs_f64();
                    // start = None
                    done = true;
                }
            }
        }
        target = elapsedTotal + elapsed;
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let fnTimerElapsed = fnTimer.out().as_double().value;
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, fnTimerElapsed);
        assert!(fnTimerElapsed.aprox_eq(target, 2), "current '{}' != target '{}'", fnTimerElapsed, target);
        thread::sleep(Duration::from_secs_f64(0.1));
    }        
}

#[test]
fn test_total_elapsed_repeat() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_total_elapsed_repeat");
    let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
    let mut fnTimer = FnTimer::new(
        "id", 
        0,
        input.clone(),
        true,
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
        (true, 4),
        (false, 4),
        (false, 4),
    ];
    let mut start: Option<Instant> = None;
    let mut target: f64;
    let mut elapsed: f64 = 0.0;
    let mut elapsedTotal: f64 = 0.0;
    for (value, _) in test_data {
        if value {
            if start.is_none() {
                start = Some(Instant::now());
            } else {
                elapsed = start.unwrap().elapsed().as_secs_f64();
            }
        } else {
            if start.is_some() {
                elapsed = 0.0;
                elapsedTotal += start.unwrap().elapsed().as_secs_f64();
                start = None
            }
        }
        target = elapsedTotal + elapsed;
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let fnTimerElapsed = fnTimer.out().as_double().value;
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, fnTimerElapsed);
        assert!(fnTimerElapsed.aprox_eq(target, 2), "current '{}' != target '{}'", fnTimerElapsed, target);
        thread::sleep(Duration::from_secs_f64(0.1));
    }        
}

#[test]
fn test_total_elapsed_repeat_reset() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_total_elapsed_repeat_reset");
    let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
    let mut fnTimer = FnTimer::new(
        "id",
        0, 
        input.clone(),
        true,
    );
    let test_data = vec![
        (false, 0, false),
        (false, 0, false),
        (true, 1, false),
        (false, 1, false),
        (false, 1, false),
        (true, 2, false),
        (false, 2, false),
        (true, 3, false),
        (false, 3, false),
        (false, 3, false),
        (true, 4, false),
        (true, 4, true),
        (true, 4, false),
        (false, 4, false),
        (false, 4, false),
    ];
    let mut start: Option<Instant> = None;
    let mut elapsedTotal: f64 = 0.0;
    let mut elapsedSession: f64 = 0.0;
    let mut target;
    for (value, _, reset) in test_data {
        if reset {
            start = None;
            elapsedSession = 0.0;
            elapsedTotal = 0.0;
            fnTimer.reset();
        }
        if value {
            if start.is_none() {
                start = Some(Instant::now());
            } else {
                elapsedSession = start.unwrap().elapsed().as_secs_f64();
            }
        } else {
            if start.is_some() {
                elapsedSession = 0.0;
                elapsedTotal += start.unwrap().elapsed().as_secs_f64();
                start = None;
            }
        }
        target = elapsedTotal + elapsedSession;
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let fnTimerElapsed = fnTimer.out().as_double().value;
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}   |   target {}{}", value, fnTimerElapsed, target, if reset {"\t<-- reset"} else {""});
        assert!(fnTimerElapsed.aprox_eq(target, 2), "current '{}' != target '{}'", fnTimerElapsed, target);
        thread::sleep(Duration::from_secs_f64(0.1));
    }        
}


#[test]
fn test_initial_repeat() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_initial_repeat");
    let initial = 123.1234;
    let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
    let mut fnTimer = FnTimer::new(
        "id",
        initial, 
        input.clone(),
        true,
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
        (true, 4),
        (false, 4),
        (false, 4),
    ];
    let mut start: Option<Instant> = None;
    let mut target: f64;
    let mut elapsed: f64 = 0.0;
    let mut elapsedTotal: f64 = initial;
    for (value, _) in test_data {
        if value {
            if start.is_none() {
                start = Some(Instant::now());
            } else {
                elapsed = start.unwrap().elapsed().as_secs_f64();
            }
        } else {
            if start.is_some() {
                elapsed = 0.0;
                elapsedTotal += start.unwrap().elapsed().as_secs_f64();
                start = None
            }
        }
        target = elapsedTotal + elapsed;
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let fnTimerElapsed = fnTimer.out().as_double().value;
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, fnTimerElapsed);
        assert!(fnTimerElapsed.aprox_eq(target, 2), "current '{}' != target '{}'", fnTimerElapsed, target);
        thread::sleep(Duration::from_secs_f64(0.1));
    }        
}

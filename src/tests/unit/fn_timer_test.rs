#![allow(non_snake_case)]
use std::{sync::Once, time::{Instant, Duration}, thread};
#[cfg(test)]
use std::{rc::Rc, cell::RefCell};
use log::{debug, info};
use crate::{
    tests::unit::init::{TestSession, LogLevel},
    core::{nested_function::{fn_timer::FnTimer, fn_in::FnIn, fn_::FnInput, fn_::FnOutput}, aprox_eq::aprox_eq::AproxEq}, 
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
fn initEach() -> () {

}


#[test]
fn test_elapsed_repeat_false() {
    TestSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_elapsed_repeat_none");
    let input = Rc::new(RefCell::new(FnIn::new(false)));
    let mut fnTimer = FnTimer::new(
        0, 
        input.clone(),
        false,
    );
    let testData = vec![
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
    let mut elapsed: f64 = 0.0;
    for (value, _) in testData {
        if value {
            start = Some(Instant::now());
        } else {
            if start.is_some() && elapsed == 0.0 {
                elapsed = start.unwrap().elapsed().as_secs_f64();
                // start = None
            }
        }
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let fnTimerElapsed = fnTimer.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, fnTimerElapsed);
        assert!(fnTimerElapsed.aproxEq(elapsed, 2), "current '{}' != target '{}'", fnTimerElapsed, elapsed);
        thread::sleep(Duration::from_secs_f64(0.1));
    }        
}

#[test]
fn test_total_elapsed_repeat() {
    TestSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_total_elapsed");
    let input = Rc::new(RefCell::new(FnIn::new(false)));
    let mut fnTimer = FnTimer::new(
        0, 
        input.clone(),
        true,
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
    let mut start: Option<Instant> = None;
    let mut elapsed: f64 = 0.0;
    for (value, _) in testData {
        if value {
            start = Some(Instant::now());
        } else {
            if start.is_some() {
                elapsed += start.unwrap().elapsed().as_secs_f64();
                start = None
            }
        }
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let fnTimerElapsed = fnTimer.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, fnTimerElapsed);
        assert!(fnTimerElapsed.aproxEq(elapsed, 2), "current '{}' != target '{}'", fnTimerElapsed, elapsed);
        thread::sleep(Duration::from_secs_f64(0.1));
    }        
}

#[test]
fn test_initial_repeat() {
    TestSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_initial_repeat");
    let initial = 123.1234;
    let input = Rc::new(RefCell::new(FnIn::new(false)));
    let mut fnTimer = FnTimer::new(
        initial, 
        input.clone(),
        true,
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
    let mut start: Option<Instant> = None;
    let mut elapsed: f64 = initial;
    for (value, _) in testData {
        if value {
            start = Some(Instant::now());
        } else {
            if start.is_some() {
                elapsed += start.unwrap().elapsed().as_secs_f64();
                start = None
            }
        }
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let fnTimerElapsed = fnTimer.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, fnTimerElapsed);
        assert!(fnTimerElapsed.aproxEq(elapsed, 2), "current '{}' != target '{}'", fnTimerElapsed, elapsed);
        thread::sleep(Duration::from_secs_f64(0.1));
    }        
}

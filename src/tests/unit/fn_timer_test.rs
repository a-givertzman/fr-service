#![allow(non_snake_case)]
use std::sync::Once;
#[cfg(test)]
use std::{rc::Rc, cell::RefCell};
use log::{debug, info};
use crate::{
    tests::unit::init::TestSession,
    core::nested_function::{fn_timer::FnTimer, fn_in::FnIn, fn_::FnInput, fn_::FnOutput}, 
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
fn test_single() {
    TestSession::init();
    initOnce();
    initEach();
    info!("test_single");

    // let (initial, switches) = initEach();
    let input = Rc::new(RefCell::new(FnIn::new(false)));
    let mut fnTimer = FnTimer::new(
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
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let state = fnTimer.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, targetState);
    }        
}


#[test]
fn test_multy() {
    TestSession::init();
    initOnce();
    initEach();
    info!("test_single");

    // let (initial, switches) = initEach();
    let input = Rc::new(RefCell::new(FnIn::new(false)));
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
        input.borrow_mut().add(value);
        // debug!("input: {:?}", &input);
        let state = fnCount.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, targetState);
    }        
}

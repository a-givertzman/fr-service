#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    tests::unit::init::TestSession,
    core::nested_function::{fn_trip::FnTripGe, fn_in::FnIn, fn_::FnInput, fn_::FnOutput, fn_count::FnCount}, 
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
fn test_int() {
    TestSession::init();
    initOnce();
    initEach();
    info!("test_single");

    // let (initial, switches) = initEach();
    let setpoint = 4;
    let input = Rc::new(RefCell::new(FnIn::new(false)));
    let mut fnTrip = FnTripGe::new(
        false, 
        Rc::new(RefCell::new(FnCount::new(
            0, 
            input.clone(),
        ))),
        setpoint,
    );
    let testData = vec![
        (false, false),
        (false, false),
        (true, false),
        (false, false),
        (false, false),
        (true, false),
        (false, false),
        (true, false),
        (false, false),
        (false, false),
        (true, true),
        (true, true),
        (false, true),
        (false, true),
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

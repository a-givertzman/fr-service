#![allow(non_snake_case)]
#[cfg(test)]
use std::{rc::Rc, cell::RefCell};
use log::{debug, info};
use crate::{
    tests::unit::init::TestSession,
    core::nested_function::{fn_trip::FnTripGe, fn_in::FnIn, fn_::FnInput, fn_::FnOutput, fn_count::FnCount}, 
};

// Note this useful idiom: importing names from outer (for mod tests) scope.
// use super::*;

// static INIT: Once = Once::new();

// fn initOnce() {
//     INIT.call_once(|| {
//             env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
//             // env::set_var("RUST_BACKTRACE", "1");
//             env::set_var("RUST_BACKTRACE", "full");
//             env_logger::init();
//         }
//     )
// }


///
/// returns tuple(
///     - initialState: ProcessState
///     - switches: Vec<Switch<ProcessState, u8>>
/// )
// fn initEach() -> () {

// }

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

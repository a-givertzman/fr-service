#![allow(non_snake_case)]
#[cfg(test)]
use std::{rc::Rc, cell::RefCell};
use log::{debug, info};
use crate::{
    tests::unit::init::tryInit,
    core::nested_function::{fn_trip::FnTripGe, fn_in::FnIn, fn_::FnInput, fn_::FnOutput}, 
};

// Note this useful idiom: importing names from outer (for mod tests) scope.
// use super::*;

// static INIT: Once = Once::new();

// fn init() {
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
fn test_single_int() {
    tryInit();
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
    tryInit();
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

// #[test]
// fn test_multy() {
//     tryInit();
//     info!("test_single");

//     // let (initial, switches) = initEach();
//     let setpoint = 4.0;
//     let input = Rc::new(RefCell::new(FnIn::new(0.0)));
//     let mut fnTrip = FnTripGe::new(
//         false, 
//         input.clone(),
//         setpoint,
//     );
//     let testData = vec![
//         (0.0, false),
//         (1.0, false),
//         (2.0, false),
//         (3.0, false),
//         (4.0, false),
//         (5.0, false),
//         (6.0, false),
//         (5.0, false),
//         (4.0, false),
//         (3.0, false),
//         (2.0, false),
//         (1.0, false),
//         (0.0, false),
//         (0.0, false),
//     ];
//     for (value, targetState) in testData {
//         input.borrow_mut().add(value);
//         // debug!("input: {:?}", &input);
//         let state = fnTrip.out();
//         // debug!("input: {:?}", &mut input);
//         debug!("value: {:?}   |   state: {:?}", value, state);
//         assert_eq!(state, targetState);
//     }        
// }

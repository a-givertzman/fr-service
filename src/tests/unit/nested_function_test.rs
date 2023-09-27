#![allow(non_snake_case)]
#[cfg(test)]
use std::{rc::Rc, cell::RefCell};
use log::{debug, info};
use crate::{
    tests::unit::init::tryInit,
    core::nested_function::{nested_function::FnCount, fn_in::FnIn, fn_::FnInput, fn_::FnOutput}, 
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
fn test_single() {
    tryInit();
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


#[test]
fn test_multy() {
    tryInit();
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

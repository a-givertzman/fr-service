#![allow(non_snake_case)]
use std::{sync::Once, env};
use log::trace;
#[cfg(test)]
use log::{debug, info};
use std::{rc::Rc, cell::RefCell};
use crate::{
    tests::unit::init::{TestSession, LogLevel},
    core::nested_function::{fn_count::FnCount, fn_in::FnIn, fn_::FnInput, fn_::FnOutput, fn_reset::FnReset, fn_config::FnConfig}, 
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
fn test_fn_config() {
    TestSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_single");
    // let (initial, switches) = initEach();
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/tests/unit/fn_config_test.yaml";
    let fnConfig = FnConfig::read(path);
    trace!("fnConfig: {:?}", fnConfig);
    // for (value, targetState) in testData {
    //     input.borrow_mut().add(value);
    //     // debug!("input: {:?}", &input);
    //     let state = fnCount.out();
    //     // debug!("input: {:?}", &mut input);
    //     debug!("value: {:?}   |   state: {:?}", value, state);
    //     assert_eq!(state, targetState);
    // }        
}


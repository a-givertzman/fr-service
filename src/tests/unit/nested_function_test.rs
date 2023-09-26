#![allow(non_snake_case)]
#[cfg(test)]
use std::env;
use std::sync::Once;
use log::{debug, info};
use crate::core::nested_function::nested_function::NestedFunction;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ProcessState {
    Off,
    Start,
    Progress,
    Stop,
}

// Note this useful idiom: importing names from outer (for mod tests) scope.
// use super::*;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
            env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
            // env::set_var("RUST_BACKTRACE", "1");
            env::set_var("RUST_BACKTRACE", "full");
            env_logger::init();
        }
    )
}


///
/// returns tuple(
///     - initialState: ProcessState
///     - switches: Vec<Switch<ProcessState, u8>>
/// )
fn initEach() -> () {

}

#[test]
fn test_single() {
    init();
    info!("test_single");

    // let (initial, switches) = initEach();

    let mut function: NestedFunction = NestedFunction{};
    let testData = vec![
        (0, ProcessState::Off),
        (0, ProcessState::Off),
        (1, ProcessState::Off),
        (1, ProcessState::Off),
        (2, ProcessState::Off),
        (2, ProcessState::Off),
        (5, ProcessState::Start),
        (5, ProcessState::Progress),
        (6, ProcessState::Progress),
        (6, ProcessState::Progress),
        (6, ProcessState::Progress),
        (7, ProcessState::Progress),
        (7, ProcessState::Progress),
        (7, ProcessState::Progress),
        (6, ProcessState::Progress),
        (6, ProcessState::Progress),
        (6, ProcessState::Progress),
        (5, ProcessState::Progress),
        (5, ProcessState::Progress),
        (2, ProcessState::Stop),
        (2, ProcessState::Off),
        (1, ProcessState::Off),
        (1, ProcessState::Off),
    ];
    for (value, targetState) in testData {
        function.add(value);
        let state = function.state();
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state, targetState);
    }        
}

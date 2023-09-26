#![allow(non_snake_case)]

mod core;

use std::env;

use crate::core::state::switch_state::{
    SwitchState,
    Switch, SwitchCondition,
};



#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ProcessState {
    off,
    start,
    progress,
    stop,
}

#[cfg(test)]
// #[path = "./tests"]
mod tests;
// impl Eq for State


fn main() {
    env::set_var("RUST_LOG", "debug");  // off / error / warn / info / debug / trace
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    println!("main");
    // let state = State::off;
    // match state {
    //     State::off => todo!(),
    //     State::start => todo!(),
    //     State::progress => todo!(),
    //     State::stop => todo!(),
    // }
    let initial = ProcessState::off;
    let switches = vec![
        Switch{
            state: ProcessState::off,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value >= 5}),
                    target: ProcessState::start,        
                },
            ],
        },
        Switch{
            state: ProcessState::stop,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value >= 5}),
                    target: ProcessState::start,
                },
                SwitchCondition {
                    condition: Box::new(|value| {value < 5}),
                    target: ProcessState::off,
                },
            ],
        },
        Switch{
            state: ProcessState::start,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value >= 5}),
                    target: ProcessState::progress,        
                },
                SwitchCondition {
                    condition: Box::new(|value| {value < 5}),
                    target: ProcessState::stop,
                },
            ],

        },
        Switch{
            state: ProcessState::progress,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value < 5}),
                    target: ProcessState::stop,
                },
            ],

        },
    ];
    let mut switchState: SwitchState<ProcessState, i8> = SwitchState::new(
        initial,
        switches,
    );
    let sequence = vec![0,0,1,1,2,2,5,5,6,6,6,7,7,7,6,6,6,5,5,2,2,1,1];
    // let sequence = vec![0,0,1,1,2,2,5,0,6,0,6,7,7,7,6,6,6,5,5,2,2,1,1];
    // let sequence = vec![0,0,1,1,2,2,5,0,6,0,6,7,7,7,6,6,6,5,2,7,2,1,1];
    for value in sequence {
        switchState.add(value);
        let state = switchState.state();
        println!("value: {:?}   |   state: {:?}", value, state);
    }
}

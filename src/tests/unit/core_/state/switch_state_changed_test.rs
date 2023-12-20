#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::info;
    use std::sync::Once;
    use crate::core_::{
        debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        state::{switch_state::{SwitchState, Switch, SwitchCondition}, switch_state_changed::SwitchStateChanged},
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
    fn initEach<T: std::cmp::PartialOrd + Clone + 'static>(initial: u8, steps: Vec<T>) -> SwitchState<u8, T> {
        fn switch<T: std::cmp::PartialOrd + Clone + 'static>(state: &mut u8, input: Option<T>) -> Switch<u8, T> {
            let state_ = *state;
            *state = *state + 1;
            let target = state;
            Switch{
                state: state_,
                conditions: vec![
                    SwitchCondition {
                        condition: Box::new(move |value| {
                            match input.clone() {
                                Some(input) => value >= input,
                                None => false,
                            }
                        }),
                        target: *target,        
                    },
                ],
            }
        }
        let mut state: u8 = initial;
        let mut switches: Vec<Switch<u8, T>> = steps.into_iter().map(|input| {switch(&mut state, Some(input))}).collect();
        switches.push(switch(&mut state, None));
        let switchState: SwitchState<u8, T> = SwitchState::new(
            initial,
            switches,
        );
        switchState
    }
    
    #[test]
    fn test_state() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        println!("");
        println!("test SwitchState");

        let steps: Vec<f64> = vec![0.25, 0.50, 0.75];
        let initial = 1;
        let mut switchState = SwitchStateChanged::new(
            initEach(initial, steps),
        );

        let mut prevState = initial;
        for value in 0..=100 {
            let value = 0.01 * (value as f64);
            switchState.add(value);
            let state = switchState.state();
            let changed = switchState.changed();
            info!("state: {},\t changed: {},\t value: {}", state, changed, value);
            if state != prevState {
                assert!(changed == true, "\nresult: {:?}\ntarget: {:?}", changed, true);
                prevState = state;
            } else {
                assert!(changed == false, "\nresult: {:?}\ntarget: {:?}", changed, false);
            }
        }
    }
}


#![allow(non_snake_case)]

mod core;

use std::{env, collections::HashMap};

use log::{info, debug};

use crate::core::{state::switch_state::{
    SwitchState,
    Switch, SwitchCondition,
}, nested_function::fn_config::FnConfig};



#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum ProcessState {
    Off,
    Start,
    Progress,
    Stop,
}

#[cfg(test)]
// #[path = "./tests"]
mod tests;


fn main() {
    env::set_var("RUST_LOG", "trace");  // off / error / warn / info / debug / trace
    // env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();

    info!("test_create_valid_fn");
    // let (initial, switches) = initEach();
    let testData = [
        (serde_yaml::from_str(r#"let newVar:
            input:
                fn count:
                    inputConst1: const '13.5'
                    inputConst2: const '13.5'"#), FnConfig{ fnType: None, name: "".to_string(), inputs: HashMap::new() }),
        // (serde_yaml::from_str("fn count:\
        //     inputTrip:\
        //         fn trip:\
        //             input: point '/path/Point.Name'"), FnConfigType::Fn(FnConfig{ inputs: HashMap::new() })),
    ];
    for (value, target) in testData {
        debug!("test value: {:?}", value);
        let conf: serde_yaml::Value = value.unwrap();
        // let conf = testData.get("/").unwrap();

        debug!("value: {:?}   |   conf: {:?}   |   target: {:?}", "_", conf, target);
        // let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
        // debug!("\tfnKeyword: {:?}", fnKeyword);
        let fnConfig = FnConfig::new(&conf);
        debug!("\tfnConfig: {:?}", fnConfig);
        // assert_eq!(fnConfigType, target);
    }

}


fn main1() {
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
    let initial = ProcessState::Off;
    let switches = vec![
        Switch{
            state: ProcessState::Off,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value >= 5}),
                    target: ProcessState::Start,        
                },
            ],
        },
        Switch{
            state: ProcessState::Stop,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value >= 5}),
                    target: ProcessState::Start,
                },
                SwitchCondition {
                    condition: Box::new(|value| {value < 5}),
                    target: ProcessState::Off,
                },
            ],
        },
        Switch{
            state: ProcessState::Start,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value >= 5}),
                    target: ProcessState::Progress,        
                },
                SwitchCondition {
                    condition: Box::new(|value| {value < 5}),
                    target: ProcessState::Stop,
                },
            ],

        },
        Switch{
            state: ProcessState::Progress,
            conditions: vec![
                SwitchCondition {
                    condition: Box::new(|value| {value < 5}),
                    target: ProcessState::Stop,
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

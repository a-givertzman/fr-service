#![allow(non_snake_case)]

use std::{collections::HashMap, fmt::Debug, hash::Hash};

// #[derive(Sync)]
pub struct Switch<TState, TInput> {
    pub state: TState,
    pub conditions: Vec<SwitchCondition<TState, TInput>>,
}

impl<TState: Debug, TInput> Debug for Switch<TState, TInput> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Switch")
        .field("state", &self.state)
        .field("conditions", &self.conditions)
        .finish()
    }
}

pub struct SwitchCondition<TState, TInput> {
    pub condition: Box<dyn Fn(TInput) -> bool>,
    pub target: TState,
}

impl<TState: Debug, TInput> Debug for SwitchCondition<TState, TInput> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchCondition")
        .field("target", &self.target)
        .finish()
    }
}
pub struct SwitchState<TState, TInput> {
    switches: HashMap<TState, Switch<TState, TInput>>,
    state: TState,
}

impl<TState: Debug + Eq + Hash + Clone, TInput: Clone> SwitchState<TState, TInput> {
    pub fn new(initial: TState, switches: Vec<Switch<TState, TInput>>) -> Self {
        let mut switchesSet = HashMap::new();
        for switch in switches {
            // let key = format!("{:?}", switch.state);
            switchesSet.insert(switch.state.clone(), switch);
        }
        println!("SwitchState{{switches: {:?}}}", &switchesSet);
        Self { 
            switches: switchesSet,
            state: initial,
        }
    }
    ///
    pub fn add(& mut self, value: TInput) {
        let key = self.state.clone(); 
        let switchRef = &self.switches[&key];
        // let switch: Switch<T, U> = switchRef.clone().to_owned();
        for switchCondition in &switchRef.conditions {            
            let cond = (switchCondition.condition)(value.clone());
            if cond {
                self.state = switchCondition.target.clone();
            }
        };
    }
    ///
    pub fn state(&self) -> TState {
        self.state.clone()
    }
}
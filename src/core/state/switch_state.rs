use std::{collections::HashMap, fmt::Debug, hash::Hash};

// #[derive(Sync)]
pub struct Switch<TState, TInput> {
    pub state: TState,
    pub conditions: Vec<SwitchCondition<TState, TInput>>,
}

impl<T: Debug, U> Debug for Switch<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Switch")
        .field("state", &self.state)
        .field("conditions", &self.conditions)
        .finish()
    }
}

pub struct SwitchCondition<T, U> {
    pub condition: Box<dyn Fn(U) -> bool>,
    pub target: T,
}

impl<T: Debug, U> Debug for SwitchCondition<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwitchCondition")
        .field("target", &self.target)
        .finish()
    }
}
pub struct SwitchState<T, U> {
    switches: HashMap<T, Switch<T, U>>,
    state: T,
}

impl<T: Debug + Eq + Hash + Clone, U: Clone> SwitchState<T, U> {
    pub fn new(initial: T, switches: Vec<Switch<T, U>>) -> Self {
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
    pub fn add(& mut self, value: U) {
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
    pub fn state(&self) -> T {
        self.state.clone()
    }
}
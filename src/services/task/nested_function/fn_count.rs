#![allow(non_snake_case)]

use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::core_::{state::switch_state::{SwitchState, Switch, SwitchCondition}, point::point::PointType};

use super::fn_::{FnInOut, FnOut, FnIn};


///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnCount {
    input: Rc<RefCell<Box<dyn FnInOut>>>,
    state: SwitchState<bool, bool>,
    count: u128,
    initial: u128,
}
///
/// 
impl FnCount {
    ///
    /// Creates new instance of the FnCount
    #[allow(dead_code)]
    pub fn new(initial: u128, input: Rc<RefCell<Box<dyn FnInOut>>>) -> Self {
        Self { 
            input,
            state: SwitchState::new(
                false, 
                vec![
                    Switch {
                        state: false,
                        conditions: vec![SwitchCondition {
                            condition: Box::new(|value| {value}),
                            target: true,
                        }],
                    },
                    Switch {
                        state: true,
                        conditions: vec![SwitchCondition {
                            condition: Box::new(|_| {true}),
                            target: false,
                        }],
                    },
                ]
            ),
            count: initial.clone(),
            initial: initial,
        }
    }
}
///
/// 
impl FnIn for FnCount {
    fn add(&mut self, _: PointType) {
        panic!("FnCount.add | method is not used")
    }
}
///
/// 
impl FnOut for FnCount {
    ///
    fn out(&mut self) -> u128 {
        // trace!("FnCount.out | input: {:?}", self.input.print());
        let point = self.input.borrow_mut().out().asBool();
        let value = point.value.0;
        self.state.add(value);
        let state = self.state.state();
        trace!("FnCount.out | input.out: {:?}   | state: {:?}", &value, state);
        if state {
            self.count += 1;
        }
        self.count
    }
    fn reset(&mut self) {
        self.count = self.initial;
        self.state.reset();
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnCount {}

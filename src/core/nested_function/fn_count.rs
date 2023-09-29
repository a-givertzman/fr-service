#![allow(non_snake_case)]

use log::debug;
use std::{cell::RefCell, rc::Rc};

use crate::core::state::switch_state::{SwitchState, Switch, SwitchCondition};

use super::{fn_::FnOutput, fn_reset::FnReset};

///
/// Counts number of raised fronts of boolean input
pub struct FnCount<TIn> {
    input: Rc<RefCell<dyn FnOutput<bool>>>,
    state: SwitchState<bool, bool>,
    count: TIn,
    initial: TIn,
}

impl<TIn: Clone> FnCount<TIn> {
    #[allow(dead_code)]
    pub fn new(initial: TIn, input: Rc<RefCell<dyn FnOutput<bool>>>) -> Self {
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

impl FnOutput<u128> for FnCount<u128> {
    ///
    fn out(&mut self) -> u128 {
        // debug!("FnCount.out | input: {:?}", self.input.print());
        let value = self.input.borrow_mut().out();
        self.state.add(value);
        let state = self.state.state();
        debug!("FnCount.out | input.out: {:?}   | state: {:?}", &value, state);
        if state {
            self.count += 1;
        }
        self.count
    }
}

impl FnReset for FnCount<u128> {
    fn reset(&mut self) {
        self.count = self.initial;
        // self.input.re
    }
}
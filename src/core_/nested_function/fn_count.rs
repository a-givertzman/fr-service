#![allow(non_snake_case)]

use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::core_::state::switch_state::{SwitchState, Switch, SwitchCondition};

use super::{fn_::FnOutput, fn_reset::FnReset};


///
/// Counts number of raised fronts of boolean input
pub struct FnCount<TInput> where TInput: FnOutput<bool> {
    // input: Rc<RefCell<(dyn FnCountInput)>>,
    input: Rc<RefCell<TInput>>,
    state: SwitchState<bool, bool>,
    count: u128,
    initial: u128,
}

impl<TInput: FnOutput<bool>> FnCount<TInput> {
    #[allow(dead_code)]
    pub fn new(initial: u128, input: Rc<RefCell<TInput>>) -> Self {
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

impl<TInput: FnOutput<bool> + FnReset> FnOutput<u128> for FnCount<TInput> {
    ///
    fn out(&mut self) -> u128 {
        // debug!("FnCount.out | input: {:?}", self.input.print());
        let value = self.input.borrow_mut().out();
        self.state.add(value);
        let state = self.state.state();
        trace!("FnCount.out | input.out: {:?}   | state: {:?}", &value, state);
        if state {
            self.count += 1;
        }
        self.count
    }
}

impl<TInput: FnOutput<bool> + FnReset> FnReset for FnCount<TInput> {
    fn reset(&mut self) {
        self.count = self.initial;
        self.state.reset();
        self.input.borrow_mut().reset();
    }
}

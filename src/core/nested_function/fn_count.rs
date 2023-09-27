#![allow(non_snake_case)]

use log::debug;
use std::{cell::RefCell, rc::Rc};

use super::fn_::FnOutput;

///
/// Counts number of raised fronts of boolean input
pub struct FnCount<TIn> {
    input: Rc<RefCell<dyn FnOutput<bool>>>,
    inputValue: bool,
    count: TIn,
}

impl<TIn> FnCount<TIn> {
    pub fn new(initial: TIn, input: Rc<RefCell<dyn FnOutput<bool>>>) -> Self {
        Self { 
            input,
            inputValue: false,
            count: initial ,
        }
    }
}

impl FnOutput<u128> for FnCount<u128> {
    ///
    fn out(&mut self) -> u128 {
        // debug!("FnCount.out | input: {:?}", self.input.print());
        let value = self.input.borrow_mut().out();
        debug!("FnCount.out | input.out: {:?}", &value);
        if (!self.inputValue) && value {
            self.count += 1;
        }
        self.inputValue = value;
        self.count
    }
}


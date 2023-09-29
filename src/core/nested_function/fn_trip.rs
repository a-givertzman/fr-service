#![allow(non_snake_case)]

use log::trace;
use std::{cell::RefCell, rc::Rc, fmt::Debug};

use super::{fn_::FnOutput, fn_reset::FnReset};


///
/// Returns true on input grater then setpoint
pub struct FnTripGe<TIn, TInput> where TInput: FnOutput<TIn> + FnReset {
    // input: Rc<RefCell<dyn FnOutput<TIn>>>,
    input: Rc<RefCell<TInput>>,
    setpoint: TIn,
    trip: bool,
    initial: bool,
}

impl<TIn, TInput: FnOutput<TIn> + FnReset> FnTripGe<TIn, TInput> {
    #[allow(dead_code)]
    pub fn new(initial: bool, input: Rc<RefCell<TInput>>, setpoint: TIn) -> Self {
        Self { 
            input,
            setpoint,
            trip: initial,
            initial: initial,
        }
    }
}


impl<T: PartialOrd + Debug, TInput: FnOutput<T> + FnReset> FnOutput<bool> for FnTripGe<T, TInput> {
    ///
    fn out(&mut self) -> bool {
        // debug!("FnTrip.out | input: {:?}", self.input.print());
        let value = self.input.borrow_mut().out();
        trace!("FnTrip.out | input.out: {:?}", &value);
        if self.trip {
            if value < self.setpoint {
                self.trip = false;
            }
        } else {
            if value >= self.setpoint {
                self.trip = true;
            }
        }
        self.trip
    }
}

impl<T, TInput: FnOutput<T> + FnReset> FnReset for FnTripGe<T, TInput> {
    fn reset(&mut self) {
        self.trip = self.initial;
        self.input.borrow_mut().reset();
    }
}

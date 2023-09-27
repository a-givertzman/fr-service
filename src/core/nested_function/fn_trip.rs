#![allow(non_snake_case)]

use log::debug;
use std::{cell::RefCell, rc::Rc, fmt::Debug};

use super::fn_::FnOutput;


///
/// Returns true on input grater then setpoint
pub struct FnTripGe<TIn> {
    input: Rc<RefCell<dyn FnOutput<TIn>>>,
    setpoint: TIn,
    // inputValue: TIn,
    trip: bool,
}

impl<TIn> FnTripGe<TIn> {
    pub fn new(initial: bool, input: Rc<RefCell<dyn FnOutput<TIn>>>, setpoint: TIn) -> Self {
        Self { 
            input,
            setpoint,
            // inputValue: TIn::zero(),
            trip: initial ,
        }
    }
}


impl<T: PartialOrd + Debug> FnOutput<bool> for FnTripGe<T> {
    ///
    fn out(&mut self) -> bool {
        // debug!("FnTrip.out | input: {:?}", self.input.print());
        let value = self.input.borrow_mut().out();
        debug!("FnTrip.out | input.out: {:?}", &value);
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


// impl<Integer> FnOutput<bool> for FnTripGe<Integer> {
//     ///
//     fn out(&mut self) -> bool {
//         // debug!("FnTrip.out | input: {:?}", self.input.print());
//         let value = self.input.borrow_mut().out();
//         debug!("FnTrip.out | input.out: {:?}", &value);
//         if self.trip {
//             if value < self.setpoint {
//                 self.trip = false;
//             }
//         } else {
//             if value >= self.setpoint {
//                 self.trip = true;
//             }
//         }
//         self.trip
//     }
// }
// impl<Float> FnOutput<bool> for FnTripGe<Float> {
//     ///
//     fn out(&mut self) -> bool {
//         // debug!("FnTrip.out | input: {:?}", self.input.print());
//         let value = self.input.borrow_mut().out();
//         debug!("FnTrip.out | input.out: {:?}", &value);
//         if self.trip {
//             if value < self.setpoint {
//                 self.trip = false;
//             }
//         } else {
//             if value >= self.setpoint {
//                 self.trip = true;
//             }
//         }
//         self.trip
//     }
// }

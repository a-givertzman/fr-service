#![allow(non_snake_case)]

use log::trace;
use std::{cell::RefCell, rc::Rc, fmt::Debug};

use crate::core_::{point::{point_type::PointType, point::Point}, types::{type_of::DebugTypeOf, bool::Bool}};

use super::fn_::{FnInOut, FnIn, FnOut};


///
/// Returns true on input grater then setpoint
#[derive(Debug)]
pub struct FnTripGe {
    id: String,
    input: Rc<RefCell<Box<dyn FnInOut>>>,
    setpoint: f64,
    initial: bool,
}
///
/// 
impl FnTripGe {
    #[allow(dead_code)]
    pub fn new(id: &str, initial: bool, input: Rc<RefCell<Box<dyn FnInOut>>>, setpoint: f64) -> Self {
        Self { 
            id: id.into(),
            input,
            setpoint,
            initial: initial,
        }
    }
}
///
/// 
impl FnIn for FnTripGe {
    fn add(&mut self, _: PointType) {
        panic!("FnTimer.add | method is not used")
    }
}
///
///
impl FnOut for FnTripGe {
    //
    //
    fn out(&mut self) -> PointType {
        // debug!("FnTrip.out | input: {:?}", self.input.print());
        let point = self.input.borrow_mut().out();        
        let value: bool = match &point {
            PointType::Bool(point) => {
                let value = if point.value.0 {1.0} else {0.0};
                value > self.setpoint
            },
            PointType::Int(point) => {
                point.value as f64 > self.setpoint
            },
            PointType::Float(point) => {
                point.value > self.setpoint
            },
            _ => panic!("FnCount.out | {:?} type is not supported: {:?}", point.typeOf(), point),
        };
        trace!("FnTrip.out | input.out: {:?}", &value);
        PointType::Bool(
            Point::<Bool> {
                name: String::from("FnTripGe"),
                value: Bool(value),
                status: point.status(),
                timestamp: point.timestamp(),
            }
        )
    }
    //
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnTripGe {}
#![allow(non_snake_case)]

use log::{trace, debug};

use crate::core_::{point::{point_type::PointType, point::Point}, types::{type_of::DebugTypeOf, bool::Bool, fn_in_out_ref::FnInOutRef}};

use super::fn_::{FnInOut, FnIn, FnOut};


///
/// Greater than function
/// Ge ( input1, input2 ) === input1.value >= input2.value
#[derive(Debug)]
pub struct FnTripGe {
    id: String,
    input1: FnInOutRef,
    input2: FnInOutRef,
}
///
/// 
impl FnTripGe {
    #[allow(dead_code)]
    pub fn new(id: &str, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        Self { 
            id: id.into(),
            input1,
            input2,
        }
    }
    ///
    /// 
    fn toFloat(&self, point: &PointType) -> f64 {
        match point {
            PointType::Bool(point) => {
                if point.value.0 {1.0} else {0.0}
            },
            PointType::Int(point) => {
                point.value as f64
            },
            PointType::Float(point) => {
                point.value
            },
            _ => panic!("FnCount.out | {:?} type is not supported: {:?}", point.typeOf(), point),
        }
    }
}
///
/// 
impl FnIn for FnTripGe {}
///
///
impl FnOut for FnTripGe {
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    fn inputs(&self) -> Vec<String> {
        self.input1.borrow().inputs()
    }
    //
    //
    fn out(&mut self) -> PointType {
        // debug!("FnTrip.out | input: {:?}", self.input.print());
        let point1 = self.input1.borrow_mut().out();     
        let point2 = self.input2.borrow_mut().out();    
        let value = self.toFloat(&point1) >= self.toFloat(&point2);
        debug!("FnTripGe.out | input.out: {:?}", &value);
        let status = match point1.status().cmp(&point2.status()) {
            std::cmp::Ordering::Less => point2.status(),
            std::cmp::Ordering::Equal => point1.status(),
            std::cmp::Ordering::Greater => point1.status(),
        };
        let timestamp = match point1.timestamp().cmp(&point2.timestamp()) {
            std::cmp::Ordering::Less => point2.timestamp(),
            std::cmp::Ordering::Equal => point1.timestamp(),
            std::cmp::Ordering::Greater => point1.timestamp(),
        };
        PointType::Bool(
            Point::<Bool> {
                name: String::from("FnTripGe"),
                value: Bool(value),
                status: status,
                timestamp: timestamp,
            }
        )
    }
    //
    //
    fn reset(&mut self) {
        self.input1.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnTripGe {}

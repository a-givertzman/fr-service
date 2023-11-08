#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};

use log::{trace, debug};

use crate::core_::{point::{point_type::PointType, point::Point}, types::{type_of::DebugTypeOf, bool::Bool, fn_in_out_ref::FnInOutRef}};

use super::{fn_::{FnInOut, FnIn, FnOut}, fn_kind::FnKind};


///
/// Function | Greater than or equal to
/// FnGe ( input1, input2 ) === input1.value >= input2.value
#[derive(Debug)]
pub struct FnGe {
    id: String,
    kind: FnKind,
    input1: FnInOutRef,
    input2: FnInOutRef,
}
static COUNT: AtomicUsize = AtomicUsize::new(0);
///
/// 
impl FnGe {
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        Self { 
            id: format!("{}{}", id.into(), COUNT.load(Ordering::Relaxed)),
            kind: FnKind::Fn,
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
impl FnIn for FnGe {}
///
///
impl FnOut for FnGe {
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    fn kind(&self) -> &FnKind {
        &self.kind
    }
    //
    fn inputs(&self) -> Vec<String> {
        let mut inputs = self.input1.borrow().inputs();
        inputs.extend(self.input2.borrow().inputs());
        inputs
    }
    //
    //
    fn out(&mut self) -> PointType {
        // debug!("FnTrip.out | input: {:?}", self.input.print());
        let point1 = self.input1.borrow_mut().out();     
        let point2 = self.input2.borrow_mut().out();    
        let value = self.toFloat(&point1) >= self.toFloat(&point2);
        debug!("{}.out | input.out: {:?}", self.id, &value);
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
                name: String::from(format!("{}.out", self.id)),
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
impl FnInOut for FnGe {}

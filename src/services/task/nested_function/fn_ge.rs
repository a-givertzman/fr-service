#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};

use log::debug;

use crate::core_::{point::{point::{Cot, Point}, point_type::PointType}, types::{bool::Bool, fn_in_out_ref::FnInOutRef, type_of::DebugTypeOf}};

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
///
/// 
impl FnGe {
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        Self { 
            id: format!("{}/FnGe{}", parent.into(), COUNT.load(Ordering::Relaxed)),
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
            _ => panic!("{}.out | {:?} type is not supported: {:?}", self.id, point.print_type_of(), point),
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
        let (txId, timestamp) = match point1.timestamp().cmp(&point2.timestamp()) {
            std::cmp::Ordering::Less => (point2.tx_id(), point2.timestamp()),
            std::cmp::Ordering::Equal => (point1.tx_id(), point1.timestamp()),
            std::cmp::Ordering::Greater => (point1.tx_id(), point1.timestamp()),
        };
        PointType::Bool(
            Point::<Bool> {
                tx_id: *txId,
                name: format!("{}.out", self.id),
                value: Bool(value),
                status: status,
                direction: Cot::Read,
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
///
/// 
pub static COUNT: AtomicUsize = AtomicUsize::new(0);

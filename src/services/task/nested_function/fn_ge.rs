use log::debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::core_::{
    cot::cot::Cot,
    point::{point::Point, point_type::PointType},
    types::{bool::Bool, fn_in_out_ref::FnInOutRef},
};
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
//
// 
impl FnGe {
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnGe{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input1,
            input2,
        }
    }
}
//
// 
impl FnIn for FnGe {}
//
//
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
        let input1 = self.input1.borrow_mut().out();     
        let input2 = self.input2.borrow_mut().out();    
        let value = input1 >= input2;
        debug!("{}.out | value: {:?}", self.id, &value);
        let status = match input1.status().cmp(&input2.status()) {
            std::cmp::Ordering::Less => input2.status(),
            std::cmp::Ordering::Equal => input1.status(),
            std::cmp::Ordering::Greater => input1.status(),
        };
        let (tx_id, timestamp) = match input1.timestamp().cmp(&input2.timestamp()) {
            std::cmp::Ordering::Less => (input2.tx_id(), input2.timestamp()),
            std::cmp::Ordering::Equal => (input1.tx_id(), input1.timestamp()),
            std::cmp::Ordering::Greater => (input1.tx_id(), input1.timestamp()),
        };
        PointType::Bool(
            Point::new(
                tx_id,
                &format!("{}.out", self.id),
                Bool(value),
                status,
                Cot::Inf,
                timestamp,
            )
        )
    }
    //
    //
    fn reset(&mut self) {
        self.input1.borrow_mut().reset();
        self.input2.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnGe {}
///
/// Global static counter of FnOut instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

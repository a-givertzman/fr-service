use log::debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::core_::{cot::cot::Cot, point::{point::Point, point_type::PointType}, types::{bool::Bool, fn_in_out_ref::FnInOutRef, type_of::DebugTypeOf}};
use super::{fn_::{FnIn, FnInOut, FnOut, FnResult}, fn_kind::FnKind};
use concat_string::concat_string;
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
        Self {
            id: format!("{}/FnGe{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input1,
            input2,
        }
    }
    ///
    ///
    fn to_double(&self, point: &PointType) -> f64 {
        match point {
            PointType::Bool(point) => {
                if point.value.0 {1.0} else {0.0}
            }
            PointType::Int(point) => {
                point.value as f64
            }
            PointType::Real(point) => {
                point.value as f64
            }
            PointType::Double(point) => {
                point.value
            }
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
    fn out(&mut self) -> FnResult {
        // debug!("FnTrip.out | input: {:?}", self.input.print());
        let input1 = self.input1.borrow_mut().out();
        let input2 = self.input2.borrow_mut().out();
        debug!("{}.out | input1: {:?}", self.id, input1);
        debug!("{}.out | value2: {:?}", self.id, input2);
        match (input1, input2) {
            (FnResult::Ok(point1), FnResult::Ok(point2)) => {
                let value = self.to_double(&point1) >= self.to_double(&point2);
                debug!("{}.out | input.out: {:?}", self.id, &value);
                let status = match point1.status().cmp(&point2.status()) {
                    std::cmp::Ordering::Less => point2.status(),
                    std::cmp::Ordering::Equal => point1.status(),
                    std::cmp::Ordering::Greater => point1.status(),
                };
                let (tx_id, timestamp) = match point1.timestamp().cmp(&point2.timestamp()) {
                    std::cmp::Ordering::Less => (point2.tx_id(), point2.timestamp()),
                    std::cmp::Ordering::Equal => (point1.tx_id(), point1.timestamp()),
                    std::cmp::Ordering::Greater => (point1.tx_id(), point1.timestamp()),
                };
                FnResult::Ok(PointType::Bool(
                    Point::<Bool> {
                        tx_id: *tx_id,
                        name: format!("{}.out", self.id),
                        value: Bool(value),
                        status,
                        cot: Cot::Inf,
                        timestamp,
                    }
                ))
            }
            (FnResult::Ok(_), FnResult::Err(err)) => FnResult::Err(err),
            (FnResult::Err(err), FnResult::Ok(_)) => FnResult::Err(err),
            (FnResult::Err(err1), FnResult::Err(err2)) => FnResult::Err(concat_string!(err1, "\n", err2)),
            (FnResult::Err(_), FnResult::None) => FnResult::None,
            (FnResult::Ok(_), FnResult::None) => FnResult::None,
            (FnResult::None, FnResult::Ok(_)) => FnResult::None,
            (FnResult::None, FnResult::Err(_)) => FnResult::None,
            (FnResult::None, FnResult::None) => FnResult::None,
        }
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
/// Global static counter of FnOut instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

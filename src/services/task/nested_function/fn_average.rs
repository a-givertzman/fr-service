use std::sync::atomic::{AtomicUsize, Ordering};
use log::{debug, trace};
use crate::core_::{
    cot::cot::Cot, point::{point::Point, point_type::PointType}, types::{fn_in_out_ref::FnInOutRef, type_of::DebugTypeOf}
};
use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};
///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnAverage {
    id: String,
    kind: FnKind,
    enable: FnInOutRef,
    input: FnInOutRef,
    count: i64,
    sum: f64,
}
///
/// 
impl FnAverage {
    ///
    /// Creates new instance of the FnAverage
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, enable: FnInOutRef, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnAverage{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind:FnKind::Fn,
            enable,
            input,
            count: 0,
            sum: 0.0,
        }
    }
}
///
/// 
impl FnIn for FnAverage {}
///
/// 
impl FnOut for FnAverage {
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
        self.input.borrow().inputs()
    }
    ///
    fn out(&mut self) -> PointType {
        let enable = match self.input.borrow_mut().out() {
            PointType::Bool(point) => point.value.0,
            PointType::Int(point) => point.value > 0,
            PointType::Real(point) => point.value > 0.0,
            PointType::Double(point) => point.value > 0.0,
            PointType::String(_) => panic!("{}.out | Type 'String' - is not supported for 'enable'", self.id),
        };
        // trace!("{}.out | enable: {:?}", self.id, enable);
        let input = self.input.borrow_mut().out();
        // trace!("{}.out | input: {:?}", self.id, input);
        if enable {
            let value = match &input {
                PointType::Bool(point) => if point.value.0 {1.0} else {0.0},
                PointType::Int(point) => point.value as f64,
                PointType::Real(point) => point.value as f64,
                PointType::Double(point) => point.value,
                _ => panic!("{}.out | {:?} type is not supported: {:?}", self.id,  input.print_type_of(), input),
            };
            self.sum += value;
            self.count += 1;
        } else {
            self.count = 0;
            self.sum = 0.0;
        }
        let average = self.sum / (self.count as f64);
        debug!("{}.out | average: {:?}", self.id, average);
        PointType::Double(
            Point {
                tx_id: *input.tx_id(),
                name: self.id.clone(),
                value: average,
                status: input.status(),
                cot: input.cot(),
                timestamp: input.timestamp(),
            }
        )
    }
    fn reset(&mut self) {
        self.count = 0;
        self.sum = 0.0;
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnAverage {}
///
/// Global static counter of FnAverage instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

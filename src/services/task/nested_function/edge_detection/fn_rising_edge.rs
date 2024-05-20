use log::debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    core_::{point::{point::Point, point_type::PointType}, types::{bool::Bool, fn_in_out_ref::FnInOutRef}},
    services::task::nested_function::{
        fn_::{FnIn, FnInOut, FnOut},
        fn_kind::FnKind,
    },
};
///
/// Function | Returns true one tic (single computation cycle)
/// if input value rising false -> true (0 (or any negative) -> any positive)
#[derive(Debug)]
pub struct FnRisingEdge {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    prev: Point<Bool>,
}
///
/// 
impl FnRisingEdge {
    ///
    /// Creates new instance of the FnRisingEdge
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnRisingEdge{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
            prev: Point::new_bool(0, "inner", false)
        }
    }    
}
///
/// 
impl FnIn for FnRisingEdge {}
///
/// 
impl FnOut for FnRisingEdge { 
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
    //
    //
    fn out(&mut self) -> PointType {
        let input = self.input.borrow_mut().out();
        debug!("{}.out | input: {:#?}", self.id, input);
        let (name, value, tx_id, timestamp, status, cot) = match input {
            PointType::Bool(point) => (point.name, point.value.0, point.tx_id, point.timestamp, point.status, point.cot),
            PointType::Int(point) => (point.name, point.value > 0, point.tx_id, point.timestamp, point.status, point.cot),
            PointType::Real(point) => (point.name, point.value > 0.0, point.tx_id, point.timestamp, point.status, point.cot),
            PointType::Double(point) => (point.name, point.value > 0.0, point.tx_id, point.timestamp, point.status, point.cot),
            PointType::String(_) => panic!("{}.out | String input value does not supported", self.id),
        };
        let value = PointType::Bool(Point::new(
            tx_id,
            &name,
            Bool(value & (! self.prev.value.0)),
            status,
            cot,
            timestamp,
        ));
        debug!("{}.out | value: {:#?}", self.id, value);
        value
    }
    //
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnRisingEdge {}
///
/// Global static counter of FnRisingEdge instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

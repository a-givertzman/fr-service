use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use crate::core_::{
    cot::cot::Cot, point::{point::Point, point_type::PointType}, types::{fn_in_out_ref::FnInOutRef, type_of::DebugTypeOf}
};
use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};
///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnCount {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    prev: bool,
    count: i64,
    initial: i64,
}
//
// 
impl FnCount {
    ///
    /// Creates new instance of the FnCount
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, initial: i64, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnCount{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind:FnKind::Fn,
            input,
            prev: false,
            count: initial,
            initial,
        }
    }
}
//
// 
impl FnIn for FnCount {}
//
// 
impl FnOut for FnCount {
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
        // trace!("{}.out | input: {:?}", self.id, self.input.print());
        let input = self.input.borrow_mut().out();
        let value = input.to_bool().as_bool().value.0;
        if !self.prev && value {
            self.count += 1;
        }
        self.prev = value;
        trace!("{}.out | input.out: {:?}   | state: {:?}", self.id, &value, self.count);
        PointType::Int(
            Point::new(
                *input.tx_id(),
                &format!("{}.out", self.id),
                self.count,
                input.status(),
                input.cot(),
                input.timestamp(),
            )
        )
    }
    fn reset(&mut self) {
        self.count = self.initial;
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnCount {}
///
/// Global static counter of FnCount instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

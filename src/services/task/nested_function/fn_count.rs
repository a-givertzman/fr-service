#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};

use log::trace;

use crate::core_::{
    types::{type_of::DebugTypeOf, fn_in_out_ref::FnInOutRef},
    point::{point_type::PointType, point::Point}, 
};

use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};


///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnCount {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    count: f64,
    initial: f64,
}
///
/// 
impl FnCount {
    ///
    /// Creates new instance of the FnCount
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>, initial: f64, input: FnInOutRef) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        let id = "FnCount";
        Self { 
            id: format!("{}{}", id, COUNT.load(Ordering::Relaxed)),
            kind:FnKind::Fn,
            input,
            count: initial.clone(),
            initial: initial,
        }
    }
}
///
/// 
impl FnIn for FnCount {}
///
/// 
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
        // trace!("FnCount.out | input: {:?}", self.input.print());
        let point = self.input.borrow_mut().out();
        let value = match &point {
            PointType::Bool(point) => if point.value.0 {1.0} else {0.0},
            PointType::Int(point) => point.value as f64,
            PointType::Float(point) => point.value,
            _ => panic!("FnCount.out | {:?} type is not supported: {:?}", point.typeOf(), point),
        };
        self.count += value;
        trace!("FnCount.out | input.out: {:?}   | state: {:?}", &value, self.count);
        PointType::Float(
            Point {
                txId: point.txId(),
                name: String::from(format!("{}.out", self.id)),
                value: self.count,
                status: point.status(),
                timestamp: point.timestamp(),
            }
        )
    }
    fn reset(&mut self) {
        self.count = self.initial;
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnCount {}
///
///
static COUNT: AtomicUsize = AtomicUsize::new(0);
pub fn resetCount() {
    COUNT.store(0, Ordering::SeqCst)
}

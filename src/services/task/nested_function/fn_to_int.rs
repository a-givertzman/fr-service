use log::trace;
use std::sync::atomic::{AtomicUsize, Ordering};
use concat_string::concat_string;
use crate::{
    core_::{point::{point::Point, point_type::PointType}, types::{fn_in_out_ref::FnInOutRef, type_of::DebugTypeOf}},
    services::task::nested_function::{
        fn_::{FnIn, FnInOut, FnOut},
        fn_kind::FnKind,
    },
};
///
/// Function converts input to Int
///  - bool: true -> 1, false -> 0
///  - real: 0.1 -> 0 | 0.5 -> 0 | 0.9 -> 0 | 1.1 -> 1
///  - string: try to parse int
#[derive(Debug)]
pub struct FnToInt {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
}
//
// 
impl FnToInt {
    ///
    /// Creates new instance of the FnToInt
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnToInt{}", parent.into(), COUNT.fetch_add(1, Ordering::SeqCst)),
            kind: FnKind::Fn,
            input,
        }
    }    
}
//
// 
impl FnIn for FnToInt {}
//
// 
impl FnOut for FnToInt { 
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
        let point = self.input.borrow_mut().out();
        trace!("{}.out | input: {:?}", self.id, point);
        let out = match &point {
            PointType::Bool(value) => {
                if value.value.0 {1} else {0}
            }
            PointType::Int(value) => {
                value.value
            }
            PointType::Real(value) => {
                value.value.trunc() as i64
            }
            PointType::Double(value) => {
                value.value.trunc() as i64
            }
            _ => panic!("{}.out | {:?} type is not supported: {:?}", self.id, point.print_type_of(), point),
        };
        trace!("{}.out | out: {:?}", self.id, &out);
        PointType::Int(
            Point {
                tx_id: *point.tx_id(),
                name: concat_string!(self.id, ".out"),
                value: out,
                status: point.status(),
                cot: point.cot(),
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
//
// 
impl FnInOut for FnToInt {}
///
/// Global static counter of FnToInt instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

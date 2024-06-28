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
/// Function converts input to Real
///  - bool: true -> 1.0, false -> 0.0
///  - string: try to parse Real
#[derive(Debug)]
pub struct FnToReal {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
}
//
// 
impl FnToReal {
    ///
    /// Creates new instance of the FnToReal
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnToReal{}", parent.into(), COUNT.fetch_add(1, Ordering::SeqCst)),
            kind: FnKind::Fn,
            input,
        }
    }    
}
//
// 
impl FnIn for FnToReal {}
//
// 
impl FnOut for FnToReal { 
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
    fn out(&mut self) -> FnResult<PointType, String> {
        let point = self.input.borrow_mut().out();
        trace!("{}.out | input: {:?}", self.id, point);
        let out = match &point {
            PointType::Bool(value) => {
                if value.value.0 {1.0f32} else {0.0f32}
            }
            PointType::Int(value) => {
                value.value as f32
            }
            PointType::Real(value) => {
                value.value
            }
            PointType::Double(value) => {
                value.value as f32
            }
            _ => panic!("{}.out | {:?} type is not supported: {:?}", self.id, point.print_type_of(), point),
        };
        trace!("{}.out | out: {:?}", self.id, &out);
        PointType::Real(
            Point::new(
                point.tx_id(),
                &concat_string!(self.id, ".out"),
                out,
                point.status(),
                point.cot(),
                point.timestamp(),
            )
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
impl FnInOut for FnToReal {}
///
/// Global static counter of FnToReal instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

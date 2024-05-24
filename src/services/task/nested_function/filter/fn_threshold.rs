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
/// Function | Returns filtered input
/// - if factor is not specified:
///     - new input value returned if (prev - [input]) > [threshold]
/// - if factor is specified:
///     - each cycle: delta = (prev - [input]) * factor
///     - new input value returned if delta > [threshold]
#[derive(Debug)]
pub struct FnThreshold {
    id: String,
    kind: FnKind,
    threshold: FnInOutRef,
    factor: Option<FnInOutRef>,
    input: FnInOutRef,
    delta: f64,
}
//
// 
impl FnThreshold {
    ///
    /// Creates new instance of the FnThreshold
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, threshold: FnInOutRef, factor: Option<FnInOutRef>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnThreshold{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
            prev: false,
        }
    }    
}
//
// 
impl FnIn for FnThreshold {}
//
// 
impl FnOut for FnThreshold { 
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
        let input_value = input.to_bool().as_bool().value.0;
        let value = PointType::Bool(Point::new(
            *input.tx_id(),
            &input.name(),
            Bool((! input_value) & self.prev),
            input.status(),
            input.cot(),
            input.timestamp(),
        ));
        self.prev = input_value;
        debug!("{}.out | value: {:#?}", self.id, value);
        value
    }
    //
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
        self.prev = false;
    }
}
//
// 
impl FnInOut for FnThreshold {}
///
/// Global static counter of FnThreshold instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

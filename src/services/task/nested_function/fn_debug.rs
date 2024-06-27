use chrono::Utc;
use log::debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    core_::{cot::cot::Cot, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status, types::fn_in_out_ref::FnInOutRef},
    services::task::nested_function::{
        fn_::{FnIn, FnInOut, FnOut},
        fn_kind::FnKind,
    },
};
///
/// Function | Just doing debug of values coming from inputs
/// - Returns value from the last input
#[derive(Debug)]
pub struct FnDebug {
    id: String,
    kind: FnKind,
    inputs: Vec<FnInOutRef>,
}
//
// 
impl FnDebug {
    ///
    /// Creates new instance of the FnDebug
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, inputs: Vec<FnInOutRef>) -> Self {
        Self { 
            id: format!("{}/FnDebug{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            inputs,
        }
    }    
}
//
// 
impl FnIn for FnDebug {}
//
// 
impl FnOut for FnDebug { 
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
        let mut inputs = vec![];
        for input in &self.inputs {
            inputs.append(&mut input.borrow().inputs());
        }
        inputs
    }
    //
    //
    fn out(&mut self) -> PointType {
        let mut value = PointType::String(Point::new(
            PointTxId::from_str(&self.id),
            &self.id, "No inputs to get the value".to_owned(),
            Status::Ok,
            Cot::Inf,
            Utc::now(),
        ));
        for input in &self.inputs {
            value = input.borrow_mut().out();
            debug!("{}.out | value: {:#?}", self.id, value);
        }
        value
    }
    //
    //
    fn reset(&mut self) {
        for input in &self.inputs {
            input.borrow_mut().reset();
        }
    }
}
//
// 
impl FnInOut for FnDebug {}
///
/// Global static counter of FnDebug instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

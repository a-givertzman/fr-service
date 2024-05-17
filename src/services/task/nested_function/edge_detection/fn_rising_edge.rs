use log::debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    core_::{point::point_type::PointType, types::fn_in_out_ref::FnInOutRef},
    services::task::nested_function::{
        fn_::{FnInOut, FnIn, FnOut},
        fn_kind::FnKind,
    },
};
///
/// Function just doing debug of value coming from input
#[derive(Debug)]
pub struct FnDebug {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
}
///
/// 
impl FnDebug {
    ///
    /// Creates new instance of the FnDebug
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnDebug{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
        }
    }    
}
///
/// 
impl FnIn for FnDebug {}
///
/// 
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
        self.input.borrow().inputs()
    }
    //
    //
    fn out(&mut self) -> PointType {
        let value = self.input.borrow_mut().out();
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
impl FnInOut for FnDebug {}
///
/// Global static counter of FnDebug instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

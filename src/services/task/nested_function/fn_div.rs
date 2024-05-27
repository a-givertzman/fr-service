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
/// Function | Returns input1 / input2
#[derive(Debug)]
pub struct FnDiv {
    id: String,
    kind: FnKind,
    input1: FnInOutRef,
    input2: FnInOutRef,
}
//
// 
impl FnDiv {
    ///
    /// Creates new instance of the FnDiv
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnDiv{}", parent.into(), COUNT.fetch_add(1, Ordering::SeqCst)),
            kind: FnKind::Fn,
            input1,
            input2,
        }
    }    
}
//
// 
impl FnIn for FnDiv {}
//
// 
impl FnOut for FnDiv { 
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
    fn out(&mut self) -> PointType {
        // TODO Mul overflow check
        let input1 = self.input1.borrow_mut().out();
        debug!("{}.out | input1: {:?}", self.id, &input1);
        let input2 = self.input2.borrow_mut().out();
        debug!("{}.out | input2: {:?}", self.id, &input2);
        let out = input1 / input2;
        debug!("{}.out | out: {:?}", self.id, &out);
        out
    }
    //
    //
    fn reset(&mut self) {
        self.input1.borrow_mut().reset();
        self.input2.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnDiv {}
///
/// Global static counter of FnDiv instances
static COUNT: AtomicUsize = AtomicUsize::new(1);
use log::trace;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    core_::{point::point_type::PointType, types::fn_in_out_ref::FnInOutRef},
    services::task::nested_function::{
        fn_::{FnInOut, FnIn, FnOut},
        fn_kind::FnKind,
    },
};
///
/// Function | Returns input1 - input2
#[derive(Debug)]
pub struct FnSub {
    id: String,
    kind: FnKind,
    input1: FnInOutRef,
    input2: FnInOutRef,
}
//
// 
impl FnSub {
    ///
    /// Creates new instance of the FnSub
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnSub{}", parent.into(), COUNT.fetch_add(1, Ordering::SeqCst)),
            kind: FnKind::Fn,
            input1,
            input2,
        }
    }    
}
//
// 
impl FnIn for FnSub {}
//
// 
impl FnOut for FnSub { 
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
        // TODO Add overflow check
        let input1 = self.input1.borrow_mut().out();
        trace!("{}.out | input1: {:?}", self.id, &input1);
        let input2 = self.input2.borrow_mut().out();
        trace!("{}.out | input2: {:?}", self.id, &input2);
        let out = input1 - input2;
        trace!("{}.out | out: {:?}", self.id, &out);
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
impl FnInOut for FnSub {}
///
/// Global static counter of FnSub instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

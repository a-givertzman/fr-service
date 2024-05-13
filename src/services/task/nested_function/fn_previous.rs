use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use crate::{
    core_::
        types::fn_in_out_ref::FnInOutRef
    ,
    services::task::nested_function::{
        fn_::{FnIn, FnInOut, FnOut, FnResult},
        fn_kind::FnKind,
    }
};
///
/// Function | Returns:
///  - if input is Ok - value from input
///  - if input is Err - value from input
///  - if input is None - stored previous value
#[derive(Debug)]
pub struct FnPrevious {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    state: FnResult,
}
///
/// 
impl FnPrevious {
    ///
    /// Creates new instance of the FnPrevious
    // #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnPrevious{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
            state: FnResult::None,
        }
    }    
}
///
/// 
impl FnIn for FnPrevious {}
///
/// 
impl FnOut for FnPrevious { 
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
    fn out(&mut self) -> FnResult {
        let input = self.input.borrow_mut().out();
        trace!("{}.out | value: {:?}", self.id, input);
        match &input {
            FnResult::Ok(_) => {
                self.state = input.clone();
                input
            }
            FnResult::Err(_) => input,
            FnResult::None => self.state.clone(),
        }
    }
    //
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnPrevious {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

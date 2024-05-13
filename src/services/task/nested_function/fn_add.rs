use log::{debug, trace};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::{
    core_::{point::point_type::PointType, types::{type_of::DebugTypeOf, fn_in_out_ref::FnInOutRef}},
    services::task::nested_function::{
        fn_::{FnInOut, FnIn, FnOut},
        fn_kind::FnKind,
    },
};
use concat_string::concat_string;
use super::fn_::FnResult;
///
/// Function do Add of input1 and input2
#[derive(Debug)]
pub struct FnAdd {
    id: String,
    kind: FnKind,
    input1: FnInOutRef,
    input2: FnInOutRef,
}
///
/// 
impl FnAdd {
    ///
    /// Creates new instance of the FnAdd
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnAdd{}", parent.into(), COUNT.fetch_add(1, Ordering::SeqCst)),
            kind: FnKind::Fn,
            input1,
            input2,
        }
    }    
}
///
/// 
impl FnIn for FnAdd {}
///
/// 
impl FnOut for FnAdd { 
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
    fn out(&mut self) -> FnResult {
        // TODO Add overflow check
        let input1 = self.input1.borrow_mut().out();
        let input2 = self.input2.borrow_mut().out();
        debug!("{}.out | input1: {:?}", self.id, input1);
        debug!("{}.out | input2: {:?}", self.id, input2);
        match (input1, input2) {
            (FnResult::Ok(value1), FnResult::Ok(value2)) => {
                let out = match value1 {
                    PointType::Bool(value1) => {
                        PointType::Bool(value1 + value2.as_bool())
                    }
                    PointType::Int(value1) => {
                        PointType::Int(value1 + value2.as_int())
                    }
                    PointType::Real(value1) => {
                        PointType::Real(value1 + value2.as_real())
                    }
                    PointType::Double(value1) => {
                        PointType::Double(value1 + value2.as_double())
                    }
                    _ => panic!("{}.out | {:?} type is not supported: {:?}", self.id, value1.print_type_of(), value1),
                };
                trace!("{}.out | out: {:?}", self.id, &out);
                FnResult::Ok(out)
            },
            (FnResult::Ok(_), FnResult::Err(err)) => FnResult::Err(err),
            (FnResult::Err(err), FnResult::Ok(_)) => FnResult::Err(err),
            (FnResult::Err(err1), FnResult::Err(err2)) => FnResult::Err(concat_string!(err1, "\n", err2)),
            (FnResult::Err(_), FnResult::None) => FnResult::None,
            (FnResult::Ok(_), FnResult::None) => FnResult::None,
            (FnResult::None, FnResult::Ok(_)) => FnResult::None,
            (FnResult::None, FnResult::Err(_)) => FnResult::None,
            (FnResult::None, FnResult::None) => FnResult::None,
        }
    }
    //
    //
    fn reset(&mut self) {
        self.input1.borrow_mut().reset();
        self.input2.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnAdd {}
///
/// Global static counter of FnAdd instances
static COUNT: AtomicUsize = AtomicUsize::new(1);






// pub struct FnMul;
// pub struct FnOr;
// pub struct FnCompare;

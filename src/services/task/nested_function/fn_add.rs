#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};

use log::trace;

use crate::core_::{point::point_type::PointType, types::{type_of::DebugTypeOf, fn_in_out_ref::FnInOutRef}};

use super::{fn_::{FnInOut, FnIn, FnOut}, fn_kind::FnKind};

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
    /// Creates new instance of the FnCount
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>, input1: FnInOutRef, input2: FnInOutRef) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        Self { 
            id: format!("{}{}", id.into(), COUNT.load(Ordering::Relaxed)),
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
    fn out(&mut self) -> PointType {
        // TODO Add overflow check
        let value1 = self.input1.borrow_mut().out();
        trace!("FnAdd({}).out | value1: {:?}", self.id, &value1);
        let value2 = self.input2.borrow_mut().out();
        trace!("FnAdd({}).out | value2: {:?}", self.id, &value2);
        let out = match value1 {
            PointType::Bool(value1) => {
                PointType::Bool(value1 + value2.asBool())
            },
            PointType::Int(value1) => {
                PointType::Int(value1 + value2.asInt())
            },
            PointType::Float(value1) => {
                PointType::Float(value1 + value2.asFloat())
            },
            _ => panic!("FnCount.out | {:?} type is not supported: {:?}", value1.printTypeOf(), value1),
        };
        trace!("FnAdd({}).out | out: {:?}", self.id, &out);
        out
    }
    //
    //
    fn reset(&mut self) {
        todo!()
    }
}
///
/// 
impl FnInOut for FnAdd {}
///
/// 
static COUNT: AtomicUsize = AtomicUsize::new(0);
pub fn resetCount() {
    COUNT.store(0, Ordering::SeqCst)
}




















pub struct FnMul;
pub struct FnOr;
pub struct FnCompare;

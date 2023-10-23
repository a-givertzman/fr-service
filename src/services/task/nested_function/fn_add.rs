#![allow(non_snake_case)]


use std::{rc::Rc, cell::RefCell};

use log::trace;

use crate::core_::{point::point_type::PointType, types::type_of::DebugTypeOf};

use super::fn_::{FnInOut, FnIn, FnOut};




///
/// Function do Add of input1 and input2
#[derive(Debug)]
pub struct FnAdd {
    id: String,
    input1: Rc<RefCell<Box<dyn FnInOut>>>,
    input2: Rc<RefCell<Box<dyn FnInOut>>>,
}
///
/// 
impl FnAdd {
    ///
    /// Creates new instance of the FnCount
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>, input1: Rc<RefCell<Box<dyn FnInOut>>>, input2: Rc<RefCell<Box<dyn FnInOut>>>) -> Self {
        Self { 
            id: id.into(),
            input1,
            input2,
        }
    }    
}
///
/// 
impl FnIn for FnAdd {
    fn add(&mut self, _: PointType) {
        panic!("FnAdd.add | method is not used")
    }
}
///
/// 
impl FnOut for FnAdd { 
    //
    //
    fn out(&mut self) -> PointType {
        let value1 = self.input1.borrow_mut().out();
        trace!("FnAdd({}).out | value1: {:?}", self.id, &value1);
        let value2 = self.input2.borrow_mut().out();
        trace!("FnAdd({}).out | value2: {:?}", self.id, &value2);
        let out = match value1 {
            PointType::Bool(value1) => {
                PointType::Bool(value1 | value2.asBool())
            },
            PointType::Int(value1) => {
                PointType::Int(value1 + value2.asInt())
            },
            PointType::Float(value1) => {
                PointType::Float(value1 + value2.asFloat())
            },
            _ => panic!("FnCount.out | {:?} type is not supported: {:?}", value1.typeOf(), value1),
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





















pub struct FnMul;
pub struct FnOr;
pub struct FnCompare;

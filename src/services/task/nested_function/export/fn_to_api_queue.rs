use std::{rc::Rc, cell::RefCell};

use crate::services::task::nested_function::fn_::{FnInOut, FnIn, FnOut};

///
/// Exports data from the input into the associated queue
#[derive(Debug)]
pub struct FnToApiQueue {
    id: String,
    input: Rc<RefCell<Box<dyn FnInOut>>>,

}
///
/// 
impl FnToApiQueue {
    ///
    /// creates new instance of the FnToApiQueue
    /// - id - just for proper debugging
    /// - input - incoming points
    pub fn new(id: impl Into<String>, input: Rc<RefCell<Box<dyn FnInOut>>>) -> Self {
        Self {  
            id: id.into(),
            input,
        }
    }
}
///
/// 
impl FnIn for FnToApiQueue {
    //
    fn add(&mut self, point: crate::core_::point::point_type::PointType) {
        panic!("FnToApiQueue.add | method is not used");
    }
}
///
/// 
impl FnOut for FnToApiQueue {
    //
    fn out(&mut self) -> crate::core_::point::point_type::PointType {
        todo!()
    }
    //
    fn reset(&mut self) {
        todo!()
    }
}
///
/// 
impl FnInOut for FnToApiQueue {}

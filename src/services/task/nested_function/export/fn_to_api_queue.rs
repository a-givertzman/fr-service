#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell, sync::mpsc::Sender};

use log::{error, trace};

use crate::{services::task::nested_function::fn_::{FnInOut, FnIn, FnOut}, core_::point::point_type::{PointType, ToPoint}};

///
/// Exports data from the input into the associated queue
#[derive(Debug)]
pub struct FnToApiQueue {
    id: String,
    input: Rc<RefCell<Box<dyn FnInOut>>>,
    sendQueue: Sender<PointType>,
    state: String,
}
///
/// 
impl FnToApiQueue {
    ///
    /// creates new instance of the FnToApiQueue
    /// - id - just for proper debugging
    /// - input - incoming points
    pub fn new(id: impl Into<String>, input: Rc<RefCell<Box<dyn FnInOut>>>, send: Sender<PointType>) -> Self {
        Self {  
            id: id.into(),
            input,
            sendQueue: send,
            state: String::new(),
        }
    }
}
///
/// 
impl FnIn for FnToApiQueue {
    //
    fn add(&mut self, _: PointType) {
        panic!("FnToApiQueue.add | method is not used");
    }
}
///
/// 
impl FnOut for FnToApiQueue {
    //
    fn out(&mut self) -> PointType {
        let point = self.input.borrow_mut().out();
        let sql = point.asString().value;
        if sql != self.state {
            self.state = sql.clone();
            match self.sendQueue.send(point.clone()) {
                Ok(_) => {
                    trace!("FnToApiQueue.out | sql sent to queueu successfully");
                },
                Err(err) => {
                    error!("FnToApiQueue.out | Error sending to queue sql: {}\n\terror: {:?}", &sql, err);
                },
            };
        }
        point
    }
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnToApiQueue {}

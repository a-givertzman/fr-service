#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell};

use log::warn;

use crate::core_::point::point_type::PointType;

use super::{nested_function::fn_::{FnInOut, FnIn, FnOut}, queue_send::QueueSend};

///
/// holds a reference to the associated queueu
#[derive(Debug)]
pub struct PushToQueue {
    queueu: Rc<RefCell<Box<dyn QueueSend<PointType>>>>,
    input: Rc<RefCell<Box<dyn FnInOut>>>,
    state: PointType,
    buffer: Vec<PointType>,
}

///
/// 
impl FnIn for PushToQueue {
    //
    //
    fn add(&mut self, _: PointType) {
        panic!("FnSum.add | method is not used")
    }
}
///
/// 
impl FnOut for PushToQueue {
    //
    //
    fn out(&mut self) -> PointType {
        let point = self. input.borrow_mut().out();
        if point != self.state {
            self.buffer.push(point.clone());
            self.state = point.clone();
        }
        let mut queue = self.queueu.borrow_mut();
        while let Some(point) = self.buffer.pop() {
            match queue.send(point.clone()) {
                Ok(_) => {},
                Err(err) => {
                    warn!("PushToQueue.out | error push to queue: {:?}", err);
                    self.buffer.insert(0, point);
                    break;
                },
            };
        }        
        point
    }
    //
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for PushToQueue {}

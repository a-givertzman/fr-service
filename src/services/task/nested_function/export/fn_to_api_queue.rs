use std::sync::{mpsc::Sender, atomic::{AtomicUsize, Ordering}};
use log::{debug, error};
use crate::{services::task::nested_function::{fn_::{FnInOut, FnIn, FnOut}, fn_kind::FnKind}, core_::{point::point_type::PointType, types::fn_in_out_ref::FnInOutRef}};
///
/// Exports data from the input into the associated queue
#[derive(Debug)]
pub struct FnToApiQueue {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    tx_send: Sender<PointType>,
    state: String,
}
///
/// 
static COUNT: AtomicUsize = AtomicUsize::new(1);
//
// 
impl FnToApiQueue {
    ///
    /// creates new instance of the FnToApiQueue
    /// - id - just for proper debugging
    /// - input - incoming points
    pub fn new(parent: impl Into<String>, input: FnInOutRef, send: Sender<PointType>) -> Self {
        Self {  
            id: format!("{}/FnToApiQueue{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
            tx_send: send,
            state: String::new(),
        }
    }
}
///
///  += 1
impl FnIn for FnToApiQueue {
    //
    fn add(&mut self, _: PointType) {
        panic!("{}.add | method is not used", self.id);
    }
}
//
// 
impl FnOut for FnToApiQueue {
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
    fn out(&mut self) -> PointType {
        let point = self.input.borrow_mut().out();
        let sql = point.as_string().value;
        if sql != self.state {
            self.state = sql.clone();
            match self.tx_send.send(point.clone()) {
                Ok(_) => {
                    debug!("{}.out | Sent sql: {}", self.id, sql);
                }
                Err(err) => {
                    error!("{}.out | Send error: {:?}\n\tsql: {:?}", self.id, err, sql);
                }
            };
        }
        point
    }
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnToApiQueue {}

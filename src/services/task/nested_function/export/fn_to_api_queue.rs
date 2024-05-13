use std::sync::{mpsc::Sender, atomic::{AtomicUsize, Ordering}};
use log::{debug, error};
use crate::{core_::{point::point_type::PointType, types::fn_in_out_ref::FnInOutRef}, services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut, FnResult}, fn_kind::FnKind}};
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
///
/// 
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
///
/// 
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
    fn out(&mut self) -> FnResult {
        match self.input.borrow_mut().out() {
            FnResult::Ok(point) => {
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
                FnResult::Ok(point)
            }
            FnResult::Err(err) => FnResult::Err(err),
            FnResult::None => FnResult::None,
        }
    }
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnToApiQueue {}

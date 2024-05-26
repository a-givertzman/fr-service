use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use crate::core_::{
    point::{point::Point, point_type::PointType},
    types::fn_in_out_ref::FnInOutRef,
};
use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};
///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnCount {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    prev: bool,
    count: Option<i64>,
    initial: Option<FnInOutRef>,
}
//
// 
impl FnCount {
    ///
    /// Creates new instance of the FnCount
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, initial: Option<FnInOutRef>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnCount{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind:FnKind::Fn,
            input,
            prev: false,
            count: None,
            initial,
        }
    }
}
//
// 
impl FnIn for FnCount {}
//
// 
impl FnOut for FnCount {
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
        let mut inputs = vec![];
        inputs.append(&mut self.input.borrow().inputs());
        if let Some(initial) = &self.initial {
            inputs.append(&mut initial.borrow().inputs());
        }
        inputs
    }
    ///
    fn out(&mut self) -> PointType {
        // trace!("{}.out | input: {:?}", self.id, self.input.print());
        let mut count = match self.count {
            Some(count) => count,
            None => {
                match &mut self.initial {
                    Some(initial) => {
                        initial.borrow_mut().out().as_int().value
                    },
                    None => 0,
                }
            }
        };
        let input = self.input.borrow_mut().out();
        let value = input.to_bool().as_bool().value.0;
        if !self.prev && value {
            count += 1;
            self.count = Some(count);
        }
        self.prev = value;
        trace!("{}.out | input.out: {:?}   | state: {:?}", self.id, &value, self.count);
        PointType::Int(
            Point::new(
                input.tx_id(),
                &format!("{}.out", self.id),
                count,
                input.status(),
                input.cot(),
                input.timestamp(),
            )
        )
    }
    fn reset(&mut self) {
        let initial = match &self.initial {
            Some(initial) => {
                initial.borrow_mut().reset();
                initial.borrow_mut().out().as_int().value
            }
            None => 0,
        };
        self.count = Some(initial);
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnCount {}
///
/// Global static counter of FnCount instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

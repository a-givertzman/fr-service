use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use crate::core_::{
    point::{point::Point, point_type::PointType},
    types::fn_in_out_ref::FnInOutRef,
};
use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};
///
/// Accumulates numeric incoming Point's value
/// - if input is not numeric - will panic
/// - if input is bool, false = 0, true = 1
#[derive(Debug)]
pub struct FnAcc {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    acc: Option<PointType>,
    initial: Option<FnInOutRef>,
}
//
// 
impl FnAcc {
    ///
    /// Creates new instance of the FnAcc
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, name: Option<String>, initial: Option<FnInOutRef>, input: FnInOutRef) -> Self {
        Self { 
            id: name.unwrap_or(format!("{}/FnAcc{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed))),
            kind:FnKind::Fn,
            input,
            acc: None,
            initial,
        }
    }
}
//
// 
impl FnIn for FnAcc {}
//
// 
impl FnOut for FnAcc {
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
        if let Some(initial) = &self.initial {
            inputs.append(&mut initial.borrow().inputs());
        }
        inputs.append(&mut self.input.borrow().inputs());
        inputs
    }
    ///
    fn out(&mut self) -> PointType {
        // trace!("{}.out | input: {:?}", self.id, self.input.print());
        let input = self.input.borrow_mut().out();
        let acc = match self.acc.clone() {
            Some(acc) => acc,
            None => {
                match &mut self.initial {
                    Some(initial) => {
                        initial.borrow_mut().out()
                    }
                    None => match input {
                        PointType::Bool(_) | PointType::Int(_)  => PointType::Int(Point::new(
                            *input.tx_id(), &input.name(), 0, input.status(), input.cot(), input.timestamp(),
                        )),
                        PointType::Real(_) => PointType::Real(Point::new(
                            *input.tx_id(), &input.name(), 0.0, input.status(), input.cot(), input.timestamp(),
                        )),
                        PointType::Double(_) => PointType::Double(Point::new(
                            *input.tx_id(), &input.name(), 0.0, input.status(), input.cot(), input.timestamp(),
                        )),
                        _ => panic!("{}.out | Invalit input type '{:?}'", self.id, input.type_()),
                    }
                }
            }
        };
        let acc = match acc {
            PointType::Bool(acc) => {
                PointType::Bool(acc + input.as_bool())
            }
            PointType::Int(acc) => {
                PointType::Int(acc + input.as_int())
            }
            PointType::Real(acc) => {
                PointType::Real(acc + input.as_real())
            }
            PointType::Double(acc) => {
                PointType::Double(acc + input.as_double())
            }
            _ => panic!("{}.out | Invalit type '{:?}'", self.id, acc.type_()),
        };
        trace!("{}.out | value: {:?}", self.id, acc);
        self.acc = Some(acc.clone());
        acc
    }
    fn reset(&mut self) {
        if let Some(initial) = &self.initial {
            initial.borrow_mut().reset();
        }
        self.acc = None;
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnAcc {}
///
/// Global static counter of FnAcc instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

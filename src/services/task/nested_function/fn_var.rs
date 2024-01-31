#![allow(non_snake_case)]

use std::sync::atomic::{Ordering, AtomicUsize};

use log::trace;

use crate::core_::{point::point_type::PointType, types::fn_in_out_ref::FnInOutRef};

use super::{fn_::{FnIn, FnOut, FnInOut}, fn_kind::FnKind};

///
/// Specific kinde of function
/// - has additional method .eval(), 
/// nothing returns, 
/// but evaluete all calculations,
/// result stores into inner
/// - calculated result returns in .out() method
/// - out() method do not evaluete calculations, just returns the result
#[derive(Debug, Clone)]
pub struct FnVar {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    result: Option<PointType>,
}
///
/// 
impl FnVar {
    pub fn new(parent: impl Into<String>, input: FnInOutRef) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            id: format!("{}/FnTimer{}", parent.into(), COUNT.load(Ordering::Relaxed)),
            kind: FnKind::Var,
            input: input,
            result: None, 
        }
    }
}
///
/// 
impl FnIn for FnVar {}
///
/// 
impl FnOut for FnVar {
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
    /// Returns nothing, 
    /// - Evaluetes all calculations,
    /// - Result stores into inner
    /// - calculated result returns in .out() method
    fn eval(&mut self) {
        trace!("FnVar({}).eval | evaluating...", self.id);
        self.result = Some(self.input.borrow_mut().out());
    }
    ///
    /// Do not evaluete calculations, 
    /// just returns the result if evalueted, else panic
    fn out(&mut self) -> PointType {
        match &self.result {
            Some(result) => {
                trace!("FnVar({}).out | value: {:?}", self.id, &self.result);
                result.clone()
            },
            None => {
                panic!("FnVar({}).out | not initialised", self.id);
            },
        }
    }
    //
    fn reset(&mut self) {
        self.result = None;
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnVar {}
///
/// 
static COUNT: AtomicUsize = AtomicUsize::new(0);

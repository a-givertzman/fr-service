#![allow(non_snake_case)]

use log::trace;
use std::fmt::Debug;

use crate::core_::{point::point_type::PointType, types::fn_in_out_ref::FnInOutRef};

use super::fn_::{FnIn, FnOut, FnInOut};

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
    pub id: String,
    input: FnInOutRef,
    result: Option<PointType>,
}
///
/// 
impl FnVar {
    pub fn new(id: impl Into<String>, input: FnInOutRef) -> Self {
        Self {
            id: id.into(), 
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
    /// Returns nothing, 
    /// - Evaluetes all calculations,
    /// - Result stores into inner
    /// - calculated result returns in .out() method
    fn eval(&mut self) {
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

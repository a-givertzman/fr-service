#![allow(non_snake_case)]

use log::trace;
use std::fmt::Debug;

use crate::core_::point::point::PointType;

use super::fn_::{FnIn, FnOut, FnInOut};

///
/// 
#[derive(Debug, Clone)]
pub struct FnInput {
    pub id: String,
    pub point: PointType,
}
///
/// 
impl FnIn for FnInput {
    fn add(&mut self, point: PointType) {
        self.point = point;
        trace!("FnInput({}).add | value: {:?}", self.id, &self.point);
    }
}
///
/// 
impl FnOut for FnInput {
    fn out(&self) -> PointType {
        trace!("FnInput({}).out | value: {:?}", self.id, &self.point);
        self.point.clone()
    }
}
///
/// 
// impl<T: FnInOut> FnInOut for T {}
impl FnInOut for FnInput {}

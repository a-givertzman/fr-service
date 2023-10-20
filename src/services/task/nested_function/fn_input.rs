#![allow(non_snake_case)]

use log::trace;
use std::fmt::Debug;

use crate::core_::point::point_type::PointType;

use super::fn_::{FnIn, FnOut, FnInOut};

///
/// 
#[derive(Debug, Clone)]
pub struct FnInput {
    pub id: String,
    point: PointType,
    initial: PointType,
}
///
/// 
impl FnInput {
    pub fn new(id: &str, initial: PointType) -> Self {
        Self {
            id: id.into(), 
            point: initial.clone(), 
            initial
        }
    }
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
    //
    fn out(&mut self) -> PointType {
        trace!("FnInput({}).out | value: {:?}", self.id, &self.point);
        self.point.clone()
    }
    //
    fn reset(&mut self) {
        self.point = self.initial.clone();
    }
}
///
/// 
impl FnInOut for FnInput {}

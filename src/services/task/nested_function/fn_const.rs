#![allow(non_snake_case)]

use log::trace;
use std::fmt::Debug;

use crate::core_::point::point_type::PointType;

use super::fn_::{FnIn, FnOut, FnInOut};

///
/// 
#[derive(Debug, Clone)]
pub struct FnConst {
    pub id: String,
    point: PointType,
}
///
/// 
impl FnConst {
    pub fn new(id: &str, value: PointType) -> Self {
        Self {
            id: id.into(), 
            point: value
        }
    }
}
///
/// 
impl FnIn for FnConst {}
///
/// 
impl FnOut for FnConst {
    //
    fn out(&mut self) -> PointType {
        trace!("FnConst({}).out | value: {:?}", self.id, &self.point);
        self.point.clone()
    }
    //
    fn reset(&mut self) {}
}
///
/// 
impl FnInOut for FnConst {}

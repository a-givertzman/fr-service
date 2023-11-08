#![allow(non_snake_case)]

use log::trace;
use std::fmt::Debug;

use crate::core_::point::point_type::PointType;

use super::{fn_::{FnIn, FnOut, FnInOut}, fn_kind::FnKind};

///
/// 
#[derive(Debug, Clone)]
pub struct FnInput {
    id: String,
    kind: FnKind,
    point: PointType,
    initial: PointType,
}
///
/// 
impl FnInput {
    pub fn new(id: &str, initial: PointType) -> Self {
        Self {
            id: id.into(), 
            kind: FnKind::Input,
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
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    fn kind(&self) -> &FnKind {
        &self.kind
    }
    //
    fn inputs(&self) -> Vec<String> {
        vec![self.point.name()]
    }
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

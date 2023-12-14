#![allow(non_snake_case)]

use log::trace;

use crate::core_::point::point_type::PointType;

use super::{fn_::{FnIn, FnOut, FnInOut}, fn_kind::FnKind};

///
/// 
#[derive(Debug, Clone)]
pub struct FnConst {
    id: String,
    txId: usize,
    kind: FnKind,
    point: PointType,
}
///
/// 
impl FnConst {
    pub fn new(id: &str, txId: usize, value: PointType) -> Self {
        Self {
            id: id.into(), 
            txId,
            kind: FnKind::Input,
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
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    fn kind(&self) -> &FnKind {
        &self.kind
    }
    //
    fn inputs(&self) -> Vec<String> {
        vec![]
    }
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

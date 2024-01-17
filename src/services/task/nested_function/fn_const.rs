#![allow(non_snake_case)]

use std::sync::atomic::{Ordering, AtomicUsize};

use log::trace;

use crate::core_::point::point_type::PointType;

use super::{fn_::{FnIn, FnOut, FnInOut}, fn_kind::FnKind};

///
/// 
#[derive(Debug, Clone)]
pub struct FnConst {
    id: String,
    kind: FnKind,
    point: PointType,
}
///
/// 
impl FnConst {
    pub fn new(parent: &str, value: PointType) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            id: format!("{}/FnConst{}", parent, COUNT.load(Ordering::Relaxed)),
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
        trace!("{}.out | value: {:?}", self.id, &self.point);
        self.point.clone()
    }
    //
    fn reset(&mut self) {}
}
///
/// 
impl FnInOut for FnConst {}
///
/// 
static COUNT: AtomicUsize = AtomicUsize::new(0);

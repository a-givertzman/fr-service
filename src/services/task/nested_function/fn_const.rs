use std::sync::atomic::{Ordering, AtomicUsize};
use log::trace;
use crate::core_::point::point_type::PointType;
use super::{fn_::{FnIn, FnOut, FnInOut}, fn_kind::FnKind};
///
/// Function | Constant value
#[derive(Debug, Clone)]
pub struct FnConst {
    id: String,
    kind: FnKind,
    point: PointType,
}
///
/// 
impl FnConst {
    ///
    /// Creates new instance of function [Const] value
    ///     - [parent] - name of the parent object
    ///     - [value] - PointType, contains point with constant value
    pub fn new(parent: &str, value: PointType) -> Self {
        Self {
            id: format!("{}/FnConst{}", parent, COUNT.fetch_add(1, Ordering::Relaxed)),
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
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

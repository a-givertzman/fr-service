#![allow(non_snake_case)]

use log::trace;
use std::{fmt::Debug, sync::atomic::{AtomicUsize, Ordering}};

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
    pub fn new(parent: &str, initial: PointType) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        Self {
            id: format!("{}/FnInput{}", parent, COUNT.load(Ordering::Relaxed)),
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
        trace!("{}.add | value: {:?}", self.id, &self.point);
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
        trace!("{}.out | value: {:?}", self.id, &self.point);
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
///
/// 
static COUNT: AtomicUsize = AtomicUsize::new(0);

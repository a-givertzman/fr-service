use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use log::debug;
use std::{hash::BuildHasherDefault, sync::atomic::{AtomicUsize, Ordering}};
use concat_string::concat_string;
use crate::{
    conf::point_config::point_config::PointConfig, 
    core_::{point::{point::Point, point_type::PointType}, types::fn_in_out_ref::FnInOutRef}, 
    services::task::nested_function::{
        fn_::{FnIn, FnInOut, FnOut},
        fn_kind::FnKind,
    },
};
///
/// Function returns the ID of the point from input
/// 
/// Example
/// 
/// ```yaml
/// fn PointId:
///     input: point int /App/PointName
/// ```
#[derive(Debug)]
pub struct FnPointId {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    points: IndexMap<String, usize, BuildHasherDefault<FxHasher>>,
}
//
// 
impl FnPointId {
    ///
    /// Creates new instance of the FnPointId
    // #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, input: FnInOutRef, points: Vec<PointConfig>) -> Self {
        Self { 
            id: format!("{}/FnPointId{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
            points: points.into_iter().map(|p| {(p.name, p.id)}).collect(),
        }
    }    
}
//
// 
impl FnIn for FnPointId {}
//
// 
impl FnOut for FnPointId { 
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
    //
    //
    fn out(&mut self) -> PointType {
        let point = self.input.borrow_mut().out();
        debug!("{}.out | points: {:#?}", self.id, self.points);
        debug!("{}.out | input: {:?}", self.id, point);
        match self.points.get(&point.name()) {
            Some(id) => {
                debug!("{}.out | ID: {:?}", self.id, id);
                PointType::Int(
                    Point::new(
                        point.tx_id(),
                        &concat_string!(self.id, ".out"),
                        *id as i64,
                        point.status(),
                        point.cot(),
                        point.timestamp(),
                    )
                )
            }
            None => panic!("{}.out | point '{}' - not found in configured points", self.id, point.name()),
        }
    }
    //
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnPointId {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

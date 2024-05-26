use std::sync::atomic::{AtomicUsize, Ordering};
use log::debug;
use crate::core_::{
    point::{point::Point, point_type::PointType},
    types::fn_in_out_ref::FnInOutRef,
};
use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};
///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnAverage {
    id: String,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    input: FnInOutRef,
    count: i64,
    sum: f64,
}
//
// 
impl FnAverage {
    ///
    /// Creates new instance of the FnAverage
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, enable: Option<FnInOutRef>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnAverage{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind:FnKind::Fn,
            enable,
            input,
            count: 0,
            sum: 0.0,
        }
    }
}
//
// 
impl FnIn for FnAverage {}
//
// 
impl FnOut for FnAverage {
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
        let mut inputs = vec![];
        if let Some(enable) = &self.enable {
            inputs.append(&mut enable.borrow().inputs());
        }
        inputs.append(&mut self.input.borrow().inputs());
        inputs
    }
    ///
    fn out(&mut self) -> PointType {
        let enable = match &mut self.enable {
            Some(en) => en.borrow_mut().out().to_bool().as_bool().value.0,
            None => true,
        };
        // trace!("{}.out | enable: {:?}", self.id, enable);
        let input = self.input.borrow_mut().out();
        // trace!("{}.out | input: {:?}", self.id, input);
        if enable {
            let value = input.to_double().as_double().value;
            self.sum += value;
            self.count += 1;
        } else {
            self.count = 0;
            self.sum = 0.0;
        }
        let average = if self.count != 0 {
            self.sum / (self.count as f64)
        } else {
            self.sum
        };
        debug!("{}.out | sum: {:?}", self.id, self.sum);
        debug!("{}.out | count: {:?}", self.id, self.count);
        debug!("{}.out | average: {:?}", self.id, average);
        PointType::Double(
            Point {
                tx_id: *input.tx_id(),
                name: self.id.clone(),
                value: average,
                status: input.status(),
                cot: input.cot(),
                timestamp: input.timestamp(),
            }
        )
    }
    fn reset(&mut self) {
        self.count = 0;
        self.sum = 0.0;
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnAverage {}
///
/// Global static counter of FnAverage instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

use std::sync::atomic::{AtomicUsize, Ordering};
use log::debug;
use crate::core_::{
    point::{point::Point, point_type::PointType},
    types::fn_in_out_ref::FnInOutRef,
};
use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};
///
/// Returns an max value (in Double) of the input
#[derive(Debug)]
pub struct FnMax {
    id: String,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    input: FnInOutRef,
    max: f64,
}
//
// 
impl FnMax {
    ///
    /// Creates new instance of the FnMax
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, enable: Option<FnInOutRef>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnMax{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind:FnKind::Fn,
            enable,
            input,
            max: 0.0,
        }
    }
}
//
// 
impl FnIn for FnMax {}
//
// 
impl FnOut for FnMax {
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
    //
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
            if value > self.max {
                self.max = value;
            }
        } else {
            self.max = 0.0;
        }
        debug!("{}.out | max: {:?}", self.id, self.max);
        PointType::Double(
            Point::new(
                input.tx_id(),
                &self.id,
                self.max,
                input.status(),
                input.cot(),
                input.timestamp(),
            )
        )
    }
    //
    fn reset(&mut self) {
        self.max = 0.0;
        if let Some(enable) = &self.enable {
            enable.borrow_mut().reset();
        }
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnMax {}
///
/// Global static counter of FnMax instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

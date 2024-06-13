use std::sync::atomic::{AtomicUsize, Ordering};
use log::trace;
use crate::{
    core_::{point::{point_tx_id::PointTxId, point_type::PointType}, types::fn_in_out_ref::FnInOutRef},
    services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind},
};
///
/// Function | Returns filtered input or default value
/// - [pass] if true (or [pass] > 0) - current input value will returns from now on
/// - if default is not specified and filtered value not passed yet - default value of the input type returns
#[derive(Debug)]
pub struct FnFilter {
    id: String,
    tx_id: usize,
    kind: FnKind,
    default: Option<FnInOutRef>,
    input: FnInOutRef,
    pass: FnInOutRef,
    state: Option<PointType>,
}
//
//
impl FnFilter {
    ///
    /// Creates new instance of the FnFilter
    /// - id - just for proper debugging
    /// - input - incoming points
    pub fn new(parent: impl Into<String>, default: Option<FnInOutRef>, input: FnInOutRef, pass: FnInOutRef) -> Self {
        let self_id = format!("{}/FnFilter{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed));
        Self {
            id: self_id.clone(),
            tx_id: PointTxId::fromStr(&self_id),
            kind: FnKind::Fn,
            default,
            input,
            pass,
            state: None,
        }
    }
    ///
    /// 
    fn default(&mut self) -> PointType {
        match self.input.borrow_mut().out() {
            PointType::Bool(_) => PointType::new(self.tx_id, &self.id, false),
            PointType::Int(_) => PointType::new(self.tx_id, &self.id, 0),
            PointType::Real(_) => PointType::new(self.tx_id, &self.id, 0.0f32),
            PointType::Double(_) => PointType::new(self.tx_id, &self.id, 0.0f64),
            PointType::String(_) => PointType::new(self.tx_id, &self.id, ""),
        }
    }
}
//
//
impl FnIn for FnFilter {}
//
//
impl FnOut for FnFilter {
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
        inputs.append(&mut self.pass.borrow().inputs());
        inputs.append(&mut self.input.borrow().inputs());
        if let Some(default) = &self.default {
            inputs.append(&mut default.borrow().inputs());
        }
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let input = self.input.borrow_mut().out();
        let pass_point = self.pass.borrow_mut().out();
        let pass = pass_point.to_bool().as_bool().value.0;
        trace!("{}.out | pass: {:?}", self.id, pass);
        if pass {
            trace!("{}.out | Passed input: {:?}", self.id, input);
            self.state = Some(input.clone());
            input
        } else {
            match &self.state {
                Some(state) => {
                    trace!("{}.out | Passed prev state: {:?}", self.id, state);
                    state.to_owned()
                }
                None => {
                    match &self.default {
                        Some(default) => {
                            let default = default.borrow_mut().out();
                            self.state = Some(default.clone());
                            trace!("{}.out | Passed default input: {:?}", self.id, default);
                            default
                        }
                        None => {
                            let default = self.default();
                            self.state = Some(default.clone());
                            trace!("{}.out | Passed default: {:?}", self.id, default);
                            default
                        }
                    }
                }
            }
        }
    }
    //
    fn reset(&mut self) {
        if let Some(default) = &self.default {
            default.borrow_mut().reset();
        }
        self.input.borrow_mut().reset();
        self.pass.borrow_mut().reset();
    }
}
//
//
impl FnInOut for FnFilter {}
///
/// Global static counter of FnFilter instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

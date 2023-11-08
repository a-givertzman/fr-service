#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};

use log::trace;

use crate::core_::{
    types::{type_of::DebugTypeOf, fn_in_out_ref::FnInOutRef},
    state::switch_state::{SwitchState, Switch, SwitchCondition}, 
    point::{point_type::PointType, point::Point}, 
};

use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};


///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnPulseCount {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    state: SwitchState<bool, bool>,
    count: i64,
    initial: i64,
}
static COUNT: AtomicUsize = AtomicUsize::new(0);
///
/// 
impl FnPulseCount {
    ///
    /// Creates new instance of the FnPulseCount
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>, initial: i64, input: FnInOutRef) -> Self {
        COUNT.fetch_add(1, Ordering::SeqCst);
        let id = "FnPulseCount";
        Self { 
            id: format!("{}{}", id, COUNT.load(Ordering::Relaxed)),
            kind:FnKind::Fn,
            input,
            state: SwitchState::new(
                false, 
                vec![
                    Switch {
                        state: false,
                        conditions: vec![SwitchCondition {
                            condition: Box::new(|value| {value}),
                            target: true,
                        }],
                    },
                    Switch {
                        state: true,
                        conditions: vec![SwitchCondition {
                            condition: Box::new(|_| {true}),
                            target: false,
                        }],
                    },
                ]
            ),
            count: initial.clone(),
            initial: initial,
        }
    }
}
///
/// 
impl FnIn for FnPulseCount {}
///
/// 
impl FnOut for FnPulseCount {
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
    ///
    fn out(&mut self) -> PointType {
        // trace!("FnPulseCount.out | input: {:?}", self.input.print());
        let point = self.input.borrow_mut().out();
        let value = match &point {
            PointType::Bool(point) => point.value.0,
            PointType::Int(point) => point.value > 0,
            PointType::Float(point) => point.value > 0.0,
            _ => panic!("FnPulseCount.out | {:?} type is not supported: {:?}", point.typeOf(), point),
        };
        self.state.add(value);
        let state = self.state.state();
        trace!("FnPulseCount.out | input.out: {:?}   | state: {:?}", &value, state);
        if state {
            self.count += 1;
        }
        PointType::Int(
            Point {
                name: String::from(format!("{}.out", self.id)),
                value: self.count,
                status: point.status(),
                timestamp: point.timestamp(),
            }
        )
    }
    fn reset(&mut self) {
        self.count = self.initial;
        self.state.reset();
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnPulseCount {}

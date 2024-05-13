use log::{debug, trace};
use std::{time::Instant, sync::atomic::{AtomicUsize, Ordering}};
use crate::{
    core_::{
        cot::cot::Cot, point::{point::Point, point_type::PointType}, state::switch_state::{Switch, SwitchCondition, SwitchState}, types::{fn_in_out_ref::FnInOutRef, type_of::DebugTypeOf},
    },
    services::task::nested_function::{
        fn_::{FnInOut, FnIn, FnOut}, fn_kind::FnKind
    },
};

use super::fn_::FnResult;
///
///
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
// #[allow(dead_code)]
enum FnTimerState {
    Off,
    Start,
    Progress,
    Stop,
    Done,
}
///
/// Counts elapsed time from raised input (>0) to dropped (<=0)
/// - if repeat = true, then elapsed is total secods of 
/// multiple periods
#[derive(Debug)]
pub struct FnTimer {
    id: String,
    kind: FnKind,
    input: FnInOutRef,
    state: SwitchState<FnTimerState, bool>,
    session_elapsed: f64,
    initial: f64,
    total_elapsed: f64,
    start: Option<Instant>,
}
///
/// 
impl FnTimer {
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, initial: impl Into<f64> + Clone, input: FnInOutRef, repeat: bool) -> Self {
        let switches = vec![
            Switch{
                state: FnTimerState::Off,
                conditions: vec![
                    SwitchCondition {
                        condition: Box::new(|value| {value}),
                        target: FnTimerState::Start,
                    },
                ],
            },
            Switch{
                state: FnTimerState::Start,
                conditions: vec![
                    SwitchCondition {
                        condition: Box::new(|value| {value}),
                        target: FnTimerState::Progress,
                    },
                    SwitchCondition {
                        condition: Box::new(|value| {!value}),
                        target: FnTimerState::Stop,
                    },
                ],
            },
            Switch{
                state: FnTimerState::Progress,
                conditions: vec![
                    SwitchCondition {
                        condition: Box::new(|value| {!value}),
                        target: FnTimerState::Stop,
                    },
                ],
            },
            Switch{
                state: FnTimerState::Stop,
                conditions: vec![
                    SwitchCondition {
                        condition: Box::new(|value| {value}),
                        target: FnTimerState::Start,
                    },
                    SwitchCondition {
                        condition: Box::new(|value| {!value}),
                        target: if repeat {FnTimerState::Off} else {FnTimerState::Done},
                    },
                ],
            },
            Switch{
                state: FnTimerState::Done,
                conditions: vec![],
            },
        ];
        Self { 
            id: format!("{}/FnTimer{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
            state: SwitchState::new(FnTimerState::Off, switches),
            session_elapsed: 0.0,
            initial: initial.clone().into(),
            total_elapsed: initial.into(),
            start: None,
        }
    }
}
///
/// 
impl FnIn for FnTimer {}
///
///
impl FnOut for FnTimer {
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
    fn out(&mut self) -> FnResult {
        // trace!("{}.out | input: {:?}", self.id, self.input.print());
        let input = self.input.borrow_mut().out();
        trace!("{}.out | input: {:?}", self.id, input);
        match input {
            FnResult::Ok(point) => {
                let value = match &point {
                    PointType::Bool(point) => point.value.0,
                    PointType::Int(point) => point.value > 0,
                    PointType::Double(point) => point.value > 0.0,
                    _ => panic!("{}.out | {:?} type is not supported: {:?}", self.id, point.print_type_of(), point),
                };
                self.state.add(value);
                let state = self.state.state();
                debug!("{}.out | input.out: {:?}   |   state: {:?}", self.id, &value, &state);
                match state {
                    FnTimerState::Off => {}
                    FnTimerState::Start => {
                        self.start = Some(Instant::now());
                    }
                    FnTimerState::Progress => {
                        self.session_elapsed = self.start.unwrap().elapsed().as_secs_f64();
                    }
                    FnTimerState::Stop => {
                        self.session_elapsed = 0.0;
                        self.total_elapsed += self.start.unwrap().elapsed().as_secs_f64();
                        self.start = None;
                    }
                    FnTimerState::Done => {
                        self.session_elapsed = 0.0;
                        if let Some(start) = self.start {
                            self.total_elapsed += start.elapsed().as_secs_f64();
                            self.start = None;
                        }
                    }
                };
                FnResult::Ok(PointType::Double(
                    Point {
                        tx_id: *point.tx_id(),
                        name: format!("{}.out", self.id),
                        value: self.total_elapsed + self.session_elapsed,
                        status: point.status(),
                        cot: Cot::Inf,
                        timestamp: point.timestamp(),
                    }
                ))
            }
            FnResult::Err(err) => FnResult::Err(err),
            FnResult::None => FnResult::None,
        }
    }
    ///
    /// 
    fn reset(&mut self) {
        self.start = None;
        self.session_elapsed = 0.0;
        self.total_elapsed = self.initial;
        self.state.reset();
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnTimer {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

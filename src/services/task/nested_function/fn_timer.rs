#![allow(non_snake_case)]

use log::debug;
use std::{time::Instant, sync::atomic::{AtomicUsize, Ordering}};

use crate::core_::{
    cot::cot::Cot, point::{point::Point, point_type::PointType}, state::switch_state::{Switch, SwitchCondition, SwitchState}, types::{fn_in_out_ref::FnInOutRef, type_of::DebugTypeOf} 
};

use super::{fn_::{FnInOut, FnIn, FnOut}, fn_kind::FnKind};


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
    sessionElapsed: f64,
    initial: f64,
    totalElapsed: f64,
    start: Option<Instant>,
}
///
/// 
impl FnTimer {
    #[allow(dead_code)]
    pub fn new(parent: &str, initial: impl Into<f64> + Clone, input: FnInOutRef, repeat: bool) -> Self {
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
        COUNT.fetch_add(1, Ordering::SeqCst);
        Self { 
            id: format!("{}/FnTimer{}", parent, COUNT.load(Ordering::Relaxed)),
            kind: FnKind::Fn,
            input,
            state: SwitchState::new(FnTimerState::Off, switches),
            sessionElapsed: 0.0,
            initial: initial.clone().into(),
            totalElapsed: initial.into(),
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
    fn out(&mut self) -> PointType {
        // trace!("{}.out | input: {:?}", self.id, self.input.print());
        let point = self.input.borrow_mut().out();
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
            FnTimerState::Off => {},
            FnTimerState::Start => {
                self.start = Some(Instant::now());
            },
            FnTimerState::Progress => {
                self.sessionElapsed = self.start.unwrap().elapsed().as_secs_f64();
            },
            FnTimerState::Stop => {
                self.sessionElapsed = 0.0;
                self.totalElapsed += self.start.unwrap().elapsed().as_secs_f64();
                self.start = None;
            },
            FnTimerState::Done => {
                self.sessionElapsed = 0.0;
                if let Some(start) = self.start {
                    self.totalElapsed += start.elapsed().as_secs_f64();
                    self.start = None;
                }
            },
        };
        PointType::Double(
            Point {
                tx_id: *point.tx_id(),
                name: format!("{}.out", self.id),
                value: self.totalElapsed + self.sessionElapsed,
                status: point.status(),
                cot: Cot::Inf,
                timestamp: point.timestamp(),
            }
        )
    }
    ///
    /// 
    fn reset(&mut self) {
        self.start = None;
        self.sessionElapsed = 0.0;
        self.totalElapsed = self.initial;
        self.state.reset();
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnTimer {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(0);

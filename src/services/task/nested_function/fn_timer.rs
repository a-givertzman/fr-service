use log::trace;
use std::{sync::atomic::{AtomicUsize, Ordering}, time::Instant};
use crate::{conf::point_config::point_config_type::PointConfigType, core_::{
    cot::cot::Cot, point::{point::Point, point_type::PointType},
    state::switch_state::{Switch, SwitchCondition, SwitchState},
    types::fn_in_out_ref::FnInOutRef,
}};
use super::{fn_::{FnInOut, FnIn, FnOut}, fn_kind::FnKind};
//
//
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
/// Returns elapsed time in seconds (double) from raised input (>0) to dropped (<=0)
/// - if repeat = true, then elapsed is total secods of 
/// multiple periods
#[derive(Debug)]
pub struct FnTimer {
    id: String,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    initial: Option<FnInOutRef>,
    input: FnInOutRef,
    state: SwitchState<FnTimerState, bool>,
    session_elapsed: f64,
    total_elapsed: Option<f64>,
    start: Option<Instant>,
}
//
// 
impl FnTimer {
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, enable: Option<FnInOutRef>, initial: Option<FnInOutRef>, input: FnInOutRef, repeat: bool) -> Self {
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
            enable,
            input,
            initial,
            state: SwitchState::new(FnTimerState::Off, switches),
            session_elapsed: 0.0,
            total_elapsed: None,
            start: None,
        }
    }
    ///
    /// Returns initial value
    fn initial(&mut self) -> f64 {
        self.initial.as_mut().map_or(0.0, |initial| {
            initial.borrow_mut().out().to_double().as_double().value
        })
    }
}
//
//
impl FnIn for FnTimer {}
//
//
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
        let mut inputs = vec![];
        if let Some(enable) = &self.enable {
            inputs.append(&mut enable.borrow().inputs());
        }
        if let Some(initial) = &self.initial {
            inputs.append(& mut initial.borrow().inputs());
        }
        inputs.append(& mut self.input.borrow().inputs());
        inputs
    }
    ///
    fn out(&mut self) -> FnResult<PointType, String> {
        // trace!("{}.out | input: {:?}", self.id, self.input.print());
        let enable = match &mut self.enable {
            Some(en) => en.borrow_mut().out().to_bool().as_bool().value.0,
            None => true,
        };
        let input = self.input.borrow_mut().out();
        let out = if enable {
            let total_elapsed = match &mut self.total_elapsed {
                Some(total_elapsed) => total_elapsed,
                None => {
                    let initial = self.initial();
                    self.total_elapsed = Some(initial);
                    self.total_elapsed.as_mut().unwrap()
                },
            };
            let value = input.to_bool().as_bool().value.0;
            self.state.add(value);
            let state = self.state.state();
            trace!("{}.out | input: {:?}   |   state: {:?}", self.id, value, state);
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
                    *total_elapsed += self.start.unwrap().elapsed().as_secs_f64();
                    self.start = None;
                }
                FnTimerState::Done => {
                    self.session_elapsed = 0.0;
                    if let Some(start) = self.start {
                        *total_elapsed += start.elapsed().as_secs_f64();
                        self.start = None;
                    }
                }
            };
            *total_elapsed + self.session_elapsed
        } else {
            self.start = None;
            self.session_elapsed = 0.0;
            let initial = self.initial();
            self.total_elapsed = Some(initial);
            self.state.reset();
            initial
        };
        trace!("{}.out | out: {:?}", self.id, out);
        match &self.initial {
            Some(initial) => {
                let type_ = initial.borrow_mut().out().type_();
                match type_ {
                    PointConfigType::Int => PointType::Int(
                        Point::new(
                            input.tx_id(),
                            &format!("{}.out", self.id),
                            out.round() as i64,
                            input.status(),
                            Cot::Inf,
                            input.timestamp(),
                        )
                    ),
                    PointConfigType::Real => PointType::Real(
                        Point::new(
                            input.tx_id(),
                            &format!("{}.out", self.id),
                            out as f32,
                            input.status(),
                            Cot::Inf,
                            input.timestamp(),
                        )
                    ),
                    PointConfigType::Double => PointType::Double(
                        Point::new(
                            input.tx_id(),
                            &format!("{}.out", self.id),
                            out,
                            input.status(),
                            Cot::Inf,
                            input.timestamp(),
                        )
                    ),
                    _ => panic!("{}.out | Usupported initial type '{:?}'", self.id, type_),
                }
            }
            None => PointType::Double(
                Point::new(
                    input.tx_id(),
                    &format!("{}.out", self.id),
                    out,
                    input.status(),
                    Cot::Inf,
                    input.timestamp(),
                )
            ),
        }
    }
    //
    //
    fn reset(&mut self) {
        self.start = None;
        self.session_elapsed = 0.0;
        self.total_elapsed = Some(self.initial.as_mut().map_or(0.0, |initial| {
            initial.borrow_mut().reset();
            initial.borrow_mut().out().to_double().as_double().value
        }));
        self.state.reset();
        if let Some(enable) = &self.enable {
            enable.borrow_mut().reset();
        }
        self.input.borrow_mut().reset();
    }
}
//
// 
impl FnInOut for FnTimer {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

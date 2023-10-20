#![allow(non_snake_case)]

use log::debug;
use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::core_::{state::switch_state::{SwitchState, Switch, SwitchCondition}, point::{point_type::PointType, point::Point}, types::type_of::DebugTypeOf};

use super::fn_::{FnInOut, FnIn, FnOut};


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    input: Rc<RefCell<Box<dyn FnInOut>>>,
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
    pub fn new(id: &str, initial: impl Into<f64> + Clone, input: Rc<RefCell<Box<dyn FnInOut>>>, repeat: bool) -> Self {
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
            id: id.into(),
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
impl FnIn for FnTimer {
    fn add(&mut self, _: PointType) {
        panic!("FnTimer.add | method is not used")
    }
}
///
///
impl FnOut for FnTimer {
    ///
    fn out(&mut self) -> PointType {
        // trace!("FnTimer.out | input: {:?}", self.input.print());
        let point = self.input.borrow_mut().out();
        let value = match &point {
            PointType::Bool(point) => point.value.0,
            PointType::Int(point) => point.value > 0,
            PointType::Float(point) => point.value > 0.0,
            _ => panic!("FnCount.out | {:?} type is not supported: {:?}", point.typeOf(), point),
        };
        self.state.add(value);
        let state = self.state.state();
        debug!("FnTimer.out | input.out: {:?}   |   state: {:?}", &value, &state);
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
                match self.start {
                    Some(start) => {
                        self.totalElapsed += start.elapsed().as_secs_f64();
                        self.start = None;
                    },
                    None => {},
                }
            },
        };
        PointType::Float(
            Point {
                name: String::from("FnTimer"),
                value: self.totalElapsed + self.sessionElapsed,
                status: point.status(),
                timestamp: point.timestamp(),
            }
        )
    }
    ///
    /// 
    fn reset(&mut self) {
        self.start = None;
        self.sessionElapsed = 0.0;
        self.totalElapsed = self.initial.into();
        self.state.reset();
        self.input.borrow_mut().reset();
    }
}
///
/// 
impl FnInOut for FnTimer {}
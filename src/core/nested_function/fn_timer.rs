#![allow(non_snake_case)]

use log::debug;
use std::{cell::RefCell, rc::Rc, time::Instant};

use crate::core::state::switch_state::{SwitchState, Switch, SwitchCondition};

use super::fn_::FnOutput;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[allow(dead_code)]
enum FnTimerState {
    Off,
    Start,
    Progress,
    Stop,
    Done,
}
///
/// Counts elapsed time from raised onput to dropped
/// - if repeat = true, then elapsed is total secods of 
/// multiple periods
pub struct FnTimer {
    input: Rc<RefCell<dyn FnOutput<bool>>>,
    state: SwitchState<FnTimerState, bool>,
    sessionElapsed: f64,
    totalElapsed: f64,
    start: Option<Instant>,
}

impl FnTimer {
    #[allow(dead_code)]
    pub fn new(initial: impl Into<f64>, input: Rc<RefCell<dyn FnOutput<bool>>>, repeat: bool) -> Self {
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
                        target: FnTimerState::Start,
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
                    // SwitchCondition {
                    //     condition: Box::new(|value| {value}),
                    //     target: FnTimerState::Progress,
                    // },
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
            input,
            state: SwitchState::new(FnTimerState::Off, switches),
            sessionElapsed: 0.0,
            totalElapsed: initial.into(),
            start: None,
        }
    }
}

impl FnOutput<f64> for FnTimer {
    ///
    fn out(&mut self) -> f64 {
        // debug!("FnTimer.out | input: {:?}", self.input.print());
        let value = self.input.borrow_mut().out();
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
                self.totalElapsed += self.start.unwrap().elapsed().as_secs_f64();
                self.start = None;
            },
            FnTimerState::Done => {
                match self.start {
                    Some(start) => {
                        self.totalElapsed = start.elapsed().as_secs_f64();
                        self.start = None;
                    },
                    None => {},
                }
            },
        };
        self.totalElapsed + self.sessionElapsed
    }
}

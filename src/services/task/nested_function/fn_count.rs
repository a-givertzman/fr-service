#![allow(non_snake_case)]

use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::core_::{state::switch_state::{SwitchState, Switch, SwitchCondition}, point::{point_type::PointType, point::Point}, types::type_of::DebugTypeOf};

use super::fn_::{FnInOut, FnOut, FnIn};


///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct FnCount {
    id: String,
    input: Rc<RefCell<Box<dyn FnInOut>>>,
    state: SwitchState<bool, bool>,
    count: i64,
    initial: i64,
}
///
/// 
impl FnCount {
    ///
    /// Creates new instance of the FnCount
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>, initial: i64, input: Rc<RefCell<Box<dyn FnInOut>>>) -> Self {
        Self { 
            id: id.into(),
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
impl FnIn for FnCount {}
///
/// 
impl FnOut for FnCount {
    ///
    fn out(&mut self) -> PointType {
        // trace!("FnCount.out | input: {:?}", self.input.print());
        let point = self.input.borrow_mut().out();
        let value = match &point {
            PointType::Bool(point) => point.value.0,
            PointType::Int(point) => point.value > 0,
            PointType::Float(point) => point.value > 0.0,
            _ => panic!("FnCount.out | {:?} type is not supported: {:?}", point.typeOf(), point),
        };
        self.state.add(value);
        let state = self.state.state();
        trace!("FnCount.out | input.out: {:?}   | state: {:?}", &value, state);
        if state {
            self.count += 1;
        }
        PointType::Int(
            Point {
                name: String::from("FnCount.out"),
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
impl FnInOut for FnCount {}

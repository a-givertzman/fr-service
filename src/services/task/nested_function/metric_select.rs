#![allow(non_snake_case)]

use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::core_::conf::metric_config::MetricConfig;

use super::{fn_::FnInOut, fn_inputs::FnInputs};


pub trait FnMetric {
    fn new(conf: MetricConfig) -> Self;
    fn out(&self) -> String;
    fn reset(&mut self);
}



///
/// Counts number of raised fronts of boolean input
// #[derive(Debug, Deserialize)]
pub struct MetricSelect {
    // _marker: PhantomData<S>,
    input: Rc<RefCell<Box<dyn FnInOut>>>,
    initial: FnInOut,
    table: String,
    sql: String,
}

impl FnMetric for MetricSelect {
    ///
    /// 
    fn new(conf: &mut MetricConfig, inputs: &mut FnInputs) -> FnMetric {
        let initial = match conf.initial {
            Initial::Bool(initial) => {
                PointType::Bool(  Point { value: Bool(initial),   name:String::from("bool"),  status: 0, timestamp: chrono::offset::Utc::now() })
            }
            Initial::Int(initial) => {
                PointType::Int(   Point { value: initial,     name:String::from("int"),   status: 0, timestamp: chrono::offset::Utc::now() })
            },
            Initial::Float(initial) => {
                PointType::Float( Point { value: initial,  name:String::from("float"), status: 0, timestamp: chrono::offset::Utc::now() })
            },
            Initial::None => panic!("Unknown type of initial"),
        };
        let func = NestedFn::new(conf, initial, inputs);
        FnMetric {
            id: conf.id.clone(),
            input: func,
        }
    }
    
    fn out(&self) -> String {
        let pointType = self.input.borrow().out();
        match pointType {
            crate::traits::app_core::point::PointType::Bool(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
            crate::traits::app_core::point::PointType::Int(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
            crate::traits::app_core::point::PointType::Float(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
        }
    }
}

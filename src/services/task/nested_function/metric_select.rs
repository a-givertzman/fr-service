#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc};

use crate::core_::{conf::metric_config::MetricConfig, point::{point_type::PointType, point::Point}};

use super::{fn_::{FnInOut, FnOut, FnIn}, fn_inputs::FnInputs, nested_fn::NestedFn};


// pub trait FnMetric {
//     ///
//     /// Creates new MetricXyz instance deppending on config
//     // fn new(conf: &mut MetricConfig, inputs: &mut FnInputs) -> Self;
//     ///
//     /// returns output string containing sql
//     fn out(&self) -> String;
//     ///
//     /// 
//     fn reset(&mut self);
// }



///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct MetricSelect {
    // _marker: PhantomData<S>,
    id: String,
    input: Rc<RefCell<Box<dyn FnInOut>>>,
    initial: f64,
    table: String,
    sql: String,
}
///
/// 
impl MetricSelect {
    //
    //
    pub fn new(conf: &mut MetricConfig, taskStuff: &mut FnInputs) -> MetricSelect {
        let (inputName, inputConf) = conf.inputs.iter_mut().next().unwrap();
        let input = NestedFn::new(inputConf, taskStuff);
        MetricSelect {
            id: conf.name.clone(),
            input: input,
            initial: conf.initial,
            table: conf.table.clone(),
            sql: conf.sql.clone(),
        }
    }
}
///
/// 
impl FnIn for MetricSelect {
    fn add(&mut self, point: PointType) {
        panic!("MetricSelect.add | method is not used")
    }
}
///
/// 
impl FnOut for MetricSelect {
    //
    //
    fn out(&mut self) -> PointType {
        let pointType = self.input.borrow_mut().out();
        match pointType {
            PointType::Bool(point) => {
                PointType::String(Point::newString(
                    "asBool", 
                    format!("insert into {} values(id, value, timestamp) ({},{},{})", self.table, self.id, point.value, point.timestamp)
                ))
            },
            PointType::Int(point) => {
                PointType::String(Point::newString(
                    "asBool", 
                    format!("insert into {} values(id, value, timestamp) ({},{},{})", self.table, self.id, point.value, point.timestamp)
                ))
            },
            PointType::Float(point) => {
                PointType::String(Point::newString(
                    "asBool", 
                    format!("insert into {} values(id, value, timestamp) ({},{},{})", self.table, self.id, point.value, point.timestamp)
                ))
            },
            PointType::String(point) => {
                PointType::String(Point::newString(
                    "asBool", 
                    format!("insert into {} values(id, value, timestamp) ({},{},{})", self.table, self.id, point.value, point.timestamp)
                ))
            },
        }
    }

    fn reset(&mut self) {
        todo!()
    }
}
///
/// 
impl FnInOut for MetricSelect {}

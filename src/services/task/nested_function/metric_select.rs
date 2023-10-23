#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc};

use crate::{core_::{conf::fn_config::FnConfig, point::{point_type::PointType, point::Point}}, services::task::task_stuff::TaskStuff};

use super::{fn_::{FnInOut, FnOut, FnIn}, nested_fn::NestedFn};


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
    pub fn new(conf: &mut FnConfig, taskStuff: &mut TaskStuff) -> MetricSelect {
        let inputConf = conf.inputConf("input");
        let input = NestedFn::new(inputConf, taskStuff);
        MetricSelect {
            id: conf.name.clone(),
            input: input,
            initial: conf.param("initial").name.parse().unwrap(),
            table: conf.param("table").name.clone(),
            sql: conf.param("sql").name.clone(),
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
                    "asInt", 
                    format!("insert into {} values(id, value, timestamp) ({},{},{})", self.table, self.id, point.value, point.timestamp)
                ))
            },
            PointType::Float(point) => {
                PointType::String(Point::newString(
                    "asFloat", 
                    format!("insert into {} values(id, value, timestamp) ({},{},{})", self.table, self.id, point.value, point.timestamp)
                ))
            },
            PointType::String(point) => {
                PointType::String(Point::newString(
                    "asString", 
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

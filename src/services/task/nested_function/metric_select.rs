#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc};

use crate::core_::{conf::metric_config::MetricConfig, point::point::PointType};

use super::{fn_::FnInOut, fn_inputs::FnInputs, nested_fn::NestedFn};


pub trait FnMetric {
    ///
    /// Creates new MetricXyz instance deppending on config
    // fn new(conf: &mut MetricConfig, inputs: &mut FnInputs) -> Self;
    ///
    /// returns output string containing sql
    fn out(&self) -> String;
    ///
    /// 
    fn reset(&mut self);
}



///
/// Counts number of raised fronts of boolean input
// #[derive(Debug)]
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
    pub fn new(conf: &mut MetricConfig, inputs: &mut FnInputs) -> MetricSelect {
        let (inputName, inputConf) = conf.inputs.iter_mut().next().unwrap();
        let func = NestedFn::new(inputConf, inputs);
        MetricSelect {
            id: conf.name,
            input: func,
            initial: conf.initial,
            table: conf.table,
            sql: conf.sql,
        }
    }
}
///
/// 
impl FnMetric for MetricSelect {
    //
    //
    // fn new(conf: &mut MetricConfig, inputs: &mut FnInputs) -> MetricSelect {
    //     let 
    //     let func = NestedFn::new(conf, inputs);
    //     MetricSelect {
    //         id: conf.name,
    //         input: func,
    //         initial: conf.initial,
    //         table: conf.table,
    //         sql: conf.sql,
    //     }
    // }
    //
    //
    fn out(&self) -> String {
        let pointType = self.input.borrow().out();
        match pointType {
            PointType::Bool(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
            PointType::Int(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
            PointType::Float(point) => {
                format!("insert into table values(id, value, timestamp) ({},{},{})", self.id, point.value, point.timestamp)
            },
        }
    }

    fn reset(&mut self) {
        todo!()
    }
}

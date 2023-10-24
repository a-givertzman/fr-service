#![allow(non_snake_case)]

use std::{cell::RefCell, rc::Rc, collections::HashMap};

use log::{debug, error, trace, warn};

use crate::{core_::{conf::fn_config::FnConfig, point::{point_type::PointType, point::Point}, format::format::Format}, services::task::task_stuff::TaskStuff};

use super::{fn_::{FnInOut, FnOut, FnIn}, nested_fn::NestedFn};


///
/// Counts number of raised fronts of boolean input
#[derive(Debug)]
pub struct MetricSelect {
    id: String,
    inputs: HashMap<String, Rc<RefCell<Box<dyn FnInOut>>>>,
    initial: f64,
    table: String,
    sql: Format,
    sqlNames: HashMap<String, (String, Option<String>)>,
}
///
/// 
impl MetricSelect {
    //
    //
    pub fn new(conf: &mut FnConfig, taskStuff: &mut TaskStuff) -> MetricSelect {
        let mut inputs = HashMap::new();
        let mut inputConfs = conf.inputs.clone();
        inputConfs.remove("initial");
        inputConfs.remove("table");
        inputConfs.remove("sql");
        for (name, fnConf) in inputConfs {
            debug!("MetricSelect.new | input name: {:?}", name);
            let inputConf = conf.inputConf(&name);
            inputs.insert(
                name, 
                NestedFn::new(inputConf, taskStuff),
            );
        }
        let id = conf.name.clone();
        let initial = conf.param("initial").name.parse().unwrap();
        let table = conf.param("table").name.clone();
        let mut sqlFormat = Format::new(&conf.param("sql").name);
        sqlFormat.insert("id", &id);
        sqlFormat.insert("table", &table);
        let mut sqlNames = sqlFormat.names();
        sqlNames.remove("initial");
        sqlNames.remove("table");
        sqlNames.remove("sql");
        sqlNames.remove("id");
        MetricSelect {
            id: id,
            inputs: inputs,
            initial: initial,
            table: table,
            sql: sqlFormat,
            sqlNames: sqlNames,
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
    fn out(&mut self) -> PointType {
        for (fullName, (name, sufix)) in &self.sqlNames {
            trace!("MetricSelect.out | name: {:?}, sufix: {:?}", &name, &sufix);
            match self.inputs.get(name) {
                Some(input) => {
                    trace!("MetricSelect.out | input: {:?} - found", &name);
                    let pointType = input.borrow_mut().out();
                    match sufix {
                        Some(sufix) => {
                            match sufix.as_str() {
                                "value" => {
                                    match pointType {
                                        PointType::Bool(point) => {
                                            self.sql.insert(&fullName, point.value);
                                        },
                                        PointType::Int(point) => {
                                            self.sql.insert(&fullName, point.value);
                                        },
                                        PointType::Float(point) => {
                                            self.sql.insert(&fullName, point.value);
                                        },
                                        PointType::String(point) => {
                                            self.sql.insert(&fullName, point.value);
                                        },
                                    };
                                },
                                "timestamp" => self.sql.insert(&fullName, pointType.timestamp()),
                                _ => panic!("MetricSelect.out | Unknown input sufix in: {:?}, allowed: .value or .timestamp", &name),
                            }
                        },
                        None => {
                            panic!("MetricSelect.out | name: {:?}, sufix: None", &name);
                        },
                    }
                },
                None => {
                    panic!("MetricSelect.out | input: {:?} - not found", &name);
                },
            };
        }
        debug!("MetricSelect.out | sql: {:?}", self.sql.out());
        PointType::String(Point::newString(
            "MetricSelect.out", 
            self.sql.out(),
        ))
    }
    //
    fn reset(&mut self) {
        todo!()
    }
}
///
/// 
impl FnInOut for MetricSelect {}

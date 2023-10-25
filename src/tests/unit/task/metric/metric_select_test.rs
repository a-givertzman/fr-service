#![allow(non_snake_case)]
use log::{warn, trace};
#[cfg(test)]
use log::{debug, info};
use regex::RegexBuilder;
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel}, 
    point::{point_type::{PointType, ToPoint}, point::Point}, conf::fn_config::FnConfig}, 
    services::task::{nested_function::{fn_::{FnInOut, FnOut}, metric_select::MetricSelect}, task_stuff::TaskStuff},
};

// Note this useful idiom: importing names from outer (for mod tests) scope.
// use super::*;

static INIT: Once = Once::new();

///
/// once called initialisation
fn initOnce() {
    INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        }
    )
}


///
/// returns:
///  - ...
fn initEach(conf: &mut FnConfig, inputs: &mut TaskStuff) -> Rc<RefCell<Box<dyn FnInOut>>> {
    fn boxFnInput(input: MetricSelect) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    Rc::new(RefCell::new(
        boxFnInput(
            MetricSelect::new(conf, inputs)
        )
    ))
}


// #[test]
fn test_int() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_int");
    let path = "./src/tests/unit/task/metric/metric_select_int_test.yaml";
    let mut conf = FnConfig::read(path);
    debug!("conf: {:?}", conf);
    let mut taskStuff = TaskStuff::new();
    let mut fnCount = MetricSelect::new(
        &mut conf, 
        &mut taskStuff,
    );
    debug!("taskStuff: {:?}", taskStuff);
    let testData = vec![
        (1, "/path/Point.Name", 3),
        (1, "/path/Point.Name", 3),
        (1, "/path/Point.Name", 3),
        (1, "/path/Point.Name", 3),
        (0, "/path/Point.Name", 2),
        (1, "/path/Point.Name", 3),
        (2, "/path/Point.Name", 4),
        (3, "/path/Point.Name", 5),
        (4, "/path/Point.Name", 6),
        (5, "/path/Point.Name", 7),
        (6, "/path/Point.Name", 8),
        (7, "/path/Point.Name", 9),
        (8, "/path/Point.Name", 10),
        (9, "/path/Point.Name", 11),
    ];
    for (value, name, targetValue) in testData {
        let point = value.toPoint(name);
        let inputName = &point.name();
        match taskStuff.getInput(&inputName) {
            Some(input) => {
                input.borrow_mut().add(point.clone());
                // debug!("input: {:?}", &input);
                let state = fnCount.out();
                // debug!("input: {:?}", &mut input);
                debug!("value: {:?}   |   state: {:?}", point.asInt().value, state.asString().value);
                assert_eq!(
                    state.asString().value, 
                    format!("UPDATE SelectMetric_test_table_name SET kind = '{}' WHERE id = '{}';",targetValue, 1.11),
                    // format!("insert into SelectMetric_test_table_name values(id, value, timestamp) (sqlSelectMetric,{},{})", targetValue, point.timestamp())
                );
            },
            None => {
                panic!("input {:?} - not found in the current taskStuff", &inputName)
            },
        };
    }        
}


#[test]
fn test_float() {
    DebugSession::init(LogLevel::Trace);
    initOnce();
    info!("test_float");
    let path = "./src/tests/unit/task/metric/metric_select_float_test.yaml";
    let mut conf = FnConfig::read(path);
    debug!("conf: {:?}", conf);
    let mut taskStuff = TaskStuff::new();
    let mut fnCount = MetricSelect::new(
        &mut conf, 
        &mut taskStuff,
    );
    debug!("taskStuff: {:?}", taskStuff);
    let testData = vec![
        (1.1, "/path/Point.Name", 3.3),
        (1.2, "/path/Point.Name", 3.4),
        (1.3, "/path/Point.Name", 3.5),
        (1.4, "/path/Point.Name", 3.6),
        (0.1, "/path/Point.Name", 2.3),
        (1.1, "/path/Point.Name", 3.3),
        (2.2, "/path/Point.Name", 4.4),
        (3.3, "/path/Point.Name", 5.5),
        (4.4, "/path/Point.Name", 6.6),
        (5.5, "/path/Point.Name", 7.7),
        (6.6, "/path/Point.Name", 8.8),
        (7.7, "/path/Point.Name", 9.9),
        (8.8, "/path/Point.Name", 11.0),
        (9.9, "/path/Point.Name", 12.1),
    ];
    for (value, name, targetValue) in testData {
        let point = value.toPoint(name);
        let inputName = &point.name();
        match taskStuff.getInput(&inputName) {
            Some(input) => {
                input.borrow_mut().add(point.clone());
                // debug!("input: {:?}", &input);
                let state = fnCount.out();
                let out = state.asString().value;
                let re = r"(UPDATE SelectMetric_test_table_name SET kind = ')(\d+(?:\.\d+)*)(' WHERE id = '3.33';)";
                trace!("re: {}", re);
                let re = RegexBuilder::new(&re).multi_line(false).build().unwrap();
                let digits: f64 = re.captures(&out).unwrap().get(2).unwrap().as_str().parse().unwrap();
                let digits = format!("{:.1}", digits);
                trace!("digits: {:?}", digits);
                let out = re.replace(&out, "$1{!}$3");
                let out = out.replace("{!}", &digits);
                trace!("out: {}", out);
        
                debug!("value: {:?}   |   state: {:?}", point.asFloat().value, state.asString().value);
                assert_eq!(
                    out, 
                    format!("UPDATE SelectMetric_test_table_name SET kind = '{:.1}' WHERE id = '{}';",targetValue, 3.33),
                    // format!("insert into SelectMetric_test_table_name values(id, value, timestamp) (sqlSelectMetric,{:.3},{})", targetValue, point.timestamp())
                );
            },
            None => {
                panic!("input {:?} - not found in the current taskStuff", &inputName)
            },
        };
    }        
}

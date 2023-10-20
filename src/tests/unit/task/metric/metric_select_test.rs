#![allow(non_snake_case)]
use log::warn;
#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};

use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel}, 
    point::{point_type::PointType, point::Point}, conf::metric_config::MetricConfig}, 
    services::task::nested_function::{fn_::{FnInOut, FnOut}, metric_select::MetricSelect, fn_inputs::FnInputs},
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
fn initEach(conf: &mut MetricConfig, inputs: &mut FnInputs) -> Rc<RefCell<Box<dyn FnInOut>>> {
    fn boxFnInput(input: MetricSelect) -> Box<(dyn FnInOut)> {
        Box::new(input)
    }
    Rc::new(RefCell::new(
        boxFnInput(
            MetricSelect::new(conf, inputs)
        )
    ))
}

const CONF_PATH: &str = "./src/tests/unit/task/metric/metric_select_test.yaml";

#[test]
fn test_single() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    info!("test_single");
    let mut conf = MetricConfig::read(CONF_PATH);
    debug!("conf: {:?}", conf);
    let mut taskStuff = FnInputs::new();
    let mut fnCount = MetricSelect::new(
        &mut conf, 
        &mut taskStuff,
    );
    debug!("taskStuff: {:?}", taskStuff);
    let testData = vec![
        (1, "Point.Name", 3),
        (1, "Point.Name", 3),
        (1, "Point.Name", 3),
        (1, "Point.Name", 3),
        (0, "Point.Name", 2),
        (1, "Point.Name", 3),
        (2, "Point.Name", 4),
        (3, "Point.Name", 5),
        (4, "Point.Name", 6),
        (5, "Point.Name", 7),
        (6, "Point.Name", 8),
        (7, "Point.Name", 9),
        (8, "Point.Name", 10),
        (9, "Point.Name", 11),
    ];
    for (value, name, targetValue) in testData {
        let point = PointType::Int(Point::newInt(name, value));
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
                    format!("insert into SelectMetric_test_table_name values(id, value, timestamp) (sqlSelectMetric,{},{})", targetValue, point.timestamp())
                );
            },
            None => {
                warn!("input {:?} - not found in the current taskStuff", &inputName)
            },
        };
    }        
}

















































// 
// #[test]
// fn test_multiple() {
//     DebugSession::init(LogLevel::Debug);
//     initOnce();
//     info!("test_multiple");
//     let input = initEach(PointType::Bool(Point::newBool("bool", false)));
//     let mut fnCount = FnCount::new(
//         0, 
//         input.clone(),
//     );
//     let testData = vec![
//         (false, 0),
//         (false, 0),
//         (true, 1),
//         (false, 1),
//         (false, 1),
//         (true, 2),
//         (false, 2),
//         (true, 3),
//         (false, 3),
//         (false, 3),
//         (true, 4),
//         (true, 4),
//         (false, 4),
//         (false, 4),
//     ];
//     for (value, targetState) in testData {
//         let point = PointType::Bool(Point::newBool("test", value));
//         input.borrow_mut().add(point);
//         // debug!("input: {:?}", &input);
//         let state = fnCount.out();
//         // debug!("input: {:?}", &mut input);
//         debug!("value: {:?}   |   state: {:?}", value, state);
//         assert_eq!(state.asInt().value, targetState);
//     }        
// }

// #[test]
// fn test_multiple_reset() {
//     DebugSession::init(LogLevel::Debug);
//     initOnce();
//     info!("test_multiple_reset");
//     let input = initEach(PointType::Bool(Point::newBool("bool", false)));
//     let mut fnCount = FnCount::new(
//         0, 
//         input.clone(),
//     );
//     let testData = vec![
//         (false, 0, false),
//         (false, 0, false),
//         (true, 1, false),
//         (false, 1, false),
//         (false, 1, false),
//         (true, 2, false),
//         (false, 0, true),
//         (true, 1, false),
//         (false, 1, false),
//         (false, 1, false),
//         (true, 2, false),
//         (true, 2, false),
//         (false, 0, true),
//         (false, 0, false),
//     ];
//     for (value, targetState, reset) in testData {
//         if reset {
//             fnCount.reset();
//         }
//         let point = PointType::Bool(Point::newBool("test", value));
//         input.borrow_mut().add(point);
//         // debug!("input: {:?}", &input);
//         let state = fnCount.out();
//         // debug!("input: {:?}", &mut input);
//         debug!("value: {:?}   |   state: {:?}", value, state);
//         assert_eq!(state.asInt().value, targetState);
//     }        
// }

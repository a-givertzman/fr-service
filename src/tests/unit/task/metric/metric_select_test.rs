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
        (1, "Point.name1", 4),
        (1, "Point.name", 4),
        (1, "Point.name", 4),
        (1, "Point.name", 4),
        (0, "Point.name", 0),
        (1, "Point.name", 0),
        (2, "Point.name", 1),
        (3, "Point.name", 1),
        (4, "Point.name", 1),
        (5, "Point.name", 2),
        (6, "Point.name", 2),
        (7, "Point.name", 3),
        (8, "Point.name", 3),
        (9, "Point.name", 3),
    ];
    for (value, name, targetState) in testData {
        let point = PointType::Int(Point::newInt(name, value));
        let inputName = point.name();
        match taskStuff.getInput(&inputName) {
            Some(input) => {
                input.borrow_mut().add(point);
                // debug!("input: {:?}", &input);
                let state = fnCount.out();
                // debug!("input: {:?}", &mut input);
                debug!("value: {:?}   |   state: {:?}", value, state);
                // assert_eq!(state.asInt().value, targetState);
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

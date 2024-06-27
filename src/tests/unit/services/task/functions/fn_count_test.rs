#[cfg(test)]
use log::{debug, info};
use std::{sync::Once, rc::Rc, cell::RefCell};
use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
use crate::{
    conf::fn_::fn_conf_keywd::FnConfPointType, core_::{point::point_type::{PointType, ToPoint},
    types::fn_in_out_ref::FnInOutRef},
    services::task::nested_function::{
        fn_::FnOut, fn_count::{self, FnCount}, fn_input::FnInput, reset_counter::AtomicReset,
    }
};
///
///
static INIT: Once = Once::new();
///
/// once called initialisation
fn init_once() {
    INIT.call_once(|| {
        // implement your initialisation code to be called only once for current test file
    })
}
///
/// returns:
///  - ...
fn init_each(initial: PointType, type_: FnConfPointType) -> FnInOutRef {
    fn_count::COUNT.reset(0);
    Rc::new(RefCell::new(Box::new(
        FnInput::new("test", initial, type_)
    )))
}
///
///
#[test]
fn test_single() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_single");
    let initial = Some(init_each(0.to_point(0, "initial int"), FnConfPointType::Int));
    let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
    let mut fn_count = FnCount::new(
        "test",
        initial,
        input.clone(),
    );
    let test_data = vec![
        (false, 0),
        (false, 0),
        (true, 1),
        (false, 1),
        (false, 1),
        (true, 2),
        (false, 2),
        (true, 3),
        (false, 3),
        (false, 3),
        (true, 4),
        (true, 4),
        (false, 4),
        (false, 4),
    ];
    for (value, target) in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fn_count.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_int().value, target);
    }
}
//

#[test]
fn test_multiple() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_multiple");
    let initial = Some(init_each(0.to_point(0, "initial int"), FnConfPointType::Int));
    let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
    let mut fn_count = FnCount::new(
        "test",
        initial,
        input.clone(),
    );
    let test_data = vec![
        (false, 0),
        (false, 0),
        (true, 1),
        (false, 1),
        (false, 1),
        (true, 2),
        (false, 2),
        (true, 3),
        (false, 3),
        (false, 3),
        (true, 4),
        (true, 4),
        (false, 4),
        (false, 4),
    ];
    for (value, target) in test_data {
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fn_count.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_int().value, target);
    }
}

#[test]
fn test_multiple_reset() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    init_once();
    info!("test_multiple_reset");
    let initial = Some(init_each(0.to_point(0, "initial int"), FnConfPointType::Int));
    let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
    let mut fn_count = FnCount::new(
        "test",
        initial,
        input.clone(),
    );
    let test_data = vec![
        (false, 0, false),
        (false, 0, false),
        (true, 1, false),
        (false, 1, false),
        (false, 1, false),
        (true, 2, false),
        (false, 0, true),
        (true, 1, false),
        (false, 1, false),
        (false, 1, false),
        (true, 2, false),
        (true, 2, false),
        (false, 0, true),
        (false, 0, false),
    ];
    for (value, target, reset) in test_data {
        if reset {
            fn_count.reset();
        }
        let point = value.to_point(0, "test");
        input.borrow_mut().add(point);
        // debug!("input: {:?}", &input);
        let state = fn_count.out();
        // debug!("input: {:?}", &mut input);
        debug!("value: {:?}   |   state: {:?}", value, state);
        assert_eq!(state.as_int().value, target);
    }
}

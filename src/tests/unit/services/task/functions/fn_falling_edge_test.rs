#[cfg(test)]
mod fn_falling_edge {
    use log::{debug, info};
    use testing::entities::test_value::Value;
    use std::{sync::Once, rc::Rc, cell::RefCell};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use crate::{
        conf::fn_::fn_conf_keywd::FnConfPointType, 
        core_::{
            point::point_type::ToPoint, types::fn_in_out_ref::FnInOutRef,
        },
        services::task::nested_function::{
            edge_detection::fn_falling_edge::FnFallingEdge, fn_::FnOut, fn_input::FnInput
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
    fn init_each(parent: &str, initial: Value) -> FnInOutRef {
        Rc::new(RefCell::new(Box::new(
            FnInput::new(
                parent,
                initial.to_point(0, "test"),
                match initial {
                    Value::Bool(_) => FnConfPointType::Bool,
                    Value::Int(_) => FnConfPointType::Int,
                    Value::Real(_) => FnConfPointType::Real,
                    Value::Double(_) => FnConfPointType::Double,
                    Value::String(_) => FnConfPointType::String,
                } 
            )
        )))
    }
    ///
    ///
    #[test]
    fn test_bool() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let self_id = "test_bool";
        info!("{}", self_id);
        let input = init_each(&self_id, Value::Bool(false));
        let mut fn_rising_edge = FnFallingEdge::new(
            self_id,
            input.clone(),
        );
        let test_data = vec![
            (00,    false,     false),
            (01,    false,     false),
            (02,    true,      false),
            (03,    false,     true),
            (04,    false,     false),
            (05,    true,      false),
            (06,    false,     true),
            (07,    true,      false),
            (08,    false,     true),
            (09,    false,     false),
            (10,    true,      false),
            (11,    true,      false),
            (12,    true,      false),
            (13,    false,     true),
            (14,    false,     false),
        ];
        for (step, value, target) in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point);
            // debug!("input: {:?}", &input);
            let result = fn_rising_edge.out();
            // debug!("input: {:?}", &mut input);
            debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
        assert!(result.as_bool().value.0 == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
    ///
    ///
    #[test]
    fn test_int() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let self_id = "test_int";
        info!("{}", self_id);
        let input = init_each(&self_id, Value::Int(0));
        let mut fn_rising_edge = FnFallingEdge::new(
            self_id,
            input.clone(),
        );
        let test_data = vec![
            (00,    0,      false),
            (01,    0,      false),
            (02,    1,      false),
            (03,    0,      true),
            (04,    0,      false),
            (05,    3,      false),
            (06,    0,      true),
            (07,    2,      false),
            (08,    0,      true),
            (09,    -1,     false),
            (10,    3,      false),
            (11,    77,     false),
            (12,    65,     false),
            (13,    0,      true),
            (14,    -10,    false),
        ];
        for (step, value, target) in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point);
            // debug!("input: {:?}", &input);
            let result = fn_rising_edge.out();
            // debug!("input: {:?}", &mut input);
            debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
        assert!(result.as_bool().value.0 == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
    ///
    ///
    #[test]
    fn test_real() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let self_id = "test_real";
        info!("{}", self_id);
        let input = init_each(&self_id, Value::Real(0.0));
        let mut fn_rising_edge = FnFallingEdge::new(
            self_id,
            input.clone(),
        );
        let test_data = vec![
            (00,    0.0,      false),
            (01,    0.0,      false),
            (02,    0.1,      false),
            (03,    0.0,      true),
            (04,    0.0,      false),
            (05,    3.0,      false),
            (06,    0.0,      true),
            (07,    2.0,      false),
            (08,    0.0,      true),
            (09,    -1.0,     false),
            (10,    3.0,      false),
            (11,    77.0,     false),
            (12,    65.0,     false),
            (13,    0.0,      true),
            (14,    -10.0,    false),
        ];
        for (step, value, target) in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point);
            // debug!("input: {:?}", &input);
            let result = fn_rising_edge.out();
            // debug!("input: {:?}", &mut input);
            debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
        assert!(result.as_bool().value.0 == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
    ///
    ///
    #[test]
    fn test_double() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let self_id = "test_real";
        info!("{}", self_id);
        let input = init_each(&self_id, Value::Double(0.0));
        let mut fn_rising_edge = FnFallingEdge::new(
            self_id,
            input.clone(),
        );
        let test_data = vec![
            (00,    0.0,      false),
            (01,    0.0,      false),
            (02,    0.1,      false),
            (03,    0.0,      true),
            (04,    0.0,      false),
            (05,    3.0,      false),
            (06,    0.0,      true),
            (07,    2.0,      false),
            (08,    0.0,      true),
            (09,    -1.0,     false),
            (10,    3.0,      false),
            (11,    77.0,     false),
            (12,    65.0,     false),
            (13,    0.0,      true),
            (14,    -10.0,    false),
        ];
        for (step, value, target) in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point);
            // debug!("input: {:?}", &input);
            let result = fn_rising_edge.out();
            // debug!("input: {:?}", &mut input);
            debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
        assert!(result.as_bool().value.0 == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }     
}
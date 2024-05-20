#[cfg(test)]
mod fn_average {
    use log::{debug, info};
    use testing::entities::test_value::Value;
    use std::{cell::RefCell, rc::Rc, sync::Once};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use crate::{
        conf::fn_::fn_conf_keywd::FnConfPointType, 
        core_::{
            aprox_eq::aprox_eq::AproxEq, point::point_type::ToPoint, types::fn_in_out_ref::FnInOutRef
        },
        services::task::nested_function::{
            fn_::FnOut, fn_average::FnAverage, fn_input::FnInput
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
        let enable = init_each(&self_id, Value::Bool(false));
        let input = init_each(&self_id, Value::Bool(false));
        let mut fn_average = FnAverage::new(
            self_id,
            enable.clone(),
            input.clone(),
        );
        let test_data = vec![
            (00,    true,  false,     0.000),
            (01,    true,  false,     0.000),
            (02,    true,   true,      0.333),
            (03,    true,  false,     0.250),
            (04,    true,  false,     0.200),
            (05,    true,   true,      0.333),
            (06,    true,  false,     0.285),
            (07,    true,   true,      0.375),
            (08,    true,  false,     0.333),
            (09,    true,  false,     0.300),
            (10,    true,   true,      0.363),
            (11,    true,   true,      0.416),
            (12,    true,   true,      0.333),
            (13,    true,  false,     0.388),
            (14,    true,  false,     0.444),
        ];
        for (step, en, value, target) in test_data {
            let en = en.to_point(0, "enable");
            let point = value.to_point(0, "input");
            enable.borrow_mut().add(en);
            input.borrow_mut().add(point);
            // debug!("input: {:?}", &input);
            let result = fn_average.out();
            // debug!("input: {:?}", &mut input);
            debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
            assert!(result.as_double().value.aprox_eq(target, 3), "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
    // ///
    // ///
    // #[test]
    // fn test_int() {
    //     DebugSession::init(LogLevel::Debug, Backtrace::Short);
    //     init_once();
    //     let self_id = "test_int";
    //     info!("{}", self_id);
    //     let input = init_each(&self_id, Value::Int(0));
    //     let mut fn_rising_edge = FnFallingEdge::new(
    //         self_id,
    //         input.clone(),
    //     );
    //     let test_data = vec![
    //         (00,    0,      false),
    //         (01,    0,      false),
    //         (02,    1,      false),
    //         (03,    0,      true),
    //         (04,    0,      false),
    //         (05,    3,      false),
    //         (06,    0,      true),
    //         (07,    2,      false),
    //         (08,    0,      true),
    //         (09,    -1,     false),
    //         (10,    3,      false),
    //         (11,    77,     false),
    //         (12,    65,     false),
    //         (13,    0,      true),
    //         (14,    -10,    false),
    //     ];
    //     for (step, value, target) in test_data {
    //         let point = value.to_point(0, "test");
    //         input.borrow_mut().add(point);
    //         // debug!("input: {:?}", &input);
    //         let result = fn_rising_edge.out();
    //         // debug!("input: {:?}", &mut input);
    //         debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
    //     assert!(result.as_bool().value.0 == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    //     }
    // }
    // ///
    // ///
    // #[test]
    // fn test_real() {
    //     DebugSession::init(LogLevel::Debug, Backtrace::Short);
    //     init_once();
    //     let self_id = "test_real";
    //     info!("{}", self_id);
    //     let input = init_each(&self_id, Value::Real(0.0));
    //     let mut fn_rising_edge = FnFallingEdge::new(
    //         self_id,
    //         input.clone(),
    //     );
    //     let test_data = vec![
    //         (00,    0.0,      false),
    //         (01,    0.0,      false),
    //         (02,    0.1,      false),
    //         (03,    0.0,      true),
    //         (04,    0.0,      false),
    //         (05,    3.0,      false),
    //         (06,    0.0,      true),
    //         (07,    2.0,      false),
    //         (08,    0.0,      true),
    //         (09,    -1.0,     false),
    //         (10,    3.0,      false),
    //         (11,    77.0,     false),
    //         (12,    65.0,     false),
    //         (13,    0.0,      true),
    //         (14,    -10.0,    false),
    //     ];
    //     for (step, value, target) in test_data {
    //         let point = value.to_point(0, "test");
    //         input.borrow_mut().add(point);
    //         // debug!("input: {:?}", &input);
    //         let result = fn_rising_edge.out();
    //         // debug!("input: {:?}", &mut input);
    //         debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
    //     assert!(result.as_bool().value.0 == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    //     }
    // }
    // ///
    // ///
    // #[test]
    // fn test_double() {
    //     DebugSession::init(LogLevel::Debug, Backtrace::Short);
    //     init_once();
    //     let self_id = "test_real";
    //     info!("{}", self_id);
    //     let input = init_each(&self_id, Value::Double(0.0));
    //     let mut fn_rising_edge = FnFallingEdge::new(
    //         self_id,
    //         input.clone(),
    //     );
    //     let test_data = vec![
    //         (00,    0.0,      false),
    //         (01,    0.0,      false),
    //         (02,    0.1,      false),
    //         (03,    0.0,      true),
    //         (04,    0.0,      false),
    //         (05,    3.0,      false),
    //         (06,    0.0,      true),
    //         (07,    2.0,      false),
    //         (08,    0.0,      true),
    //         (09,    -1.0,     false),
    //         (10,    3.0,      false),
    //         (11,    77.0,     false),
    //         (12,    65.0,     false),
    //         (13,    0.0,      true),
    //         (14,    -10.0,    false),
    //     ];
    //     for (step, value, target) in test_data {
    //         let point = value.to_point(0, "test");
    //         input.borrow_mut().add(point);
    //         // debug!("input: {:?}", &input);
    //         let result = fn_rising_edge.out();
    //         // debug!("input: {:?}", &mut input);
    //         debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
    //     assert!(result.as_bool().value.0 == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    //     }
    // }     
}

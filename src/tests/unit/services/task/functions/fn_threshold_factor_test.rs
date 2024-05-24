#[cfg(test)]
mod fn_threshold_factor {
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
            filter::fn_threshold::FnThreshold, fn_::FnOut, fn_input::FnInput
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
    /// Threshold Int's
    #[test]
    fn fn_threshold_int() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let self_id = "fn_threshold_int";
        info!("{}", self_id);
        let threshold = init_each(&self_id, Value::Double(0.0));
        let factor = init_each(&self_id, Value::Double(0.5));
        let input = init_each(&self_id, Value::Int(0));
        let mut fn_threshold = FnThreshold::new(
            self_id,
            threshold.clone(),
            Some(factor),
            input.clone(),
        );
        let test_data = vec![
            (00,    3.0,  0,     0),// delta
            (01,    3.0,  1,     0),// 0.5
            (01,    3.0,  1,     0),// 1.0
            (02,    3.0,  2,     0),// 2.0
            (02,    3.0,  2,     2),// 3.0 -> 2
            (03,    3.0,  3,     2),// 0.5
            (04,    3.0,  4,     2),// 1.5
            (05,    3.0,  5,     5),// 3.0 -> 5
            (06,    3.0,  4,     5),// 0.5
            (07,    3.0,  3,     5),// 1.5
            (08,    3.0,  2,     2),// 3.0 -> 2
            (09,    3.0,  1,     2),// 0.5
            (10,    3.0,  0,     2),// 1.5
            (11,    3.0,  0,     2),// 2.5
            (12,    3.0,  0,     0),// 3.5 -> 0
            (13,    3.0,  0,     0),// 0.0
            (14,    3.0,  0,     0),// 0.0
        ];
        for (step, thrh, value, target) in test_data {
            let thrh = thrh.to_point(0, "threshold");
            let point = value.to_point(0, &format!("input step {}", step));
            threshold.borrow_mut().add(thrh);
            input.borrow_mut().add(point);
            // debug!("input: {:?}", &input);
            let result = fn_threshold.out();
            // debug!("input: {:?}", &mut input);
            debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
            assert!(result.as_int().value == target, "step {}\nresult: {:?}\ntarget: {:?}", step, result, target);
            println!("------------")
        }
    }
    // ///
    // /// Threshold Reals's
    // #[test]
    // fn fn_threshold_real() {
    //     DebugSession::init(LogLevel::Debug, Backtrace::Short);
    //     init_once();
    //     let self_id = "fn_threshold_real";
    //     info!("{}", self_id);
    //     let threshold = init_each(&self_id, Value::Double(0.0));
    //     let input = init_each(&self_id, Value::Real(0.0));
    //     let mut fn_threshold = FnThreshold::new(
    //         self_id,
    //         threshold.clone(),
    //         None,
    //         input.clone(),
    //     );
    //     let test_data = vec![
    //         (00,    0.3,  0.0,     0.0),
    //         (01,    0.3,  0.1,     0.0),
    //         (02,    0.3,  0.2,     0.0),
    //         (03,    0.3,  0.3,     0.3),
    //         (04,    0.3,  0.4,     0.3),
    //         (05,    0.3,  0.5,     0.3),
    //         (06,    0.3,  0.4,     0.3),
    //         (07,    0.3,  0.3,     0.3),
    //         (08,    0.3,  0.2,     0.3),
    //         (09,    0.3,  0.1,     0.3),
    //         (10,    0.3,  0.0,     0.0),
    //         (11,    0.3,  0.0,     0.0),
    //         (12,    0.3,  0.0,     0.0),
    //         (13,    0.3,  0.0,     0.0),
    //         (14,    0.3,  0.0,     0.0),
    //     ];
    //     for (step, thrh, value, target) in test_data {
    //         let thrh = thrh.to_point(0, "threshold");
    //         let point = value.to_point(0, &format!("input step {}", step));
    //         threshold.borrow_mut().add(thrh);
    //         input.borrow_mut().add(point);
    //         // debug!("input: {:?}", &input);
    //         let result = fn_threshold.out();
    //         // debug!("input: {:?}", &mut input);
    //         debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
    //         assert!(result.as_real().value == target, "step {}\nresult: {:?}\ntarget: {:?}", step, result, target);
    //     }
    // }
    // ///
    // /// Threshold Double's
    // #[test]
    // fn fn_threshold_double() {
    //     DebugSession::init(LogLevel::Debug, Backtrace::Short);
    //     init_once();
    //     let self_id = "fn_threshold_double";
    //     info!("{}", self_id);
    //     let threshold = init_each(&self_id, Value::Double(0.0));
    //     let input = init_each(&self_id, Value::Double(0.0));
    //     let mut fn_threshold = FnThreshold::new(
    //         self_id,
    //         threshold.clone(),
    //         None,
    //         input.clone(),
    //     );
    //     let test_data = vec![
    //         (00,    0.3,  0.0,     0.0),
    //         (01,    0.3,  0.1,     0.0),
    //         (02,    0.3,  0.2,     0.0),
    //         (03,    0.3,  0.3,     0.3),
    //         (04,    0.3,  0.4,     0.3),
    //         (05,    0.3,  0.5,     0.3),
    //         (06,    0.3,  0.4,     0.3),
    //         (07,    0.3,  0.3,     0.3),
    //         (08,    0.3,  0.2,     0.3),
    //         (09,    0.3,  0.1,     0.3),
    //         (10,    0.3,  0.0,     0.0),
    //         (11,    0.3,  0.0,     0.0),
    //         (12,    0.3,  0.0,     0.0),
    //         (13,    0.3,  0.0,     0.0),
    //         (14,    0.3,  0.0,     0.0),
    //     ];
    //     for (step, thrh, value, target) in test_data {
    //         let thrh = thrh.to_point(0, "threshold");
    //         let point = value.to_point(0, &format!("input step {}", step));
    //         threshold.borrow_mut().add(thrh);
    //         input.borrow_mut().add(point);
    //         // debug!("input: {:?}", &input);
    //         let result = fn_threshold.out();
    //         // debug!("input: {:?}", &mut input);
    //         debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
    //         assert!(result.as_double().value == target, "step {}\nresult: {:?}\ntarget: {:?}", step, result, target);
    //     }
    // }
}

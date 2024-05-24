#[cfg(test)]
mod fn_smooth {
    use log::{debug, info};
    use testing::entities::test_value::Value;
    use std::{cell::RefCell, rc::Rc, sync::Once};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use crate::{
        conf::fn_::fn_conf_keywd::FnConfPointType, 
        core_::{
            point::point_type::ToPoint, types::fn_in_out_ref::FnInOutRef
        },
        services::task::nested_function::{
            filter::fn_smooth::FnSmooth, fn_::FnOut, fn_input::FnInput
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
    fn fn_smooth_int() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        let self_id = "fn_smooth_int";
        info!("{}", self_id);
        let factor = init_each(&self_id, Value::Double(0.125));
        let input = init_each(&self_id, Value::Real(0.0));
        let mut fn_smooth = FnSmooth::new(
            self_id,
            factor,
            input.clone(),
        );
        let test_data = vec![
        //  step    input  target
            (00,    0.0f32,     0.0),// delta
            (01,    0.0,        0.0),// 0.5
            (02,    3.3,        0.4125),// 1.0
            (03,    0.1,        0.3734375),// 2.0
            (04,    0.0,        0.3267578125),// 3.0 -> 2
            (05,    1.6,        0.4859130859375),// 0.5
            (06,    0.0,        0.425173950195313),// 1.5
            (07,    7.2,        1.2720272064209),// 3.0 -> 5
            (08,    0.0,        1.11302380561829),// 0.5
            (09,    0.3,        1.011395829916),// 1.5
            (10,    2.2,        1.1599713511765),// 3.0 -> 2
            (11,    8.1,        2.02747493227944),// 0.5
            (12,    1.9,        2.01154056574451),// 1.5
            (13,    0.1,        1.77259799502644),// 2.5
            (14,    0.0,        1.55102324564814),// 3.5 -> 0
            (15,    0.0,        1.35714533994212),// 0.0
            (16,    5.0,        1.81250217244936),// 0.0
        ];
        for (step, value, target) in test_data {
            let point = value.to_point(0, &format!("input step {}", step));
            input.borrow_mut().add(point);
            // debug!("input: {:?}", &input);
            let result = fn_smooth.out();
            // debug!("input: {:?}", &mut input);
            debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
            assert!(result.as_real().value == target, "step {}\nresult: {:?}\ntarget: {:?}", step, result, target);
            println!("------------")
        }
    }
    // ///
    // /// Threshold Real's
    // #[test]
    // fn fn_smooth_real() {
    //     DebugSession::init(LogLevel::Debug, Backtrace::Short);
    //     init_once();
    //     let self_id = "fn_smooth_real";
    //     info!("{}", self_id);
    //     let threshold = init_each(&self_id, Value::Double(0.0));
    //     let factor = init_each(&self_id, Value::Double(0.5));
    //     let input = init_each(&self_id, Value::Real(0.0));
    //     let mut fn_smooth = FnThreshold::new(
    //         self_id,
    //         threshold.clone(),
    //         Some(factor),
    //         input.clone(),
    //     );
    //     let test_data = vec![
    //     //  step    thrh  input  target
    //         (00,    0.3,  0.0,     0.0),// delta
    //         (01,    0.3,  0.1,     0.0),// 0.05
    //         (02,    0.3,  0.1,     0.0),// 0.10
    //         (03,    0.3,  0.2,     0.0),// 0.20
    //         (04,    0.3,  0.2,     0.2),// 0.30 -> 0.2
    //         (05,    0.3,  0.3,     0.2),// 0.05
    //         (06,    0.3,  0.4,     0.2),// 0.15
    //         (07,    0.3,  0.5,     0.5),// 0.30 -> 0.5
    //         (08,    0.3,  0.4,     0.5),// 0.05
    //         (09,    0.3,  0.3,     0.5),// 0.15
    //         (10,    0.3,  0.2,     0.5),// 0.2999
    //         (11,    0.3,  0.1,     0.1),// 0.4999 -> 0.1
    //         (12,    0.3,  0.0,     0.1),// 0.05
    //         (13,    0.3,  0.0,     0.1),// 0.10
    //         (14,    0.3,  0.0,     0.1),// 0.15
    //         (15,    0.3,  0.0,     0.1),// 0.20
    //         (16,    0.3,  0.0,     0.1),// 0.25
    //         (17,    0.3,  0.0,     0.0),// 0.30 -> 0.0
    //         (18,    0.3,  0.0,     0.0),// 0.25
    //     ];
    //     for (step, thrh, value, target) in test_data {
    //         let thrh = thrh.to_point(0, "threshold");
    //         let point = value.to_point(0, &format!("input step {}", step));
    //         threshold.borrow_mut().add(thrh);
    //         input.borrow_mut().add(point);
    //         // debug!("input: {:?}", &input);
    //         let result = fn_smooth.out();
    //         // debug!("input: {:?}", &mut input);
    //         debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
    //         assert!(result.as_real().value == target, "step {}\nresult: {:?}\ntarget: {:?}", step, result, target);
    //         println!("------------")
    //     }
    // }
    // ///
    // /// Threshold Double's
    // #[test]
    // fn fn_smooth_double() {
    //     DebugSession::init(LogLevel::Debug, Backtrace::Short);
    //     init_once();
    //     let self_id = "fn_smooth_double";
    //     info!("{}", self_id);
    //     let threshold = init_each(&self_id, Value::Double(0.0));
    //     let factor = init_each(&self_id, Value::Double(0.5));
    //     let input = init_each(&self_id, Value::Double(0.0));
    //     let mut fn_smooth = FnThreshold::new(
    //         self_id,
    //         threshold.clone(),
    //         Some(factor),
    //         input.clone(),
    //     );
    //     let test_data = vec![
    //     //  step    thrh  input  target
    //         (00,    0.3,  0.0,     0.0),// delta
    //         (01,    0.3,  0.1,     0.0),// 0.05
    //         (02,    0.3,  0.1,     0.0),// 0.10
    //         (03,    0.3,  0.2,     0.0),// 0.20
    //         (04,    0.3,  0.2,     0.2),// 0.30 -> 0.2
    //         (05,    0.3,  0.3,     0.2),// 0.05
    //         (06,    0.3,  0.4,     0.2),// 0.15
    //         (07,    0.3,  0.5,     0.5),// 0.30 -> 0.5
    //         (08,    0.3,  0.4,     0.5),// 0.05
    //         (09,    0.3,  0.3,     0.5),// 0.15
    //         (10,    0.3,  0.2,     0.2),// 0.30 -> 0.2
    //         (11,    0.3,  0.1,     0.2),// 0.05
    //         (12,    0.3,  0.0,     0.2),// 0.15
    //         (13,    0.3,  0.0,     0.2),// 0.25
    //         (14,    0.3,  0.0,     0.0),// 0.35 -> 0.0
    //         (15,    0.3,  0.0,     0.0),// 0.00
    //         (16,    0.3,  0.0,     0.0),// 0.00
    //         (17,    0.3,  0.0,     0.0),// 0.00
    //         (18,    0.3,  0.0,     0.0),// 0.00
    //     ];
    //     for (step, thrh, value, target) in test_data {
    //         let thrh = thrh.to_point(0, "threshold");
    //         let point = value.to_point(0, &format!("input step {}", step));
    //         threshold.borrow_mut().add(thrh);
    //         input.borrow_mut().add(point);
    //         // debug!("input: {:?}", &input);
    //         let result = fn_smooth.out();
    //         // debug!("input: {:?}", &mut input);
    //         debug!("step {} \t value: {:?}   |   result: {:?}", step, value, result);
    //         assert!(result.as_double().value == target, "step {}\nresult: {:?}\ntarget: {:?}", step, result, target);
    //         println!("------------")
    //     }
    // }
}

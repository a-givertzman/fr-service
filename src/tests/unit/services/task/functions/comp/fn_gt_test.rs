#[cfg(test)]
mod fn_ge {
    use log::{debug, info};
    use std::{sync::Once, rc::Rc, cell::RefCell};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::fn_::fn_conf_keywd::FnConfPointType, 
        core_::{point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
        services::task::nested_function::{comp::fn_ge::FnGe, fn_::FnOut, fn_input::FnInput}
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
        Rc::new(RefCell::new(Box::new(
            FnInput::new("test", initial, type_)
        )))
    }
    ///
    /// Testing Task Gt Bool's
    #[test]
    fn test_bool() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "test_bool";
        info!("{}", self_id);
        let mut target: bool;
        let input1 = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
        let input2 = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
        let mut fn_ge = FnGe::new(
            self_id,
            input1.clone(),
            input2.clone(),
        );
        let test_data = vec![
            (00, false, false),
            (01, false, true),
            (02, true,  false),
            (03, true,  true),
        ];
        for (step, value1, value2) in test_data {
            let point1 = value1.to_point(0, "test");
            let point2 = value2.to_point(0, "test");
            input1.borrow_mut().add(point1.clone());
            input2.borrow_mut().add(point2.clone());
            let result = fn_ge.out().as_bool().value.0;
            debug!("step {}  |  value1: {:?} >= value2: {:?} | result: {:?}", step, value1, value2, result);
            target = value1 >= value2;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
    }
    ///
    /// Testing Task Gt Int's
    #[test]
    fn test_int() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "test_int";
        info!("{}", self_id);
        let mut target: bool;
        let input1 = init_each(0.to_point(0, "int"), FnConfPointType::Int);
        let input2 = init_each(0.to_point(0, "int"), FnConfPointType::Int);
        let mut fn_ge = FnGe::new(
            self_id,
            input1.clone(),
            input2.clone(),
        );
        let test_data = vec![
            (00, 1, 5),
            (01, 5, 1),
            (02, 3,  3),
            (03, -1,  -5),
            (04, -5,  -1),
            (05, -4,  -4),
            (06, 4,  0),
            (07, 0,  4),
            (08, 0,  0),
            (09, -4,  0),
            (10, 0,  -4),
        ];
        for (step, value1, value2) in test_data {
            let point1 = value1.to_point(0, "test");
            let point2 = value2.to_point(0, "test");
            input1.borrow_mut().add(point1.clone());
            input2.borrow_mut().add(point2.clone());
            let result = fn_ge.out().as_bool().value.0;
            debug!("step {}  |  value1: {:?} >= value2: {:?} | result: {:?}", step, value1, value2, result);
            target = value1 >= value2;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
    }
    ///
    /// Testing Gt Real's
    #[test]
    fn test_real() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "test_real";
        info!("{}", self_id);
        let mut target: bool;
        let input1 = init_each(0.0f32.to_point(0, "int"), FnConfPointType::Real);
        let input2 = init_each(0.0f32.to_point(0, "int"), FnConfPointType::Real);
        let mut fn_ge = FnGe::new(
            self_id,
            input1.clone(),
            input2.clone(),
        );
        let test_data = vec![
            (01, 0.1, 0.1),
            (02, 0.2, 0.2),
            (03, 0.5, 0.5),
            (04, -0.1, 0.1),
            (05, -0.5, 0.1),
            (06, 0.1, -0.1),
            (07, 0.1, -0.5),
            (08, 0.0, 0.0),
            (09, f32::MIN, 0.0),
            (10, f32::MIN, 0.5),
            (11, f32::MIN, 1.0),
            (12, 0.0, f32::MIN),
            (13, 0.5, f32::MIN),
            (14, 1.0, f32::MIN),
            (15, f32::MAX, 0.0),
            (16, f32::MAX, 0.5),
            (17, f32::MAX, 1.0),
            (18, 0.0, f32::MAX),
            (19, 0.5, f32::MAX),
            (20, 1.0, f32::MAX),
        ];
        for (step, value1, value2) in test_data {
            let point1 = value1.to_point(0, "test");
            let point2 = value2.to_point(0, "test");
            input1.borrow_mut().add(point1.clone());
            input2.borrow_mut().add(point2.clone());
            let result = fn_ge.out().as_bool().value.0;
            debug!("step {}  |  value1: {:?} >= value2: {:?} | result: {:?}", step, value1, value2, result);
            target = value1 >= value2;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
    }
    ///
    /// Testing Gt Double's
    #[test]
    fn test_double() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "test_double";
        info!("{}", self_id);
        let mut target: bool;
        let input1 = init_each(0.0f64.to_point(0, "int"), FnConfPointType::Double);
        let input2 = init_each(0.0f64.to_point(0, "int"), FnConfPointType::Double);
        let mut fn_ge = FnGe::new(
            self_id,
            input1.clone(),
            input2.clone(),
        );
        let test_data = vec![
            (01, 0.1, 0.1),
            (02, 0.2, 0.2),
            (03, 0.5, 0.5),
            (04, -0.1, 0.1),
            (05, -0.5, 0.1),
            (06, 0.1, -0.1),
            (07, 0.1, -0.5),
            (08, 0.0, 0.0),
            (09, f64::MIN, 0.0),
            (10, f64::MIN, 0.5),
            (11, f64::MIN, 1.0),
            (12, 0.0, f64::MIN),
            (13, 0.5, f64::MIN),
            (14, 1.0, f64::MIN),
            (15, f64::MAX, 0.0),
            (16, f64::MAX, 0.5),
            (17, f64::MAX, 1.0),
            (18, 0.0, f64::MAX),
            (19, 0.5, f64::MAX),
            (20, 1.0, f64::MAX),
        ];
        for (step, value1, value2) in test_data {
            let point1 = value1.to_point(0, "test");
            let point2 = value2.to_point(0, "test");
            input1.borrow_mut().add(point1.clone());
            input2.borrow_mut().add(point2.clone());
            let result = fn_ge.out().as_bool().value.0;
            debug!("step {}  |  value1: {:?} >= value2: {:?} | result: {:?}", step, value1, value2, result);
            target = value1 >= value2;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
    }
}

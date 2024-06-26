#[cfg(test)]
mod fn_bit_and {
    use log::{debug, info};
    use std::{sync::Once, rc::Rc, cell::RefCell};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::fn_::fn_conf_keywd::FnConfPointType, 
        core_::{point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
        services::task::nested_function::{ops::fn_bit_and::FnBitAnd, fn_::FnOut, fn_input::FnInput}
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
    /// Testing Task Eq Bool's
    #[test]
    fn test_bool() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "test_bool";
        info!("{}", self_id);
        let mut target: bool;
        let input1 = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
        let input2 = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
        let mut fn_bit_and = FnBitAnd::new(
            self_id,
            vec![
                input1.clone(),
                input2.clone(),
            ],
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
            let result = fn_bit_and.out().as_bool().value.0;
            debug!("step {}  |  value1: {:?} & value2: {:?} | result: {:?}", step, value1, value2, result);
            target = value1 & value2;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
    }
    ///
    /// Testing Task Eq Int's
    #[test]
    fn test_int() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "test_int";
        info!("{}", self_id);
        let mut target: i64;
        let input1 = init_each(0.to_point(0, "int"), FnConfPointType::Int);
        let input2 = init_each(0.to_point(0, "int"), FnConfPointType::Int);
        let mut fn_bit_and = FnBitAnd::new(
            self_id,
            vec![
                input1.clone(),
                input2.clone(),
            ],
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
            let result = fn_bit_and.out().as_int().value;
            debug!("step {}  |  value1: {:?} & value2: {:?} | result: {:?}", step, value1, value2, result);
            target = value1 & value2;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
    }
}
#[cfg(test)]
mod fn_add {
    use log::{debug, info};
    use std::{sync::Once, rc::Rc, cell::RefCell};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::fn_::fn_conf_keywd::FnConfPointType, 
        core_::{point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
        services::task::nested_function::{fn_::FnOut, fn_add::FnAdd, fn_input::FnInput}
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
    /// Testing Task Add Bool's
    #[test]
    fn test_bool() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        info!("test_bool");
        let mut value1_stored = false.to_point(0, "bool");
        let mut value2_stored = false.to_point(0, "bool");
        let mut target: PointType;
        let input1 = init_each(value1_stored.clone(), FnConfPointType::Bool);
        let input2 = init_each(value2_stored.clone(), FnConfPointType::Bool);
        let mut fn_add = FnAdd::new(
            "test",
            input1.clone(),
            input2.clone(),
        );
        let test_data = vec![
            (false, false),
            (false, true),
            (false, false),
            (true, false),
            (false, false),
            (true, true),
            (false, false),
        ];
        for (value1, value2) in test_data {
            let point1 = value1.to_point(0, "test");
            let point2 = value2.to_point(0, "test");
            input1.borrow_mut().add(point1.clone());
            let state = fn_add.out();
            debug!("value1: {:?}   |   state: {:?}", value1, state);
            value1_stored = point1.clone();
            target = PointType::Bool(value1_stored.as_bool() + value2_stored.as_bool());
            assert_eq!(state, target);
            input2.borrow_mut().add(point2.clone());
            let state = fn_add.out();
            debug!("value2: {:?}   |   state: {:?}", value2, state);
            value2_stored = point2.clone();
            target = PointType::Bool(value1_stored.as_bool() + value2_stored.as_bool());
            assert_eq!(state, target);
            println!();
        }
    }
    ///
    /// Testing Task Add Int's
    #[test]
    fn test_int() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        info!("test_int");
        let mut value1_stored = 0.to_point(0, "int");
        let mut value2_stored = 0.to_point(0, "int");
        let mut target: PointType;
        let input1 = init_each(value1_stored.clone(), FnConfPointType::Int);
        let input2 = init_each(value2_stored.clone(), FnConfPointType::Int);
        let mut fn_add = FnAdd::new(
            "test",
            input1.clone(),
            input2.clone(),
        );
        let test_data = vec![
            (1, 1),
            (2, 2),
            (5, 5),
            (-1, 1),
            (-5, 1),
            (1, -1),
            (1, -5),
            (0, 0),
            (i64::MIN, 0),
            (0, i64::MIN),
            (i64::MAX, 0),
            (0, i64::MAX),
        ];
        for (value1, value2) in test_data {
            let point1 = value1.to_point(0, "test");
            let point2 = value2.to_point(0, "test");
            input1.borrow_mut().add(point1.clone());
            let state = fn_add.out();
            debug!("value1: {:?}   |   state: {:?}", value1, state);
            value1_stored = point1.clone();
            target = PointType::Int(value1_stored.as_int() + value2_stored.as_int());
            assert_eq!(state, target);
            input2.borrow_mut().add(point2.clone());
            let state = fn_add.out();
            debug!("value2: {:?}   |   state: {:?}", value2, state);
            value2_stored = point2.clone();
            target = PointType::Int(value1_stored.as_int() + value2_stored.as_int());
            assert_eq!(state, target);
            println!();
        }
    }
}

#[cfg(test)]

mod tests {
    use log::{debug, info};
    use std::{sync::Once, rc::Rc, cell::RefCell};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::fn_::fn_conf_keywd::FnConfPointType, core_::{
            point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, services::task::nested_function::{fn_::FnOut,
            fn_input::FnInput, fn_var::FnVar
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
        Rc::new(RefCell::new(
            Box::new(
                FnInput::new("test", initial, type_)
            )
        ))
    }
    ///
    ///
    #[test]
    fn test_bool() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        info!("test_bool");
        let input = init_each(false.to_point(0, "bool"), FnConfPointType::Bool);
        let mut fn_var = FnVar::new(
            "test",
            input.clone(),
        );
        let test_data = vec![
            false,
            false,
            true,
            false,
            false,
            true,
            false,
            true,
            false,
            false,
            true,
            true,
            false,
            false,
        ];
        for value in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point.clone());
            // debug!("input: {:?}", &input);
            fn_var.eval();
            let state = fn_var.out();
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, point);
        }
    }


    #[test]
    fn test_int() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        info!("test_int");
        let input = init_each(0.to_point(0, "int"), FnConfPointType::Int);
        let mut fn_var = FnVar::new(
            "test",
            input.clone(),
        );
        let test_data = vec![
            0,
            1,
            2,
            4,
            123,
            0,
            -234,
            4,
            23,
            i64::MIN,
            i64::MAX,
        ];
        for value in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point.clone());
            // debug!("input: {:?}", &input);
            fn_var.eval();
            let state = fn_var.out();
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, point);
        }
    }
    ///
    ///
    #[test]
    fn test_real() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        info!("test_real");
        let input = init_each(0.0f32.to_point(0, "real"), FnConfPointType::Real);
        let mut fn_var = FnVar::new(
            "test",
            input.clone(),
        );
        let test_data = vec![
            0.0f32,
            0.1,
            -0.2,
            0.4,
            0.123,
            0.0,
            -0.234,
            0.4,
            0.23,
            f32::MIN,
            f32::MAX,
        ];
        for value in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point.clone());
            // debug!("input: {:?}", &input);
            fn_var.eval();
            let state = fn_var.out();
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, point);
        }
    }
    ///
    ///
    #[test]
    fn test_double() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        info!("test_double");
        let input = init_each(0.0f64.to_point(0, "double"), FnConfPointType::Double);
        let mut fn_var = FnVar::new(
            "test",
            input.clone(),
        );
        let test_data = vec![
            0.0f64,
            0.1,
            -0.2,
            0.4,
            0.123,
            0.0,
            -0.234,
            0.4,
            0.23,
            f64::MIN,
            f64::MAX,
        ];
        for value in test_data {
            let point = value.to_point(0, "test");
            input.borrow_mut().add(point.clone());
            // debug!("input: {:?}", &input);
            fn_var.eval();
            let state = fn_var.out();
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, point);
        }
    }
}

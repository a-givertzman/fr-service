#[cfg(test)]
mod fn_rising_edge {
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
            edge_detection::fn_rising_edge::FnRisingEdge, fn_::FnOut, fn_input::FnInput
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
        Rc::new(RefCell::new(
            Box::new(
                FnInput::new(parent, initial.to_point(0, "test"), FnConfPointType::Bool)
            )
        ))
    }
    ///
    ///
    #[test]
    fn test_single() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "test_single";
        info!("{}", self_id);
        let input = init_each(&self_id, Value::Bool(false));
        let mut fn_rising_edge = FnRisingEdge::new(
            self_id,
            input.clone(),
        );
        let test_data = vec![
            (00,    false,     false),
            (01,    false,     false),
            (02,    true,      true),
            (03,    false,     false),
            (04,    false,     false),
            (05,    true,      true),
            (06,    false,     false),
            (07,    true,      true),
            (08,    false,     false),
            (09,    false,     false),
            (10,    true,      true),
            (11,    true,      false),
            (12,    true,      false),
            (13,    false,     false),
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
}

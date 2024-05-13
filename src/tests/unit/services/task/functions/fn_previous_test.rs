#[cfg(test)]

mod fn_previous {
    use log::debug;
    use testing::entities::test_value::Value;
    use std::{sync::Once, rc::Rc, cell::RefCell};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::{fn_::fn_conf_keywd::FnConfPointType, point_config::{name::Name, point_config::PointConfig}},
        core_::{point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef},
        services::task::nested_function::{fn_::{FnOut, FnResult}, fn_input::FnInput, fn_point_id::FnPointId, fn_previous::FnPrevious},
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
                FnInput::new("test","test", Some(initial), type_)
            )
        ))
    }
    ///
    ///
    #[test]
    fn basic() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        let self_id = "fn_previous_test";
        println!("{}", self_id);
        let input = init_each(0.to_point(0, "test"), FnConfPointType::Any);
        let mut fn_previous = FnPrevious::new(self_id, input.clone());
        let test_data = vec![
            (Some(Value::Bool(false)),  Value::Bool(false)),
            (None,                      Value::Bool(false)),
            (Some(Value::Bool(true)),   Value::Bool(true)),
            (None,                      Value::Bool(true)),
            (None,                      Value::Bool(true)),
            (Some(Value::Int(0)),             Value::Int(0)),
            (Some(Value::Int(1)),             Value::Int(1)),
            (Some(Value::Int(2)),             Value::Int(2)),
            (None,                            Value::Int(2)),
            (Some(Value::Int(3)),             Value::Int(3)),
            (Some(Value::Int(3)),             Value::Int(3)),
            (Some(Value::Int(-1)),            Value::Int(-1)),
            (None,                            Value::Int(-1)),
            (Some(Value::Int(-2)),            Value::Int(-2)),
            (Some(Value::Int(-3)),            Value::Int(-3)),
            (Some(Value::Int(-4)),            Value::Int(-4)),
            (None,                            Value::Int(-4)),
            (Some(Value::Real(5.0)),          Value::Real(5.0)),
            (None,                            Value::Real(5.0)),
            (Some(Value::Real(6.0)),          Value::Real(6.0)),
            (Some(Value::Real(5.0)),          Value::Real(5.0)),
            (Some(Value::Real(4.0)),          Value::Real(4.0)),
            (None,                            Value::Real(4.0)),
            (None,                            Value::Real(4.0)),
            (None,                            Value::Real(4.0)),
            (Some(Value::Real(-3.0)),         Value::Real(-3.0)),
            (None,                            Value::Real(-3.0)),
            (Some(Value::Double(2.1)),        Value::Double(2.1)),
            (Some(Value::Double(1.1)),        Value::Double(1.1)),
            (None,                            Value::Double(1.1)),
            (Some(Value::Double(0.1)),        Value::Double(0.1)),
            (None,                            Value::Double(0.1)),
            (Some(Value::Double(-0.1)),       Value::Double(-0.1)),
            (None,                            Value::Double(-0.1)),
            (None,                            Value::Double(-0.1)),
            (None,                            Value::Double(-0.1)),
            (Some(Value::Double(-0.1)),       Value::Double(-0.1)),
            (Some(Value::String("2.1".to_owned())),     Value::String("2.1".to_owned())),
            (None,                                      Value::String("2.1".to_owned())),
            (Some(Value::String("1.1".to_owned())),     Value::String("1.1".to_owned())),
            (Some(Value::String("0.1".to_owned())),     Value::String("0.1".to_owned())),
            (Some(Value::String("-0.1".to_owned())),    Value::String("-0.1".to_owned())),
            (Some(Value::String("-0.1".to_owned())),    Value::String("-0.1".to_owned())),
        ];
        for (value, target) in test_data {
            if let Some(value) = &value {
                let point = value.to_point(0, "test");
                input.borrow_mut().add(point);
            }
            // debug!("input: {:?}", &input);
            let state = match fn_previous.out() {
                FnResult::Ok(point) => match point {
                    PointType::Bool(value) => Value::Bool(value.value.0),
                    PointType::Int(value) => Value::Int(value.value),
                    PointType::Real(value) => Value::Real(value.value),
                    PointType::Double(value) => Value::Double(value.value),
                    PointType::String(value) => Value::String(value.value),
                }
                FnResult::Err(_) => panic!("Must returns Ok, but Err is returned"),
                FnResult::None => panic!("Must returns Ok, but None is returned"),
            };
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, target);
        }
    }
}
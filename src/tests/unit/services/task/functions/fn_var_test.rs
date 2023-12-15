#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{debug, info};
    use std::{sync::Once, rc::Rc, cell::RefCell};
    
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
        services::task::nested_function::{fn_::{FnInOut, FnOut}, 
        fn_input::FnInput, fn_var::FnVar},
    };
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn initOnce() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach(initial: PointType) -> FnInOutRef {
        fn boxFnInput(input: FnInput) -> Box<(dyn FnInOut)> {
            Box::new(input)
        }
        Rc::new(RefCell::new(
            boxFnInput(
                FnInput::new("test", initial)
            )
        ))
    }
    

    #[test]
    fn test_bool() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        info!("test_bool");
        let input = initEach(false.toPoint(0, "bool"));
        let mut fnVar = FnVar::new(
            "test",
            input.clone(),
        );
        let testData = vec![
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
        for value in testData {
            let point = value.toPoint(0, "test");
            input.borrow_mut().add(point.clone());
            // debug!("input: {:?}", &input);
            fnVar.eval();
            let state = fnVar.out();
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, point);
        }        
    }


    #[test]
    fn test_int() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        info!("test_int");
        let input = initEach(false.toPoint(0, "bool"));
        let mut fnVar = FnVar::new(
            "test",
            input.clone(),
        );
        let testData = vec![
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
        for value in testData {
            let point = value.toPoint(0, "test");
            input.borrow_mut().add(point.clone());
            // debug!("input: {:?}", &input);
            fnVar.eval();
            let state = fnVar.out();
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, point);
        }        
    }


    #[test]
    fn test_float() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        info!("test_float");
        let input = initEach(false.toPoint(0, "bool"));
        let mut fnVar = FnVar::new(
            "test",
            input.clone(),
        );
        let testData = vec![
            0.0,
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
        for value in testData {
            let point = value.toPoint(0, "test");
            input.borrow_mut().add(point.clone());
            // debug!("input: {:?}", &input);
            fnVar.eval();
            let state = fnVar.out();
            // debug!("input: {:?}", &mut input);
            debug!("value: {:?}   |   state: {:?}", value, state);
            assert_eq!(state, point);
        }        
    }
}

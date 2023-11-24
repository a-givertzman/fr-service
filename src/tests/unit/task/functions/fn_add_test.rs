#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{debug, info};
    use std::{sync::Once, rc::Rc, cell::RefCell};
    
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
        point::point_type::{PointType, ToPoint}, types::fn_in_out_ref::FnInOutRef}, 
        services::task::nested_function::{fn_::{FnInOut, FnOut}, 
        fn_input::FnInput, fn_var::FnVar, fn_add::FnAdd},
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
        Rc::new(RefCell::new(
            Box::new(
                FnInput::new("test", initial)
            )
        ))
    }
    

    #[test]
    fn test_bool() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        info!("test_bool");
        let mut value1Stored = false.toPoint("bool");
        let mut value2Stored = false.toPoint("bool");
        let mut target: PointType;
        let input1 = initEach(value1Stored.clone());
        let input2 = initEach(value2Stored.clone());
        let mut fnAdd = FnAdd::new(
            "test",
            input1.clone(),
            input2.clone(),
        );
        let testData = vec![
            (false, false),
            (false, true),
            (false, false),
            (true, false),
            (false, false),
            (true, true),
            (false, false),
        ];
        for (value1, value2) in testData {
            let point1 = value1.toPoint("test");
            let point2 = value2.toPoint("test");
            input1.borrow_mut().add(point1.clone());
            let state = fnAdd.out();
            debug!("value1: {:?}   |   state: {:?}", value1, state);
            value1Stored = point1.clone();
            target = PointType::Bool(value1Stored.asBool() + value2Stored.asBool());
            assert_eq!(state, target);
            input2.borrow_mut().add(point2.clone());
            let state = fnAdd.out();
            debug!("value2: {:?}   |   state: {:?}", value2, state);
            value2Stored = point2.clone();
            target = PointType::Bool(value1Stored.asBool() + value2Stored.asBool());
            assert_eq!(state, target);
            println!("");
        }        
    }


    #[test]
    fn test_int() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        info!("test_int");
        let mut value1Stored = 0.toPoint("int");
        let mut value2Stored = 0.toPoint("int");
        let mut target: PointType;
        let input1 = initEach(value1Stored.clone());
        let input2 = initEach(value2Stored.clone());
        let mut fnAdd = FnAdd::new(
            "test",
            input1.clone(),
            input2.clone(),
        );
        let testData = vec![
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
        for (value1, value2) in testData {
            let point1 = value1.toPoint("test");
            let point2 = value2.toPoint("test");
            input1.borrow_mut().add(point1.clone());
            let state = fnAdd.out();
            debug!("value1: {:?}   |   state: {:?}", value1, state);
            value1Stored = point1.clone();
            target = PointType::Int(value1Stored.asInt() + value2Stored.asInt());
            assert_eq!(state, target);
            input2.borrow_mut().add(point2.clone());
            let state = fnAdd.out();
            debug!("value2: {:?}   |   state: {:?}", value2, state);
            value2Stored = point2.clone();
            target = PointType::Int(value1Stored.asInt() + value2Stored.asInt());
            assert_eq!(state, target);
            println!("");
        }        
    }
}

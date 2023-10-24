#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::sync::Once;

use crate::core_::{aprox_eq::aprox_eq::AproxEq, debug::debug_session::{DebugSession, LogLevel}, format::format::Format};

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
fn initEach() -> () {

}

#[test]
fn test_bool() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_bool");

    // let (initial, switches) = initEach();
    let testData = vec![
        ("abc {a} xyz {b} rty {c} str {d}.", (false, 12, 1.618, "1223"), "abc false xyz 12 rty 1.618 str 1223."),
        ("abc {a} xyz '{b}' rty \"{c}\" str '{d}'.", (false, 12, 1.618, "1223"), "abc false xyz '12' rty \"1.618\" str '1223'."),
        ("abc {a} xyz '{b}' rty \"{c}\" str \"{d}\".", (false, 12, 1.618, "1223"), "abc false xyz '12' rty \"1.618\" str \"1223\"."),
    ];
    // fn boxValue(v: impl ToString + 'static) -> Box<dyn ToString> {
    //     Box::new(v)
    // }
    for (input, values, target) in testData {
        // for v in values {
            
        // }
        // let values = values
        let mut format = Format::new(input);
        format.insert("a", values.0);
        format.insert("b", values.1);
        format.insert("c", values.2);
        format.insert("d", values.3);
        debug!("result: {}", format);
        assert!(format.out() == target, "format != target \nformat: {} \ntarget: {}", format.out(), target);
        // debug!("value: {:?}   |   target: {:?}  |    decimals: {:?}     |   aproxEq: {:?}", value, target, decimals, aproxEq);
        // assert_eq!(aproxEq, true, "value: {:?}   |   target: {:?}  |    decimals: {:?}    |   aproxEq: {:?}", value, target, decimals, aproxEq);
    }        
}

// #[test]
fn test_int() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_int");

     
}

// #[test]
fn test_float() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_float");

    // let (initial, switches) = initEach();
    let testData = vec![
        (16, (1.0123456789123456f64, 1.0123456789123456f64)),
        (15, (12.0123456789123451f64, 12.0123456789123456f64)),
        (14, (123.0123456789123411f64, 123.0123456789123456f64)),
        (13, (1234.0123456789123111f64, 1234.0123456789123456f64)),
        (12, (12345.0123456789121111f64, 12345.0123456789123456f64)),
        (11, (123456.0123456789111111f64, 123456.0123456789123456f64)),
        (10, (1234567.0123456789011111f64, 1234567.0123456789123456f64)),
        (9, (12345678.0123456789011111f64, 12345678.0123456789123456f64)),
        (8, (123456789.0123456789111111f64, 123456789.0123456789123456f64)),
        (7, (1234567890.0123456781111111f64, 1234567890.0123456789123456f64)),
        (6, (12345678901.0123456111111111f64, 12345678901.0123456789123456f64)),
        (5, (123456789012.0123451111111111f64, 123456789012.0123456789123456f64)),
        (4, (1234567890123.0123411111111111f64, 1234567890123.0123456789123456f64)),
        (3, (12345678901234.0123111111111111f64, 12345678901234.0123456789123456f64)),
        (2, (123456789012345.0121111111111111f64, 123456789012345.0123456789123456f64)),
        (1, (1234567890123456.0111111111111111f64, 1234567890123456.0123456789123456f64)),
        (0, (12345678901234567.0111111111111111f64, 12345678901234567.0123456789123456f64)),
    ];
    for (decimals, (value, target)) in testData {
        let aproxEq = value.aproxEq(target, decimals);
        debug!("value: {:?}   |   target: {:?}  |    decimals: {:?}     |   aproxEq: {:?}", value, target, decimals, aproxEq);
        assert_eq!(aproxEq, true, "value: {:?}   |   target: {:?}  |    decimals: {:?}    |   aproxEq: {:?}", value, target, decimals, aproxEq);
    }        
}

#![allow(non_snake_case)]
#[cfg(test)]
use log::{debug, info};
use std::sync::Once;

use crate::core_::{debug::debug_session::{DebugSession, LogLevel}, format::format::Format};

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
fn test_simple_name() {
    DebugSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_bool");

    // let (initial, switches) = initEach();
    let testData = vec![
        ("abc {a} xyz {b} rty {c} str {d}.", (false, 12, 1.618, "1223"), "abc false xyz 12 rty 1.618 str 1223."),
        ("abc {a} xyz '{b}' rty \"{c}\" str '{d}'.", (false, 12, 1.618, "1223"), "abc false xyz '12' rty \"1.618\" str '1223'."),
        ("abc {a} xyz '{b}' rty \"{c}\" str \"{d}\".", (false, 12, 1.618, "1223"), "abc false xyz '12' rty \"1.618\" str \"1223\"."),
    ];
    for (input, values, target) in testData {
        let mut format = Format::new(input);
        format.insert("a", values.0);
        format.insert("b", values.1);
        format.insert("c", values.2);
        format.insert("d", values.3);
        debug!("result: {}", format);
        assert!(format.out() == target, "format != target \nformat: {} \ntarget: {}", format.out(), target);
    }        
}


#[test]
fn test_dot_name() {
    DebugSession::init(LogLevel::Trace);
    initOnce();
    initEach();
    info!("test_dot_name");

    // let (initial, switches) = initEach();
    let testData = vec![
        ("abc {a.value} xyz {b.name} rty {c.timestamp} str {c.id}.", (false, 12, 1.618, "1223"), "abc false xyz 12 rty 1.618 str 1223."),
    ];
    for (input, values, target) in testData {
        let mut format = Format::new(input);
        format.insert("a.value", values.0);
        format.insert("b.name", values.1);
        format.insert("c.timestamp", values.2);
        format.insert("c.id", values.3);
        debug!("result: {}", format);
        assert!(format.out() == target, "format != target \nformat: {} \ntarget: {}", format.out(), target);
    }        
}

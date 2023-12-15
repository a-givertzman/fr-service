#![allow(non_snake_case)]
use log::trace;
#[cfg(test)]
use log::{debug, info};
use regex::RegexBuilder;
use std::sync::Once;

use crate::core_::{debug::debug_session::*, format::format::Format, point::point_type::ToPoint};

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
    DebugSession::init(LogLevel::Info, Backtrace::Short);
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
        format.insert("a", values.0.toPoint(0, ""));
        format.insert("b", values.1.toPoint(0, ""));
        format.insert("c", values.2.toPoint(0, ""));
        format.insert("d", values.3.toPoint(0, ""));
        debug!("result: {}", format);
        assert!(format.out() == target, "format != target \nformat: {} \ntarget: {}", format.out(), target);
    }        
}


#[test]
fn test_name_sufix() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_name_sufix");

    // let (initial, switches) = initEach();
    let testData = vec![
        ("abc {a.value} xyz {b.name} rty {c.timestamp} str {c.id}.", (false, 12, 1.618, "1223"), r"abc false xyz  rty {c.timestamp} UTC str {c.id}."),
        ("abc {a.value} xyz {b.name} rty {c.timestamp} str {c.id}.", (false, 02, 0.618, "1223"), r"abc false xyz  rty {c.timestamp} UTC str {c.id}."),
    ];
    for (input, values, target) in testData {
        let mut format = Format::new(input);
        format.insert("a.value", values.0.toPoint(0, ""));
        format.insert("b.name", values.1.toPoint(0, ""));
        format.insert("c.timestamp", values.2.toPoint(0, ""));
        debug!("result: {}", format);
        let out = format.out();
        let target = target.replace(
            "{c.timestamp}",
            values.2.toPoint(0, "").timestamp().to_rfc3339_opts(chrono::SecondsFormat::Secs, true).replace("T", " ").replace("Z", "").as_str(),
        );
        let re = format!(
            r"(abc false xyz  rty {})(\.\d{{9}})( UTC str \{{c\.id\}}\.)", 
            values.2.toPoint(0, "").timestamp().to_rfc3339_opts(chrono::SecondsFormat::Secs, true).replace("T", " ").replace("Z", ""),
        );
        trace!("re: {}", re);
        let re = RegexBuilder::new(&re).multi_line(false).build().unwrap();
        let out = re.replace(&out, "$1$3");
        trace!("out: {}", out);
        assert!(out == target, "format != target \nformat: {} \ntarget: {}", out, target);
    }        
}

#[test]
fn test_prepare() {
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_prepare");
    // let (initial, switches) = initEach();

    let mut format = Format::new("abc {const} xyz '{b.name}' rty {c.value} str {c.timestamp}.");
    format.insert("const", 12345.toPoint(0, ""));
    format.insert("b.name", "".toPoint(0, "the.name"));
    trace!("format: {}", format);
    format.prepare();
    let target = "abc 12345 xyz 'the.name' rty {c.value} str {c.timestamp}.";
    assert!(format.out() == target, "prepared format != target \nformat: {} \ntarget: {}", format, target);

    let testData = vec![
        (1.618, r"abc 12345 xyz 'the.name' rty {c.value} str {c.timestamp} UTC."),
        (0.618, r"abc 12345 xyz 'the.name' rty {c.value} str {c.timestamp} UTC."),
    ];
    for (values, target) in testData {
        format.insert("a.value", values.toPoint(0, ""));
        format.insert("c.timestamp", values.toPoint(0, ""));
        debug!("result: {}", format);
        let out = format.out();
        let target = target.replace(
            "{c.timestamp}",
            values.toPoint(0, "").timestamp().to_rfc3339_opts(chrono::SecondsFormat::Secs, true).replace("T", " ").replace("Z", "").as_str(),
        );
        let re = r"(.+)(\.\d+)( UTC)";
            // r"(abc false xyz  rty {})(\.\d{{9}})( UTC str \{{c\.id\}}\.)", 
        // );
        // values.toPoint("").timestamp().to_rfc3339_opts(chrono::SecondsFormat::Secs, true).replace("T", " ").replace("Z", ""),
        trace!("re: {}", re);
        let re = RegexBuilder::new(&re).multi_line(false).build().unwrap();
        let out = re.replace(&out, "$1$3");
        trace!("out: {}", out);
        assert!(out == target, "format != target \nformat: {} \ntarget: {}", out, target);
    }        
}

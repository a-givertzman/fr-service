#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use log::{warn, info, debug};
    use serde_json::json;
    use std::{sync::{Once, mpsc}, time::{Duration, Instant}};
    use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::{point_type::PointType, point::Point}, types::bool::Bool, net::protocols::jds::jds_serialize::JdsSerialize}, tcp::steam_read::StreamRead}; 
    
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
    fn ts() -> DateTime<Utc> {
        chrono::offset::Utc::now()
    }
    fn tsStr(ts: DateTime<Utc>) -> String {
        ts.to_rfc3339()
    }
    
    #[test]
    fn test_() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_");
        let name = "/server/line1/ied1/test1";
        let ts = ts();
        // debug!("timestamp: {:?}", ts);j
        let testData = [
            (format!(r#"{{"type": "Bool",  "name": "{}", "value": false,   "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Bool(Point::new(name, Bool(false), 0, ts))),
            (format!(r#"{{"type": "Bool",  "name": "{}", "value": true,    "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Bool(Point::new(name, Bool(true),  0, ts))),
            (format!(r#"{{"type": "Int",   "name": "{}", "value": 1,   "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Int(Point::new(name, 1, 0, ts))),
            (format!(r#"{{"type": "Int",   "name": "{}", "value": -9223372036854775808,   "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Int(Point::new(name, -9223372036854775808, 0, ts))),
            (format!(r#"{{"type": "Int",   "name": "{}", "value":  9223372036854775807,   "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Int(Point::new(name,  9223372036854775807, 0, ts))),
            (format!(r#"{{"type": "Float", "name": "{}", "value":  0.0, "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Float(Point::new(name,  0.0, 0, ts))),
            (format!(r#"{{"type": "Float", "name": "{}", "value": -1.1, "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Float(Point::new(name, -1.1, 0, ts))),
            (format!(r#"{{"type": "Float", "name": "{}", "value":  1.1, "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Float(Point::new(name,  1.1, 0, ts))),
            (format!(r#"{{"type": "Float", "name": "{}", "value": -1.7976931348623157e308, "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Float(Point::new(name, -1.7976931348623157e308, 0, ts))),
            (format!(r#"{{"type": "Float", "name": "{}", "value":  1.7976931348623157e308, "status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::Float(Point::new(name,  1.7976931348623157e308, 0, ts))),
            (format!(r#"{{"type": "String","name": "{}", "value": "~!@#$%^&*()_+`1234567890-=","status": 0, "timestamp":"{}"}}"#, name, tsStr(ts)), PointType::String(Point::new(name, "~!@#$%^&*()_+`1234567890-=".to_string(), 0, ts))),
        ];
        let (send, recv) = mpsc::channel();
        let mut jdsSerialize = JdsSerialize::new("test", recv);
        for (target, point) in testData {
            send.send(point).unwrap();
            let result = jdsSerialize.read().unwrap();
            let target: serde_json::Value = serde_json::from_str(&target).unwrap();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
}

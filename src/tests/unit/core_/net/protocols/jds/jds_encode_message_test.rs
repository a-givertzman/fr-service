#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use chrono::{DateTime, Utc};
    use std::sync::{Once, mpsc};
    use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
    use crate::{core_::{net::protocols::jds::{jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, point::{point::{Direction, Point}, point_type::PointType}, status::status::Status, types::bool::Bool}, tcp::steam_read::StreamRead}; 
    
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
    fn test_JdsEncodeMessage() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        println!("test JdsEncodeMessage");
        let name = "/server/line1/ied1/test1";
        let ts = ts();
        let txId = 0;
        // debug!("timestamp: {:?}", ts);j
        let testData = [
            (
                format!(r#"{{"type": "Bool",  "name": "{}", "value": false,   "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Bool(Point::new(txId, name, Bool(false), Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Bool",  "name": "{}", "value": true,    "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Bool(Point::new(txId, name, Bool(true), Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Int",   "name": "{}", "value": 1,   "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Int(Point::new(txId, name, 1, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Int",   "name": "{}", "value": -9223372036854775808,   "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Int(Point::new(txId, name, -9223372036854775808, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Int",   "name": "{}", "value":  9223372036854775807,   "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Int(Point::new(txId, name,  9223372036854775807, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Float", "name": "{}", "value":  0.0, "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name,  0.0, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Float", "name": "{}", "value": -1.1, "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name, -1.1, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Float", "name": "{}", "value":  1.1, "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name,  1.1, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Float", "name": "{}", "value": -1.7976931348623157e308, "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name, -1.7976931348623157e308, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "Float", "name": "{}", "value":  1.7976931348623157e308, "status": 0, "direction": "Read", "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::Float(Point::new(txId, name,  1.7976931348623157e308, Status::Ok, Direction::Read, ts))
            ),
            (
                format!(r#"{{"type": "String","name": "{}", "value": "~!@#$%^&*()_+`1234567890-=","status": 0,"direction": "Read",  "timestamp":"{}"}}"#, 
                name, tsStr(ts)), PointType::String(Point::new(txId, name, "~!@#$%^&*()_+`1234567890-=".to_string(), Status::Ok, Direction::Read, ts))
            ),
        ];
        let (send, recv) = mpsc::channel();
        let mut jdsSerialize = JdsEncodeMessage::new(
            "test",
            JdsSerialize::new("test", recv),
        );
        for (target, point) in testData {
            send.send(point).unwrap();
            let result = jdsSerialize.read().unwrap();
            let value: serde_json::Value = serde_json::from_str(&target).unwrap();
            let mut target = vec![];
            serde_json::to_writer(&mut target, &value).unwrap();
            target.push(4);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
}

#![allow(non_snake_case)]

use std::str::FromStr;

use chrono::{DateTime, Utc};
use log::{trace, warn};
use regex::RegexBuilder;

use crate::core_::types::bool::Bool;

use super::point::Point;

pub trait ToPoint {
    fn toPoint(self, name: &str) -> PointType;
}

impl ToPoint for bool {
    fn toPoint(self, name: &str) -> PointType {
        PointType::Bool(Point::newBool(name, self))
    }
}
impl ToPoint for i64 {
    fn toPoint(self, name: &str) -> PointType {
        PointType::Int(Point::newInt(name, self))
    }
}
impl ToPoint for f64 {
    fn toPoint(self, name: &str) -> PointType {
        PointType::Float(Point::newFloat(name, self))
    }
}
impl ToPoint for &str {
    fn toPoint(self, name: &str) -> PointType {
        PointType::String(Point::newString(name, self))
    }
}
impl ToPoint for String {
    fn toPoint(self, name: &str) -> PointType {
        PointType::String(Point::newString(name, self))
    }
}

///
/// enum container for Point<T>
/// - supported types: Bool, Int, Float
#[derive(Debug, Clone, PartialEq)]
pub enum PointType {
    Bool(Point<Bool>),
    Int(Point<i64>),
    Float(Point<f64>),
    String(Point<String>)
}
///
/// 
impl PointType {
    ///
    /// 
    pub fn new<T: ToPoint>(name: &str, value: T) -> Self {
        value.toPoint(name)
    }
    ///
    /// 
    pub fn fromJsonBytes(bytes: Vec<u8>) -> Result<Self, String> {
        match String::from_utf8(bytes) {
            Ok(jsonString) => {
                match serde_json::from_str(&jsonString) {
                    Ok(value) => {
                        let value: serde_json::Value = value;
                        match value.as_object() {
                            Some(obj) => {
                                match obj.get("type") {
                                    Some(type_) => {
                                        match type_.as_str() {
                                            Some("bool") | Some("Bool") => {
                                                let name = obj.get("name").unwrap().as_str().unwrap();
                                                let value = obj.get("value").unwrap().as_bool().unwrap();
                                                let status = obj.get("status").unwrap().as_i64().unwrap();
                                                let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                                let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                                Ok(PointType::Bool(Point::new(
                                                    name,
                                                    Bool(value),
                                                    status as u8,
                                                    timestamp,
                                                )))
                                            },
                                            Some("int") | Some("Int") => {
                                                let name = obj.get("name").unwrap().as_str().unwrap();
                                                let value = obj.get("value").unwrap().as_i64().unwrap();
                                                let status = obj.get("status").unwrap().as_i64().unwrap();
                                                let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                                let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                                Ok(PointType::Int(Point::new(
                                                    name,
                                                    value,
                                                    status as u8,
                                                    timestamp,
                                                )))
                                            },
                                            Some("float") | Some("Float") => {
                                                let name = obj.get("name").unwrap().as_str().unwrap();
                                                let value = obj.get("value").unwrap().as_f64().unwrap();
                                                let status = obj.get("status").unwrap().as_i64().unwrap();
                                                let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                                let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                                Ok(PointType::Float(Point::new(
                                                    name,
                                                    value,
                                                    status as u8,
                                                    timestamp,
                                                )))
                                            },
                                            Some("string") | Some("String") => {
                                                let name = obj.get("name").unwrap().as_str().unwrap();
                                                let value = obj.get("value").unwrap().as_str().unwrap();
                                                let status = obj.get("status").unwrap().as_i64().unwrap();
                                                let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                                let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                                Ok(PointType::String(Point::new(
                                                    name,
                                                    value.to_owned(),
                                                    status as u8,
                                                    timestamp,
                                                )))
                                            },
                                            _ => {
                                                let message = format!("PointType.fromBytes | Unknown point type: {}", type_);
                                                warn!("{}", message);
                                                Err(message)
                                            }
                                        }
                                    },
                                    None => {
                                        let message = format!("PointType.fromBytes | JSON convertion error: mapping not found in the JSON: {}", value);
                                        warn!("{}", message);
                                        Err(message)        
                                    },
                                }
                            },
                            None => {
                                let message = format!("PointType.fromBytes | JSON convertion error: mapping not found in the JSON: {}", value);
                                warn!("{}", message);
                                Err(message)
                            },
                        }
                    },
                    Err(err) => {
                        let message = format!("PointType.fromBytes | JSON convertion error: {:?}", err);
                        warn!("{}", message);
                        Err(message)        
                    },
                }
                // PointType::
            },
            Err(err) => {
                let message = format!("PointType.fromBytes | From bytes error: {:?}", err);
                warn!("{}", message);
                Err(message)        
            },
        }
    }

    pub fn name(&self) -> String {
        match self {
            PointType::Bool(point) => point.name.clone(),
            PointType::Int(point) => point.name.clone(),
            PointType::Float(point) => point.name.clone(),
            PointType::String(point) => point.name.clone(),
        }
    }
    pub fn asBool(&self) -> Point<Bool> {
        match self {
            PointType::Bool(point) => point.clone(),
            _ => panic!("PointType.asBool | Invalid point type Bool"),
        }
    }
    pub fn asInt(&self) -> Point<i64> {
        match self {
            PointType::Int(point) => point.clone(),
            _ => panic!("PointType.asInt | Invalid point type Int, point: {:?}", &self.name()),
        }
    }
    pub fn asFloat(&self) -> Point<f64> {
        match self {
            PointType::Float(point) => point.clone(),
            _ => panic!("PointType.asFloat | Invalid point type Float"),
        }
    }
    pub fn asString(&self) -> Point<String> {
        match self {
            PointType::String(point) => point.clone(),
            _ => panic!("PointType.asString | Invalid point type String"),
        }
    }
    pub fn status(&self) -> u8 {
        match self {
            PointType::Bool(point) => point.status,
            PointType::Int(point) => point.status,
            PointType::Float(point) => point.status,
            PointType::String(point) => point.status,
        }
    }
    pub fn timestamp(&self) -> DateTime<chrono::Utc> {
        match self {
            PointType::Bool(point) => point.timestamp,
            PointType::Int(point) => point.timestamp,
            PointType::Float(point) => point.timestamp,
            PointType::String(point) => point.timestamp,
        }
    }
}
///
/// 
impl FromStr for PointType {
    type Err = String;
    fn from_str(input: &str) -> Result<PointType, String> {
        trace!("PointType.from_str | input: {}", input);
        let re = r#"(bool|int|float){1}"#;
        let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
        match re.captures(input) {
            Some(caps) => {
                match &caps.get(1) {
                    Some(keyword) => {
                        match keyword.as_str() {
                            "bool"  => Ok( false.toPoint("bool") ),
                            "int"  => Ok( PointType::Int(Point::newInt("int", 0)) ),
                            "float"  => Ok( PointType::Float(Point::newFloat("float", 0.0)) ),
                            "string"  => Ok( PointType::String(Point::newString("string", String::new())) ),
                            _      => Err(format!("Unknown keyword '{}'", input)),
                        }
                    },
                    None => {
                        Err(format!("Unknown keyword '{}'", input))
                    },
                }
            },
            None => {
                Err(format!("Unknown keyword '{}'", input))
            },
        }
    }
}


// impl PartialEq for PointType {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
//             (Self::Int(l0), Self::Int(r0)) => l0 == r0,
//             (Self::Float(l0), Self::Float(r0)) => l0 == r0,
//             _ => false,
//         }
//     }
// }
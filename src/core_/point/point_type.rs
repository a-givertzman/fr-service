#![allow(non_snake_case)]

use chrono::DateTime;

use crate::core_::types::bool::Bool;

use super::point::Point;

pub trait ToPoint {
    fn toPoint(self, txId: usize, name: &str) -> PointType;
}

impl ToPoint for bool {
    fn toPoint(self, txId: usize, name: &str) -> PointType {
        PointType::Bool(Point::newBool(txId, name, self))
    }
}
impl ToPoint for i64 {
    fn toPoint(self, txId: usize, name: &str) -> PointType {
        PointType::Int(Point::newInt(txId, name, self))
    }
}
impl ToPoint for f64 {
    fn toPoint(self, txId: usize, name: &str) -> PointType {
        PointType::Float(Point::newFloat(txId, name, self))
    }
}
impl ToPoint for &str {
    fn toPoint(self, txId: usize, name: &str) -> PointType {
        PointType::String(Point::newString(txId, name, self))
    }
}
impl ToPoint for String {
    fn toPoint(self, txId: usize, name: &str) -> PointType {
        PointType::String(Point::newString(txId, name, self))
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
    pub fn new<T: ToPoint>(txId: usize, name: &str, value: T) -> Self {
        value.toPoint(txId, name)
    }
    ///
    /// Returns transmitter ID of the containing Point
    pub fn txId(&self) -> &usize {
        match self {
            PointType::Bool(point) => &point.txId,
            PointType::Int(point) => &point.txId,
            PointType::Float(point) => &point.txId,
            PointType::String(point) => &point.txId,
        }
    }
    ///
    /// Returns name of the containing Point
    pub fn name(&self) -> String {
        match self {
            PointType::Bool(point) => point.name.clone(),
            PointType::Int(point) => point.name.clone(),
            PointType::Float(point) => point.name.clone(),
            PointType::String(point) => point.name.clone(),
        }
    }
    ///
    /// Returns containing Point<bool>
    pub fn asBool(&self) -> Point<Bool> {
        match self {
            PointType::Bool(point) => point.clone(),
            _ => panic!("PointType.asBool | Invalid point type Bool"),
        }
    }
    ///
    /// Returns containing Point<i64>
    pub fn asInt(&self) -> Point<i64> {
        match self {
            PointType::Int(point) => point.clone(),
            _ => panic!("PointType.asInt | Invalid point type Int, point: {:?}", &self.name()),
        }
    }
    ///
    /// Returns containing Point<f64>
    pub fn asFloat(&self) -> Point<f64> {
        match self {
            PointType::Float(point) => point.clone(),
            _ => panic!("PointType.asFloat | Invalid point type Float"),
        }
    }
    ///
    /// Returns containing Point<String>
    pub fn asString(&self) -> Point<String> {
        match self {
            PointType::String(point) => point.clone(),
            _ => panic!("PointType.asString | Invalid point type String"),
        }
    }
    ///
    /// Returns status of the containing Point
    pub fn status(&self) -> u8 {
        match self {
            PointType::Bool(point) => point.status,
            PointType::Int(point) => point.status,
            PointType::Float(point) => point.status,
            PointType::String(point) => point.status,
        }
    }
    ///
    /// Returns timestamp of the containing Point
    pub fn timestamp(&self) -> DateTime<chrono::Utc> {
        match self {
            PointType::Bool(point) => point.timestamp,
            PointType::Int(point) => point.timestamp,
            PointType::Float(point) => point.timestamp,
            PointType::String(point) => point.timestamp,
        }
    }
    ///
    /// 
    pub fn cmpValue(&self, other: &PointType) -> bool {
        match self {
            PointType::Bool(point) => point.value == other.asBool().value,
            PointType::Int(point) => point.value == other.asInt().value,
            PointType::Float(point) => point.value == other.asFloat().value,
            PointType::String(point) => point.value == other.asString().value,
        }
    }
}
// ///
// /// 
// impl FromStr for PointType {
//     type Err = String;
//     fn from_str(input: &str) -> Result<PointType, String> {
//         trace!("PointType.from_str | input: {}", input);
//         let re = r#"(bool|int|float){1}"#;
//         let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
//         match re.captures(input) {
//             Some(caps) => {
//                 match &caps.get(1) {
//                     Some(keyword) => {
//                         match keyword.as_str() {
//                             "bool"  => Ok( false.toPoint("bool") ),
//                             "int"  => Ok( PointType::Int(Point::newInt("int", 0)) ),
//                             "float"  => Ok( PointType::Float(Point::newFloat("float", 0.0)) ),
//                             "string"  => Ok( PointType::String(Point::newString("string", String::new())) ),
//                             _      => Err(format!("Unknown keyword '{}'", input)),
//                         }
//                     },
//                     None => {
//                         Err(format!("Unknown keyword '{}'", input))
//                     },
//                 }
//             },
//             None => {
//                 Err(format!("Unknown keyword '{}'", input))
//             },
//         }
//     }
// }


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
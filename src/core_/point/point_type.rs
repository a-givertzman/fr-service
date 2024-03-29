use chrono::DateTime;
use concat_string::concat_string;
use serde::{Serialize, ser::SerializeStruct};
use testing::entities::test_value::Value;
use crate::core_::{cot::cot::Cot, status::status::Status, types::bool::Bool};

use super::point::Point;

pub trait ToPoint {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType;
}

impl ToPoint for bool {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType {
        PointType::Bool(Point::new_bool(tx_id, name, *self))
    }
}
impl ToPoint for i64 {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType {
        PointType::Int(Point::new_int(tx_id, name, *self))
    }
}
impl ToPoint for f64 {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType {
        PointType::Float(Point::new_float(tx_id, name, *self))
    }
}
impl ToPoint for &str {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType {
        PointType::String(Point::new_string(tx_id, name, *self))
    }
}
impl ToPoint for String {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType {
        PointType::String(Point::new_string(tx_id, name, self))
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
    pub fn new<T: ToPoint>(tx_id: usize, name: &str, value: T) -> Self {
        value.to_point(tx_id, name)
    }
    ///
    /// Returns transmitter ID of the containing Point
    pub fn tx_id(&self) -> &usize {
        match self {
            PointType::Bool(point) => &point.tx_id,
            PointType::Int(point) => &point.tx_id,
            PointType::Float(point) => &point.tx_id,
            PointType::String(point) => &point.tx_id,
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
    /// Returns destination of the containing Point
    pub fn dest(&self) -> String {
        match self {
            PointType::Bool(point) => concat_string!(point.cot, point.name),
            PointType::Int(point) => concat_string!(point.cot, point.name),
            PointType::Float(point) => concat_string!(point.cot, point.name),
            PointType::String(point) => concat_string!(point.cot, point.name),
        }
    }
    ///
    /// Returns point.value wraped into the enum Value
    pub fn value(&self) -> Value {
        match self {
            PointType::Bool(point) => Value::Bool(point.value.0),
            PointType::Int(point) => Value::Int(point.value),
            PointType::Float(point) => Value::Float(point.value),
            PointType::String(point) => Value::String(point.value.clone()),
        }
    }
    ///
    /// Returns containing Point<bool>
    pub fn as_bool(&self) -> Point<Bool> {
        match self {
            PointType::Bool(point) => point.clone(),
            _ => panic!("PointType.asBool | Invalid point type Bool"),
        }
    }
    ///
    /// Returns containing Point<i64>
    pub fn as_int(&self) -> Point<i64> {
        match self {
            PointType::Int(point) => point.clone(),
            _ => panic!("PointType.asInt | Invalid point type Int, point: {:?}", &self.name()),
        }
    }
    ///
    /// Returns containing Point<f64>
    pub fn as_float(&self) -> Point<f64> {
        match self {
            PointType::Float(point) => point.clone(),
            _ => panic!("PointType.asFloat | Invalid point type Float"),
        }
    }
    ///
    /// Returns containing Point<String>
    pub fn as_string(&self) -> Point<String> {
        match self {
            PointType::String(point) => point.clone(),
            _ => panic!("PointType.asString | Invalid point type String"),
        }
    }
    ///
    /// Returns status of the containing Point
    pub fn status(&self) -> Status {
        match self {
            PointType::Bool(point) => point.status,
            PointType::Int(point) => point.status,
            PointType::Float(point) => point.status,
            PointType::String(point) => point.status,
        }
    }
    ///
    /// Returns the cause & direction of the containing Point
    pub fn cot(&self) -> Cot {
        match self {
            PointType::Bool(point) => point.cot,
            PointType::Int(point) => point.cot,
            PointType::Float(point) => point.cot,
            PointType::String(point) => point.cot,
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
    /// Returns true if other.value == self.value
    pub fn cmp_value(&self, other: &PointType) -> bool {
        match self {
            PointType::Bool(point) => point.value == other.as_bool().value,
            PointType::Int(point) => point.value == other.as_int().value,
            PointType::Float(point) => point.value == other.as_float().value,
            PointType::String(point) => point.value == other.as_string().value,
        }
    }
}
///
///
impl Serialize for PointType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Color", 3)?;
        match self {
            PointType::Bool(point) => {
                state.serialize_field("type", "Bool")?;
                state.serialize_field("value", &point.value.0)?;
                state.serialize_field("name", &point.name)?;
                state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
                state.serialize_field("cot", &point.cot)?;
                state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
            },
            PointType::Int(point) => {
                state.serialize_field("type", "Int")?;
                state.serialize_field("value", &point.value)?;
                state.serialize_field("name", &point.name)?;
                state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
                state.serialize_field("cot", &point.cot)?;
                state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
            },
            PointType::Float(point) => {
                state.serialize_field("type", "Float")?;
                state.serialize_field("value", &point.value)?;
                state.serialize_field("name", &point.name)?;
                state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
                state.serialize_field("cot", &point.cot)?;
                state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
            },
            PointType::String(point) => {
                state.serialize_field("type", "String")?;
                state.serialize_field("value", &point.value)?;
                state.serialize_field("name", &point.name)?;
                state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
                state.serialize_field("cot", &point.cot)?;
                state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
            },
        };
        // trace!("{}.read | json: {:?}", self.id, value);
        state.end()
    }
}
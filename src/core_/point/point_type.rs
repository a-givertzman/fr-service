use std::fmt::Debug;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use testing::entities::test_value::Value;
use crate::{
    conf::point_config::point_config_type::PointConfigType,
    core_::{cot::cot::Cot, status::status::Status, types::bool::Bool},
    services::multi_queue::subscription_criteria::SubscriptionCriteria,
};
use super::point::Point;
///
///
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
impl ToPoint for f32 {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType {
        PointType::Real(Point::new_real(tx_id, name, *self))
    }
}
impl ToPoint for f64 {
    fn to_point(&self, tx_id: usize, name: &str) -> PointType {
        PointType::Double(Point::new_double(tx_id, name, *self))
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
/// - supported types: Bool, Int, Real, Double, String
#[derive(Debug, Clone, PartialEq)]
pub enum PointType {
    Bool(Point<Bool>),
    Int(Point<i64>),
    Real(Point<f32>),
    Double(Point<f64>),
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
            PointType::Real(point) => &point.tx_id,
            PointType::Double(point) => &point.tx_id,
            PointType::String(point) => &point.tx_id,
        }
    }
    ///
    /// Returns name of the containing Point
    pub fn name(&self) -> String {
        match self {
            PointType::Bool(point) => point.name.clone(),
            PointType::Int(point) => point.name.clone(),
            PointType::Real(point) => point.name.clone(),
            PointType::Double(point) => point.name.clone(),
            PointType::String(point) => point.name.clone(),
        }
    }
    ///
    /// Returns destination of the containing Point
    pub fn dest(&self) -> String {
        match self {
            PointType::Bool(point) => SubscriptionCriteria::dest(&point.cot, &point.name),    //concat_string!(point.cot, point.name),
            PointType::Int(point) => SubscriptionCriteria::dest(&point.cot, &point.name),    //concat_string!(point.cot, point.name),
            PointType::Real(point) => SubscriptionCriteria::dest(&point.cot, &point.name),    //concat_string!(point.cot, point.name),
            PointType::Double(point) => SubscriptionCriteria::dest(&point.cot, &point.name),    //concat_string!(point.cot, point.name),
            PointType::String(point) => SubscriptionCriteria::dest(&point.cot, &point.name),    //concat_string!(point.cot, point.name),
        }
    }
    ///
    /// Returns point.value wraped into the enum Value
    pub fn value(&self) -> Value {
        match self {
            PointType::Bool(point) => Value::Bool(point.value.0),
            PointType::Int(point) => Value::Int(point.value),
            PointType::Real(point) => Value::Real(point.value),
            PointType::Double(point) => Value::Double(point.value),
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
            _ => panic!("PointType.as_int | Invalid point type Int, point: {:?}", &self.name()),
        }
    }
    ///
    /// Returns containing Point<f32>
    pub fn as_real(&self) -> Point<f32> {
        match self {
            PointType::Real(point) => point.clone(),
            _ => panic!("PointType.as_real | Invalid point type Real"),
        }
    }
    ///
    /// Returns containing Point<f64>
    pub fn as_double(&self) -> Point<f64> {
        match self {
            PointType::Double(point) => point.clone(),
            _ => panic!("PointType.as_double | Invalid point type Double"),
        }
    }
    ///
    /// Returns containing Point<String>
    pub fn as_string(&self) -> Point<String> {
        match self {
            PointType::String(point) => point.clone(),
            _ => panic!("PointType.as_string | Invalid point type String"),
        }
    }
    ///
    /// Returns status of the containing Point
    pub fn status(&self) -> Status {
        match self {
            PointType::Bool(point) => point.status,
            PointType::Int(point) => point.status,
            PointType::Real(point) => point.status,
            PointType::Double(point) => point.status,
            PointType::String(point) => point.status,
        }
    }
    ///
    /// Returns the cause & direction of the containing Point
    pub fn cot(&self) -> Cot {
        match self {
            PointType::Bool(point) => point.cot,
            PointType::Int(point) => point.cot,
            PointType::Real(point) => point.cot,
            PointType::Double(point) => point.cot,
            PointType::String(point) => point.cot,
        }
    }
    ///
    /// Returns timestamp of the containing Point
    pub fn timestamp(&self) -> DateTime<chrono::Utc> {
        match self {
            PointType::Bool(point) => point.timestamp,
            PointType::Int(point) => point.timestamp,
            PointType::Real(point) => point.timestamp,
            PointType::Double(point) => point.timestamp,
            PointType::String(point) => point.timestamp,
        }
    }
    ///
    /// Returns true if other.value == self.value
    pub fn cmp_value(&self, other: &PointType) -> bool {
        match self {
            PointType::Bool(point) => point.value == other.as_bool().value,
            PointType::Int(point) => point.value == other.as_int().value,
            PointType::Real(point) => point.value == other.as_real().value,
            PointType::Double(point) => point.value == other.as_double().value,
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
        #[derive(Debug, Serialize)]
        struct PointSerialize<'a, T> {
            #[serde(rename = "type")]
            type_: &'a str,
            value: T,
            name: &'a str,
            status: u32,
            cot: Cot,
            timestamp: String,
        }
        match self {
            PointType::Bool(point) => {
                PointSerialize {
                    type_: "Bool",
                    value: if point.value.0 {1} else {0},
                    name: &point.name,
                    status: Into::<u32>::into(point.status),
                    cot: point.cot,
                    timestamp: point.timestamp.to_rfc3339(),
                }.serialize(serializer)
            },
            PointType::Int(point) => {
                PointSerialize {
                    type_: "Int",
                    value: &point.value,
                    name: &point.name,
                    status: Into::<u32>::into(point.status),
                    cot: point.cot,
                    timestamp: point.timestamp.to_rfc3339(),
                }.serialize(serializer)
            },
            PointType::Real(point) => {
                PointSerialize {
                    type_: "Real",
                    value: point.value,
                    name: &point.name,
                    status: Into::<u32>::into(point.status),
                    cot: point.cot,
                    timestamp: point.timestamp.to_rfc3339(),
                }.serialize(serializer)
            },
            PointType::Double(point) => {
                PointSerialize {
                    type_: "Double",
                    value: &point.value,
                    name: &point.name,
                    status: Into::<u32>::into(point.status),
                    cot: point.cot,
                    timestamp: point.timestamp.to_rfc3339(),
                }.serialize(serializer)
            },
            PointType::String(point) => {
                PointSerialize {
                    type_: "String",
                    value: &point.value,
                    name: &point.name,
                    status: Into::<u32>::into(point.status),
                    cot: point.cot,
                    timestamp: point.timestamp.to_rfc3339(),
                }.serialize(serializer)
            },
        }
        // let mut state = serializer.serialize_struct("Point", 6)?;
        // match self {
        //     PointType::Bool(point) => {
        //         state.serialize_field("type", "Bool")?;
        //         state.serialize_field("value", if point.value.0 {&1} else {&0})?;
        //         state.serialize_field("name", &point.name)?;
        //         state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
        //         state.serialize_field("cot", &point.cot)?;
        //         state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
        //     },
        //     PointType::Int(point) => {
        //         state.serialize_field("type", "Int")?;
        //         state.serialize_field("value", &point.value)?;
        //         state.serialize_field("name", &point.name)?;
        //         state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
        //         state.serialize_field("cot", &point.cot)?;
        //         state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
        //     },
        //     PointType::Real(point) => {
        //         state.serialize_field("type", "Real")?;
        //         state.serialize_field("value", &point.value)?;
        //         state.serialize_field("name", &point.name)?;
        //         state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
        //         state.serialize_field("cot", &point.cot)?;
        //         state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
        //     },
        //     PointType::Double(point) => {
        //         state.serialize_field("type", "Double")?;
        //         state.serialize_field("value", &point.value)?;
        //         state.serialize_field("name", &point.name)?;
        //         state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
        //         state.serialize_field("cot", &point.cot)?;
        //         state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
        //     },
        //     PointType::String(point) => {
        //         state.serialize_field("type", "String")?;
        //         state.serialize_field("value", &point.value)?;
        //         state.serialize_field("name", &point.name)?;
        //         state.serialize_field("status", &(Into::<u32>::into( point.status)))?;
        //         state.serialize_field("cot", &point.cot)?;
        //         state.serialize_field("timestamp", &point.timestamp.to_rfc3339())?;
        //     },
        // };
        // trace!("{}.read | json: {:?}", self.id, value);
        // state.end()
    }
}
///
///
impl<'de> Deserialize<'de> for PointType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        #[derive(Debug, Deserialize)]
        struct PointDeserialize {
            #[serde(alias = "type")]
            pub type_: PointConfigType,
            pub value: serde_json::Value,
            pub name: String,
            pub status: i64,  // Status,
            pub cot: Cot,
            pub timestamp: String    //DateTime<chrono::Utc>,
        }
        let tx_id = 0;
        let visitor = PointDeserialize::deserialize(deserializer)?;
        fn value_parsing_error<'de, D>(type_: &str, visitor: &PointDeserialize, err: impl Debug) -> D::Error where D: serde::Deserializer<'de>{
            serde::de::Error::custom(format!("PointType.deserialize | Error parsing {} value from {:#?}, \n\terror: {:#?}", type_, visitor, err))
        }
        fn timestamp_parsing_error<'de, D>(type_: &str, visitor: &PointDeserialize, err: impl Debug) -> D::Error where D: serde::Deserializer<'de>{
            serde::de::Error::custom(format!("PointType.deserialize | Error parsing {} timestamp from {:#?}, \n\terror: {:#?}", type_, visitor, err))
        }
        match visitor.type_ {
            PointConfigType::Bool => {
                let value = visitor.value.as_i64().ok_or_else(|| value_parsing_error::<D>("Point<Bool>", &visitor, "err"))?;
                Ok(PointType::Bool(Point::new(
                    tx_id,
                    &visitor.name,
                    Bool(value > 0),
                    Status::from(visitor.status),
                    visitor.cot,
                    visitor.timestamp.parse().map_err(|err| timestamp_parsing_error::<D>("Point<Bool>", &visitor, err))?,
                )))
            },
            PointConfigType::Int => {
                let value = visitor.value.as_i64().ok_or_else(|| value_parsing_error::<D>("Point<Int>", &visitor, "err"))?;
                Ok(PointType::Int(Point::new(
                    tx_id,
                    &visitor.name,
                    value,
                    Status::from(visitor.status),
                    visitor.cot,
                    visitor.timestamp.parse().map_err(|err| timestamp_parsing_error::<D>("Point<Int>", &visitor, err))?,
                )))
            },
            PointConfigType::Real => {
                let value = visitor.value.as_f64().ok_or_else(|| value_parsing_error::<D>("Point<Real>", &visitor, "err"))?;
                Ok(PointType::Real(Point::new(
                    tx_id,
                    &visitor.name,
                    value as f32,
                    Status::from(visitor.status),
                    visitor.cot,
                    visitor.timestamp.parse().map_err(|err| timestamp_parsing_error::<D>("Point<Real>", &visitor, err))?,
                )))
            },
            PointConfigType::Double => {
                let value = visitor.value.as_f64().ok_or_else(|| value_parsing_error::<D>("Point<Double>", &visitor, "err"))?;
                Ok(PointType::Double(Point::new(
                    tx_id,
                    &visitor.name,
                    value,
                    Status::from(visitor.status),
                    visitor.cot,
                    visitor.timestamp.parse().map_err(|err| timestamp_parsing_error::<D>("Point<Double>", &visitor, err))?,
                )))
            },
            PointConfigType::String => {
                Ok(PointType::String(Point::new(
                    tx_id,
                    &visitor.name,
                    visitor.value.as_str().unwrap().to_owned(),
                    Status::from(visitor.status),
                    visitor.cot,
                    visitor.timestamp.parse().map_err(|err| timestamp_parsing_error::<D>("Point<String>", &visitor, err))?,
                )))
            },
            PointConfigType::Json => {
                Err(serde::de::Error::custom("PointType.deserialize | Error parsing Point<Json> - Not implemented yet"))
                // Ok(PointType::String(Point::new(
                //     tx_id,
                //     &visitor.name,
                //     visitor.value.clone(),
                //     Status::from(visitor.status),
                //     visitor.cot,
                //     visitor.timestamp.parse().map_err(|err| value_parsing_timestamp::<D>("Point<Json>", &visitor, err))?,
                // )))
            },
        }
    }
}
#![allow(non_snake_case)]

use std::sync::mpsc::Receiver;

use chrono::DateTime;
use log::{trace, debug};
use serde::{Serialize, ser::SerializeStruct};
use serde_json::json;

use crate::{core_::point::point_type::PointType, tcp::steam_read::StreamRead};


///
/// Converts PointType into the squence of bytes
/// useng PointType -> Point<type> -> JSON -> bytes conversion
pub struct JdsSerialize {
    id: String,
    stream: Receiver<PointType>,
}
///
/// 
impl JdsSerialize {
    ///
    /// Creates new instance of the JdsSerialize
    pub fn new(parent: impl Into<String>, stream: Receiver<PointType>) -> Self {
        Self {
            id: format!("{}/JdsSerialize", parent.into()),
            stream,
        }
    }
    ///
    /// Serialize point into json string
    fn serialize(&self, point: PointType) -> Result<serde_json::Value, String> {
        let value = match point {
            PointType::Bool(point) => {
                json!(PointSerializable {
                    type_: String::from("Bool"),
                    name: point.name,
                    value: json!(point.value.0),
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
            PointType::Int(point) => {
                json!(PointSerializable {
                    type_: String::from("Int"),
                    name: point.name,
                    value: json!(point.value),
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
            PointType::Float(point) => {
                json!(PointSerializable {
                    type_: String::from("Float"),
                    name: point.name,
                    value: json!(point.value),
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
            PointType::String(point) => {
                json!(PointSerializable {
                    type_: String::from("String"),
                    name: point.name,
                    value: json!(point.value),
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
        };
        debug!("{}.read | json: {:?}", self.id, value);
        Ok(value)
    }    
}
///
/// 
impl StreamRead<serde_json::Value, String> for JdsSerialize {
    ///
    /// Reads single point from Receiver & serialize it into json string
    fn read(&mut self) -> Result<serde_json::Value, String> {
        match self.stream.recv() {
            Ok(point) => {
                trace!("{}.read | point: {:?}", self.id, &point);
                self.serialize(point)
            },
            Err(err) => {
                let message = format!("{}.read | error: {:?}", self.id, err);
                trace!("{:?}", message);
                Err(message)
            },
        }
    }
}
///
/// 
unsafe impl Sync for JdsSerialize {}

struct PointSerializable {
    pub type_: String,
    pub name: String,
    pub value: serde_json::Value,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}
impl Serialize for PointSerializable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("type", &self.type_)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("timestamp", &self.timestamp.to_rfc3339())?;
        state.end()
    }
}
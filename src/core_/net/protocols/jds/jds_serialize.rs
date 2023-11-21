#![allow(non_snake_case)]

use std::{sync::mpsc::Receiver, collections::HashMap};

use chrono::DateTime;
use log::trace;
use serde::{Serialize, ser::SerializeStruct};
use serde_json::json;

use crate::core_::point::point_type::PointType;


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
    /// Reads single point from Receiver & serialize it into json string
    pub fn read(&mut self) -> Result<String, String> {
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
    ///
    /// Serialize point into json string
    fn serialize(&self, point: PointType) -> Result<String, String> {
        let value = match point {
            PointType::Bool(point) => {
                json!(PointSerializable {
                    name: point.name,
                    value: point.value,
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
            PointType::Int(point) => {
                json!(PointSerializable {
                    name: point.name,
                    value: point.value,
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
            PointType::Float(point) => {
                json!(PointSerializable {
                    name: point.name,
                    value: point.value,
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
            PointType::String(point) => {
                json!(PointSerializable {
                    name: point.name,
                    value: point.value,
                    status: point.status,
                    timestamp: point.timestamp,
                })
            },
        };
        match value.as_str() {
            Some(value) => Ok(value.to_owned()),
            None => Err(format!("{}.read | json encoding error", self.id)),
        }
    }    
}


struct PointSerializable<T> {
    pub name: String,
    pub value: T,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}
impl<T> Serialize for PointSerializable<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}
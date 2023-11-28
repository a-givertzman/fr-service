#![allow(non_snake_case)]

use std::{io::BufReader, net::TcpStream};

use chrono::{DateTime, Utc};
use log::{warn, error, trace};

use crate::core_::{net::connection_status::ConnectionStatus, point::{point_type::PointType, point::Point}, types::bool::Bool};

use super::jds_decode_message::JdsDecodeMessage;

///
/// Converts squence of bytes into the PointType
/// useng bytes -> JSON -> Point<type> PointType conversion
pub struct JdsDeserialize {
    id: String,
    stream: JdsDecodeMessage,
}
///
/// 
impl JdsDeserialize {
    ///
    /// Creates new instance of the JdsDeserialize
    pub fn new(parent: impl Into<String>, stream: JdsDecodeMessage) -> Self {
        Self {
            id: format!("{}/JdsDeserialize", parent.into()),
            stream,
        }
    }
    ///
    /// Reads single point from TcpStream
    pub fn read(&mut self, tcpStream: &mut BufReader<TcpStream>) -> ConnectionStatus<Result<PointType, String>, String> {
        match self.stream.read(tcpStream) {
            ConnectionStatus::Active(bytes) => {
                match Self::deserialize(bytes) {
                    Ok(point) => {
                        ConnectionStatus::Active(Ok(point))
                    },
                    Err(err) => {
                        error!("{}", err);
                        ConnectionStatus::Active(Err(err))
                    },
                }
            },
            ConnectionStatus::Closed(err) => {
                ConnectionStatus::Closed(err)
            },
        }
    }
    ///
    /// 
    pub fn deserialize(bytes: Vec<u8>) -> Result<PointType, String> {
        match serde_json::from_slice(&bytes) {
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
                                        let message = format!("JdsDeserialize.parse | Unknown point type: {}", type_);
                                        trace!("{}", message);
                                        Err(message)
                                    }
                                }
                            },
                            None => {
                                let message = format!("JdsDeserialize.parse | JSON convertion error: mapping not found in the JSON: {}", value);
                                trace!("{}", message);
                                Err(message)        
                            },
                        }
                    },
                    None => {
                        let message = format!("JdsDeserialize.parse | JSON convertion error: mapping not found in the JSON: {}", value);
                        trace!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => {
                let message = format!("JdsDeserialize.parse | JSON convertion error: {:?}", err);
                trace!("{}", message);
                Err(message)        
            },
        }
                // PointType::
    }    
}
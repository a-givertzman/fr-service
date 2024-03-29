use std::io::{BufReader, Read};
use chrono::{DateTime, Utc};
use log::{warn, trace, LevelFilter};
use crate::{core_::{
    cot::cot::Cot, net::connection_status::ConnectionStatus, object::object::Object, point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, status::status::Status, types::bool::Bool 
}, tcp::steam_read::TcpStreamRead};
use super::jds_decode_message::JdsDecodeMessage;

///
/// Converts squence of bytes into the PointType
/// useng bytes -> JSON -> Point<type> PointType conversion
pub struct JdsDeserialize {
    id: String,
    tx_id: usize,
    stream: JdsDecodeMessage,
}
///
/// 
impl JdsDeserialize {
    ///
    /// Creates new instance of the JdsDeserialize
    pub fn new(parent: impl Into<String>, stream: JdsDecodeMessage) -> Self {
        let self_id = format!("{}/JdsDeserialize", parent.into());
        Self {
            tx_id: PointTxId::fromStr(&self_id),
            id: self_id,
            stream,
        }
    }
    ///
    /// Reads single point from TcpStream
    pub fn read(&mut self, tcp_stream: impl Read) -> ConnectionStatus<Result<PointType, String>, String> {
        match self.stream.read(tcp_stream) {
            ConnectionStatus::Active(result) => {
                match result {
                    Ok(bytes) => {
                        match Self::deserialize(&self.id, self.tx_id, bytes) {
                            Ok(point) => {
                                ConnectionStatus::Active(Ok(point))
                            },
                            Err(err) => {
                                if log::max_level() == LevelFilter::Debug {
                                    warn!("{}", err);
                                }
                                ConnectionStatus::Active(Err(err))
                            },
                        }
                    },
                    Err(err) => ConnectionStatus::Active(Err(err)),
                }
            },
            ConnectionStatus::Closed(err) => {
                ConnectionStatus::Closed(err)
            },
        }
    }
    ///
    /// Returns Cot parsed from the json::Map by it's key "cot" 
    fn parse_cot(self_id: &str, name: &str, obj: &serde_json::Map<String, serde_json::Value>) -> Cot {
        match obj.get("cot") {
            Some(value) => {
                match serde_json::from_value(value.clone()) {
                    Ok(direction) => direction,
                    Err(err) => {
                        let message = format!("{}.parse | Deserialize Point.direction error: {:?} in the: {}:{:?}", self_id, err, name, value);
                        warn!("{}", message);
                        Cot::default()
                    },
                }
            },
            None => Cot::default(),
        }
    }
    ///
    /// 
    pub fn deserialize(self_id: &str, tx_id: usize, bytes: Vec<u8>) -> Result<PointType, String> {
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
                                        let direction = Self::parse_cot(self_id, name, obj);
                                        let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                        let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                        Ok(PointType::Bool(Point::new(
                                            tx_id,
                                            name,
                                            Bool(value),
                                            Status::from(status),
                                            direction,
                                            timestamp,
                                        )))
                                    },
                                    Some("int") | Some("Int") => {
                                        let name = obj.get("name").unwrap().as_str().unwrap();
                                        let value = obj.get("value").unwrap().as_i64().unwrap();
                                        let status = obj.get("status").unwrap().as_i64().unwrap();
                                        let direction = Self::parse_cot(self_id, name, obj);
                                        let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                        let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                        Ok(PointType::Int(Point::new(
                                            tx_id,
                                            name,
                                            value,
                                            Status::from(status),
                                            direction,
                                            timestamp,
                                        )))
                                    },
                                    Some("float") | Some("Float") => {
                                        let name = obj.get("name").unwrap().as_str().unwrap();
                                        let value = obj.get("value").unwrap().as_f64().unwrap();
                                        let status = obj.get("status").unwrap().as_i64().unwrap();
                                        let direction = Self::parse_cot(self_id, name, obj);
                                        let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                        let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                        Ok(PointType::Float(Point::new(
                                            tx_id,
                                            name,
                                            value,
                                            Status::from(status),
                                            direction,
                                            timestamp,
                                        )))
                                    },
                                    Some("string") | Some("String") => {
                                        let name = obj.get("name").unwrap().as_str().unwrap();
                                        let value = obj.get("value").unwrap().as_str().unwrap();
                                        let status = obj.get("status").unwrap().as_i64().unwrap();
                                        let direction = Self::parse_cot(self_id, name, obj);
                                        let timestamp = obj.get("timestamp").unwrap().as_str().unwrap();
                                        let timestamp: DateTime<Utc> = chrono::DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);
                                        Ok(PointType::String(Point::new(
                                            tx_id,
                                            name,
                                            value.to_owned(),
                                            Status::from(status),
                                            direction,
                                            timestamp,
                                        )))
                                    },
                                    _ => {
                                        let message = format!("{}.parse | Unknown point type: {}", self_id, type_);
                                        trace!("{}", message);
                                        Err(message)
                                    }
                                }
                            },
                            None => {
                                let message = format!("{}.parse | JSON convertion error: mapping not found in the JSON: {}", self_id, value);
                                trace!("{}", message);
                                Err(message)        
                            },
                        }
                    },
                    None => {
                        let message = format!("{}.parse | JSON convertion error: mapping not found in the JSON: {}", self_id, value);
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
    }    
}
impl Object for JdsDeserialize {
    fn id(&self) -> &str {
        &self.id
    }
}
impl TcpStreamRead for JdsDeserialize {
    fn read(&mut self, tcp_stream: &mut BufReader<std::net::TcpStream>) -> ConnectionStatus<Result<PointType, String>, String> {
        self.read(tcp_stream)
    }
}
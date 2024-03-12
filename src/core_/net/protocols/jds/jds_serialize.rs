use std::sync::mpsc::{Receiver, RecvTimeoutError};
use log::trace;
use crate::{
    core_::{constants::constants::RECV_TIMEOUT, failure::recv_error::RecvError, object::object::Object, point::point_type::PointType}, 
    tcp::steam_read::StreamRead,
};

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
    // ///
    // /// Serialize point into json string
    // fn serialize(&self, point: PointType) -> Result<serde_json::Value, serde_json::Error> {
    //     let value = serde_json::to_value(&point);
    //     trace!("{}.read | json: {:?}", self.id, value);
    //     value
    // }    
}
///
/// 
impl Object for JdsSerialize {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl StreamRead<serde_json::Value, RecvError> for JdsSerialize {
    ///
    /// Reads single point from Receiver & serialize it into json string
    fn read(&mut self) -> Result<serde_json::Value, RecvError> {
        match self.stream.recv_timeout(RECV_TIMEOUT) {
            Ok(point) => {
                trace!("{}.read | point: {:?}", self.id, point);
                match serde_json::to_value(&point) {
                    Ok(point) => Ok(point),
                    Err(err) => Err(RecvError::Error(format!("{}.read | Serialize error: {:?}", self.id, err))),
                }
            },
            Err(err) => {
                match err {
                    RecvTimeoutError::Timeout => Err(RecvError::Timeout),
                    RecvTimeoutError::Disconnected => Err(RecvError::Disconnected),
                }
            },
        }
    }
}
///
/// 
unsafe impl Sync for JdsSerialize {}

// struct PointSerializable {
//     pub type_: String,
//     pub name: String,
//     pub value: serde_json::Value,
//     pub status: Status,
//     pub cot: Cot,
//     pub timestamp: DateTime<chrono::Utc>,
// }
// impl Serialize for PointSerializable {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer {
//         let mut state = serializer.serialize_struct("Color", 3)?;
//         state.serialize_field("type", &self.type_)?;
//         state.serialize_field("name", &self.name)?;
//         state.serialize_field("value", &self.value)?;
//         state.serialize_field("status", &(Into::<u32>::into( self.status)))?;
//         state.serialize_field("cot", &json!(self.cot))?;
//         state.serialize_field("timestamp", &self.timestamp.to_rfc3339())?;
//         state.end()
//     }
// }


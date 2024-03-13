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

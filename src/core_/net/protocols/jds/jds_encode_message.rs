use crate::{core_::{failure::recv_error::RecvError, object::object::Object}, tcp::steam_read::StreamRead};
use super::{jds_serialize::JdsSerialize, jds_define::JDS_END_OF_TRANSMISSION};

///
/// Converts json string into the bytes
/// adds Jds.endOfTransmission = 4 at the end of message
/// returns Result<Vec, Err>
pub struct JdsEncodeMessage {
    id: String,
    stream: JdsSerialize,
}
///
/// 
impl JdsEncodeMessage {
    ///
    /// Creates new instance of the JdsEncodeMessage
    pub fn new(parent: impl Into<String>, stream: JdsSerialize) -> Self {
        Self {
            id: format!("{}/JdsMessage", parent.into()),
            stream,
        }
    }
}
///
/// 
impl Object for JdsEncodeMessage {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl StreamRead<Vec<u8>, RecvError> for JdsEncodeMessage {
    ///
    /// Returns sequence of bytes representing encoded single PointType, ends with Jds.endOfTransmission = 4
    fn read(&mut self) -> Result<Vec<u8>, RecvError> {
        let mut bytes = Vec::new();
        match self.stream.read() {
            Ok(value) => {
                match serde_json::to_writer(&mut bytes, &value) {
                    Ok(_) => {
                        bytes.push(JDS_END_OF_TRANSMISSION);
                        Ok(bytes)
                    },
                    Err(err) => Err(RecvError::Error(format!("{}.read | error: {:?}", self.id, err))),
                }
            },
            Err(err) => Err(err),
        }
    }
}
///
/// 
unsafe impl Sync for JdsEncodeMessage {}
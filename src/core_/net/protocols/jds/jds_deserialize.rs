#![allow(non_snake_case)]

use super::jds_message::JdsMessage;

///
/// 
pub struct JdsDeserialize {
    id: String,
    stream: JdsMessage,
}
///
/// 
impl JdsDeserialize {
    ///
    /// Creates new instance of the JdsDeserialize
    pub fn new(parent: impl Into<String>, stream: JdsMessage) -> Self {
        Self {
            id: format!("{}/JdsDeserialize", parent.into()),
            stream,
        }
    }
}
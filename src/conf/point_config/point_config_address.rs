#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

///
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointConfigAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit: Option<u8>,
}
///
/// 
impl PointConfigAddress {
    pub fn empty() -> Self {
        Self { offset: None, bit: None }
    }
}

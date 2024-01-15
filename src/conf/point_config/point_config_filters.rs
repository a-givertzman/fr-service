#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

///
/// Set of the prefilters - executed during parsing data points from the protocol line
///     - [threshold]: float - 0...100% parameter for data points to be filtered
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointConfigFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<u8>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub filterName: Option<u8>,
}
// ///
// /// 
// impl PointConfigAddress {
//     pub fn empty() -> Self {
//         Self { offset: None, bit: None }
//     }
// }

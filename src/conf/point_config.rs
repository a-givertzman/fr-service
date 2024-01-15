#![allow(non_snake_case)]

use std::collections::HashMap;

use log::{trace, debug};
use serde::{Serialize, Deserialize};

use super::conf_tree::ConfTree;

///
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointConfig {
    #[serde(skip)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(alias = "type", alias = "Type")]
    pub _type: PointConfigType,
    pub history: Option<u8>,
    pub alarm: Option<u8>,
    pub address: PointConfigAddress,
    pub comment: Option<String>,
    
}
///
/// 
impl PointConfig {
    ///
    /// creates PointConfig from serde_yaml::Value of following format:
    /// ```yaml
    /// PointName:
    ///     type: bool      # bool / int / float / string / json
    ///     history: 0      # 0 / 1
    ///     alarm: 0        # 0..15
    ///     address:
    ///         offset: 0..65535
    ///         bit: 0..255
    ///     comment: Test Point 
    pub fn new(confTree: &ConfTree) -> PointConfig {
        println!("\n");
        trace!("MetricConfig.new | confTree: {:?}", confTree);
        let mut pc: PointConfig = serde_yaml::from_value(confTree.conf.clone()).unwrap();
        pc.name = confTree.key.clone();
        pc
    }    
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> PointConfig {
        debug!("PointConfig.fromYamlValue | value: {:?}", value);
        Self::new(&ConfTree::newRoot(value.clone()).next().unwrap())
    }
    ///
    /// Returns yaml representation
    pub fn asYaml(&self) -> serde_yaml::Value {
        let result: serde_yaml::Value = serde_yaml::to_value(&self).unwrap();
        let mut wrap = HashMap::new();
        wrap.insert(self.name.clone(), result);
        serde_yaml::to_value(wrap).unwrap()
    }
}

///
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointConfigAddress {
    pub offset: Option<u32>,
    pub bit: Option<u8>,
}
///
/// 
impl PointConfigAddress {
    pub fn empty() -> Self {
        Self { offset: Some(0), bit: Some(0) }
    }
}



///
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointConfigType {
    #[serde(alias = "bool", alias = "Bool")]
    Bool,
    #[serde(alias = "int", alias = "Int")]
    Int,
    #[serde(alias = "float", alias = "Float")]
    Float,
    #[serde(alias = "string", alias = "String")]
    String,
    #[serde(alias = "json", alias = "Json")]
    Json,
}
///
/// 
impl PointConfigType {

}
// ///
// /// 
// impl FromStr for PointType {
//     type Err = String;
//     fn from_str(input: &str) -> Result<PointType, String> {
//         trace!("PointType.from_str | input: {}", input);
//         let re = r#"(bool|int|float){1}"#;
//         let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
//         match re.captures(input) {
//             Some(caps) => {
//                 match &caps.get(1) {
//                     Some(keyword) => {
//                         match keyword.as_str() {
//                             "bool"  => Ok( false.toPoint("bool") ),
//                             "int"  => Ok( PointType::Int(Point::newInt("int", 0)) ),
//                             "float"  => Ok( PointType::Float(Point::newFloat("float", 0.0)) ),
//                             "string"  => Ok( PointType::String(Point::newString("string", String::new())) ),
//                             _      => Err(format!("Unknown keyword '{}'", input)),
//                         }
//                     },
//                     None => {
//                         Err(format!("Unknown keyword '{}'", input))
//                     },
//                 }
//             },
//             None => {
//                 Err(format!("Unknown keyword '{}'", input))
//             },
//         }
//     }
// }


#![allow(non_snake_case)]

use std::collections::HashMap;

use log::{trace, debug};
use serde::{Serialize, Deserialize};

use crate::conf::{
    conf_tree::ConfTree,
    point_config::{
        point_config_type::PointConfigType,
        point_config_address::PointConfigAddress,
        point_config_filters::PointConfigFilters,
    }
};

///
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointConfig {
    #[serde(skip)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(alias = "type", alias = "Type")]
    pub _type: PointConfigType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm: Option<u8>,
    pub address: PointConfigAddress,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PointConfigFilters>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

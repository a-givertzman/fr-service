use std::{collections::HashMap, str::FromStr};

use log::{trace, debug};
use serde::{Serialize, Deserialize};

use crate::conf::{
    conf_tree::ConfTree, fn_conf_keywd::FnConfKeywd, point_config::{
        point_config_address::PointConfigAddress, point_config_filters::PointConfigFilter, point_config_type::PointConfigType, point_name::PointName
    }
};

use super::point_config_history::PointConfigHistory;

///
/// The configuration of the Point
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointConfig {
    #[serde(skip)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(alias = "type", alias = "Type")]
    pub _type: PointConfigType,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_none")]
    pub history: PointConfigHistory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm: Option<u8>,
    pub address: Option<PointConfigAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PointConfigFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    
}
///
/// 
fn is_none<T: Default + PartialEq>(t: &T) -> bool {
    t == &Default::default()
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
    ///     filter:
    ///         threshold: 0.5      // absolute threshold delta
    ///         factor: 1.5         // multiplier for absolute threshold delta - in this case the delta will be accumulated
    ///     comment: Test Point 
    pub fn new(parent: &str, conf_tree: &ConfTree) -> PointConfig {
        // println!("\n");
        trace!("PointConfig.new | confTree: {:?}", conf_tree);
        let mut pc: PointConfig = serde_yaml::from_value(conf_tree.conf.clone()).unwrap();
        let keyword = FnConfKeywd::from_str(&conf_tree.key);
        let name = match keyword {
            Ok(keyword) => keyword.data(),
            Err(_) => conf_tree.key.clone(),
        };
        pc.name = PointName::new(parent, &name).full();
        if let Some(mut filter) = pc.filters.clone() {
            if let Some(factor) = filter.factor {
                if factor == 0.0 {
                    filter.factor = None
                }
            }
        }
        pc
    }    
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(parent: &str, value: &serde_yaml::Value) -> PointConfig {
        debug!("PointConfig.from_yaml | value: {:?}", value);
        Self::new(parent, &ConfTree::newRoot(value.clone()).next().unwrap())
    }
    ///
    /// Returns yaml representation
    pub fn as_yaml(&self) -> serde_yaml::Value {
        let result: serde_yaml::Value = serde_yaml::to_value(&self).unwrap();
        let mut wrap = HashMap::new();
        wrap.insert(self.name.clone(), result);
        serde_yaml::to_value(wrap).unwrap()
    }
}

use log::{trace, debug};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core::nested_function::fn_conf_keywd::FnConfKeywd;

use super::fn_config_type::FnConfigType;

#[derive(Debug, PartialEq)]
pub struct FnConfig {
    pub name: String,
    pub sql: String,
    pub inputs: HashMap<String, FnConfig>,
}
impl MetricConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// metric sqlUpdateMetric:
    /// table: "TableName"
    /// sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
    /// inputs:
    ///    input1: 
    ///       fn functionName:
    ///          ...
    ///    input2:
    ///       metric sqlSelectMetric:
    ///          ...
    pub fn new(conf: &serde_yaml::Value, vars: &mut Vec<String>) -> MetricConfig {
    }
}

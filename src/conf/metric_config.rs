#![allow(non_snake_case)]

use indexmap::IndexMap;
use log::{trace, debug};
use std::{fs, str::FromStr};

use crate::conf::{fn_config::FnConfig, conf_tree::ConfTree, fn_conf_keywd::FnConfKeywd};

use super::{fn_conf_kind::FnConfKind, point_config::point_config::PointConfig};

///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// fn sqlUpdateMetric:
///     table: "TableName"
///     sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
///     initial: 123.456
///     inputs:
///         input1: 
///             fn functionName:
///                 ...
///         input2:
///             fn SqlMetric:
///                 ...
#[derive(Debug, Clone, PartialEq)]
pub struct MetricConfig {
    pub(crate) name: String,
    pub(crate) table: String,
    pub(crate) sql: String,
    pub(crate) initial: f64,
    pub(crate) inputs: IndexMap<String, FnConfKind>,
    pub(crate) vars: Vec<String>,
}
///
/// 
impl MetricConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// fn sqlUpdateMetric:
    ///     table: "TableName"
    ///     sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
    ///     initial: 123.456
    ///     inputs:
    ///         input1: 
    ///             fn functionName:
    ///                 ...
    ///         input2:
    ///             fn SqlMetric:
    ///                 ...
    pub fn new(parent: &str, confTree: &ConfTree, vars: &mut Vec<String>) -> MetricConfig {
        println!("\n");
        trace!("MetricConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.isMapping() {
                debug!("MetricConfig.new | MAPPING VALUE");
                trace!("MetricConfig.new | confTree: {:?}", confTree);
                let selfName = match FnConfKeywd::from_str(&confTree.key) {
                    Ok(selfKeyword) => {
                        selfKeyword.data()
                    },
                    Err(err) => {
                        panic!("MetricConfig.new | Unknown metric name in {:?}\n\tdetales: {:?}", &confTree.key, err)
                    },
                };
                let mut inputs = IndexMap::new();
                match confTree.get("inputs") {
                    Some(inputsNode) => {
                        for inputConf in inputsNode.subNodes().unwrap() {
                            trace!("MetricConfig.new | input conf: {:?}\t|\t{:?}", inputConf.key, inputConf.conf);
                            if inputConf.isMapping() {
                                inputs.insert(
                                    (&inputConf).key.to_string(), 
                                    FnConfig::new(parent, &inputConf.next().unwrap(), vars),
                                );
                            } else {
                                inputs.insert(
                                    (&inputConf).key.to_string(), 
                                    FnConfig::new(parent, &inputConf, vars),
                                );
                            };
                        }
                    },
                    None => {
                        panic!("MetricConfig.new | Metric '{:?}' 'inputs' not found", &confTree.key)
                    },
                }
                MetricConfig {
                    name: selfName,
                    table: (&confTree).asStr("table").unwrap().to_string(),
                    sql: (&confTree).asStr("sql").unwrap().to_string(),
                    initial: (&confTree).asF64("initial").unwrap(),
                    inputs: inputs,
                    vars: vars.clone(),
                }
        } else {
            panic!("MetricConfig.new | Configuration is empty")
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(parent: &str, value: &serde_yaml::Value, vars: &mut Vec<String>) -> MetricConfig {
        Self::new(parent, &ConfTree::newRoot(value.clone()).next().unwrap(), vars)
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(parent: &str, path: &str) -> MetricConfig {
        let mut vars = vec![];
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        MetricConfig::fromYamlValue(parent, &config, &mut vars)
                    },
                    Err(err) => {
                        panic!("MetricConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("MetricConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        vec![]
    }
}

use log::{trace, debug, error};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core_::conf::{fn_config::FnConfig, conf_tree::ConfTree, conf_keywd::ConfKeywd};

///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// metric sqlUpdateMetric:
///     table: "TableName"
///     sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
///     initial: 123.456
///     inputs:
///         input1: 
///             fn functionName:
///                 ...
///         input2:
///             metric sqlSelectMetric:
///                 ...
#[derive(Debug, PartialEq)]
pub struct MetricConfig {
    pub(crate) name: String,
    pub(crate) table: String,
    pub(crate) sql: String,
    pub(crate) initial: f64,
    pub(crate) inputs: HashMap<String, FnConfig>,
    pub(crate) vars: Vec<String>,
}
///
/// 
impl MetricConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// metric sqlUpdateMetric:
    ///     table: "TableName"
    ///     sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
    ///     initial: 123.456
    ///     inputs:
    ///         input1: 
    ///             fn functionName:
    ///                 ...
    ///         input2:
    ///             metric sqlSelectMetric:
    ///                 ...
    pub fn new(confTree: &ConfTree, vars: &mut Vec<String>) -> MetricConfig {
        println!("\n");
        trace!("MetricConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("MetricConfig.new | FnConf must have single item, additional items was ignored")
        };
        if confTree.isMapping() {
                debug!("MetricConfig.new | MAPPING VALUE");
                trace!("MetricConfig.new | confTree: {:?}", confTree);
                let selfName = match ConfKeywd::from_str(&confTree.key) {
                    Ok(selfKeyword) => {
                        selfKeyword.name()
                    },
                    Err(err) => {
                        panic!("MetricConfig.new | Unknown metric name in {:?}\n\tdetales: {:?}", &confTree.key, err)
                    },
                };
                let mut inputs = HashMap::new();
                match confTree.get("inputs") {
                    Some(inputsNode) => {
                        for inputConf in inputsNode.subNodes().unwrap() {
                            trace!("MetricConfig.new | input conf: {:?}\t|\t{:?}", inputConf.key, inputConf.conf);
                            if inputConf.isMapping() {
                                inputs.insert(
                                    (&inputConf).key.to_string(), 
                                    FnConfig::new(&inputConf.next().unwrap(), vars),
                                );
                            } else {
                                inputs.insert(
                                    (&inputConf).key.to_string(), 
                                    FnConfig::new(&inputConf, vars),
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
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value, vars: &mut Vec<String>) -> MetricConfig {
        Self::new(&ConfTree::newRoot(value.clone()).next().unwrap(), vars)
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> MetricConfig {
        let mut vars = vec![];
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        MetricConfig::fromYamlValue(&config, &mut vars)
                    },
                    Err(err) => {
                        panic!("Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("File {} reading error: {:?}", path, err)
            },
        }
    }

}

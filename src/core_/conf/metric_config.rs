use log::{trace, debug, error};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core_::conf::{fn_config::FnConfig, conf_tree::ConfTree};

use strum::{IntoEnumIterator, EnumIter};

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
        match confTree.next() {
            Some(selfConf) => {
                debug!("FnConfig.new | MAPPING VALUE");
                trace!("FnConfig.new | selfConf: {:?}", selfConf);
                let mut inputs = HashMap::new();
                match selfConf.get("inputs") {
                    Some(inputsNode) => {
                        for inputConf in inputsNode.subNodes().unwrap() {
                            trace!("MetricConfig.new | input conf: {:?}\t|\t{:?}", inputConf.key, inputConf.conf);
                            inputs.insert(
                                inputConf.key.to_string(), 
                                FnConfig::fromYamlValue(&inputConf.conf, vars),
                            );
                        }
                    },
                    None => {
                        panic!("MetricConfig.new | Metric '{:?}' 'inputs' not found", &selfConf.key)
                    },
                }
                MetricConfig {
                    name: (&selfConf).key.clone(),
                    table: (&selfConf).asStr("table").unwrap().to_string(),
                    sql: (&selfConf).asStr("sql").unwrap().to_string(),
                    initial: (&selfConf).asF64("initial").unwrap(),
                    inputs: inputs,
                    vars: vars.clone(),
                }
            },
            None => {
                panic!("MetricConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value, vars: &mut Vec<String>) -> MetricConfig {
        Self::new(&ConfTree::new(value.clone()), vars)
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

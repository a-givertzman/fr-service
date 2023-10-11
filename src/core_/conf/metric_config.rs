use log::{trace, debug, error};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core_::conf::{fn_config::FnConfig, conf_tree::ConfTree};

use strum::{IntoEnumIterator, EnumIter};

#[derive(Debug, PartialEq)]
pub struct MetricConfig {
    pub name: String,
    pub table: String,
    pub sql: String,
    pub initial: f64,
    pub inputs: HashMap<String, FnConfig>,
    pub(crate) vars: Vec<String>,
}
impl MetricConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// metric sqlUpdateMetric:
    ///     table: "TableName"
    ///     sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
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
    pub fn fromYamlValue(value: &serde_yaml::Value, vars: &mut Vec<String>) -> MetricConfig {
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


#[derive(Debug, EnumIter)]
enum MetricParams {
    Table(String),
    Name(String),
    Sql(String),
    Inputs(String),
    Initial(String),
}

impl MetricParams {
    pub fn all() -> String {
        let cc: Vec<String> = Self::iter().map(|v| v.name()).collect();
        cc.join(", ")
    }
    pub fn name(&self) -> String {
        match self {
            MetricParams::Name(_) => "name".to_string(),
            MetricParams::Table(_) => "table".to_string(),
            MetricParams::Sql(_) => "sql".to_string(),
            MetricParams::Inputs(_) => "inputs".to_string(),
            MetricParams::Initial(_) => "initial".to_string(),
        }
    }
}


impl FromStr for MetricParams {
    type Err = String;
    fn from_str(input: &str) -> Result<MetricParams, String> {
        trace!("MetricParams.from_str | input: {}", input);
        match input {
            "name"  => Ok( MetricParams::Name( input.to_string() )),
            "table"  => Ok( MetricParams::Table( input.to_string() )),
            "sql"  => Ok( MetricParams::Sql( input.to_string() )),
            "inputs" => Ok( MetricParams::Inputs( input.to_string() )),
            "initial" => Ok( MetricParams::Initial( input.to_string() )),
            _      => Err(format!("Unknown metric parameter name '{}'", input)),
        }
    }
}

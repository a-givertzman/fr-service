use log::{trace, debug, error};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core_::conf::{fn_config::FnConfig, conf_tree::ConfTree};

use strum::{IntoEnumIterator, EnumIter};

#[derive(Debug, PartialEq)]
pub struct MetricConfig {
    pub name: String,
    pub table: String,
    pub sql: String,
    pub inputs: HashMap<String, FnConfig>,
    pub initial: String,
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
                let mut table = String::new();
                let mut sql = String::new();
                let mut initial = String::new();
                let mut inputs = HashMap::new();
                match selfConf.subNodes() {
                    Some(params) => {
                        for conf in params {
                            trace!("MetricConfig.new | param: {:?}\t|\t{:?}", conf.key, conf.conf);
                            match MetricParams::from_str(&conf.key) {
                                Ok(param) => {
                                    match param {
                                        MetricParams::Table(_) => {
                                            table = conf.conf.as_str().unwrap().to_string();
                                        },
                                        MetricParams::Name(_) => {
                                            // name = conf.conf.as_str().unwrap().to_string();
                                        },
                                        MetricParams::Sql(_) => {
                                            sql = conf.conf.as_str().unwrap().to_string();
                                        },
                                        MetricParams::Inputs(_) => {
                                            let node = ConfTree::new(conf.conf);
                                            for inputConf in node.subNodes().unwrap() {
                                                trace!("MetricConfig.new | input conf: {:?}\t|\t{:?}", inputConf.key, inputConf.conf);
                                                inputs.insert(
                                                    inputConf.key.to_string(), 
                                                    FnConfig::fromYamlValue(&inputConf.conf, vars),
                                                );
                                            }
                                        },
                                        MetricParams::Initial(_) => {
                                            initial = conf.conf.as_str().unwrap().to_string();
                                        },
                                    }
                                },
                                Err(err) => {
                                    panic!("MetricConfig.new | {}", err)
                                    // panic!("MetricConfig.new | Metric {:?} parameter missed: '{:?}'", confTree.key, paramName)
                                },
                            }
                        }
                    },
                    None => {
                        panic!("MetricConfig.new | Metric {:?} hasn't params, but must have: '{:?}'", selfConf.key, MetricParams::all())
                    },
                }
                // let mut vars: Vec<String> = vec![];
    
                MetricConfig {
                    name: (&selfConf).key.clone(),
                    table: (&selfConf).asStr("table").unwrap().to_string(),
                    sql: sql,
                    inputs: inputs,
                    vars: vars.clone(),
                    initial: initial,
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

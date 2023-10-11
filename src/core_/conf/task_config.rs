use log::{trace, debug, error};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core_::conf::{metric_config::MetricConfig, fn_config::FnConfig, conf_tree::ConfTree, fn_conf_keywd::FnConfKeywd};


#[derive(Debug, PartialEq)]
enum TaskNode {
    Fn(FnConfig),
    Metric(MetricConfig)
}


///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// task operatingMetric:
///     cycle: 100 ms
///     metrics:
///         metric sqlUpdateMetric:
///             table: "TableName"
///             sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
///             initial: 123.456
///             inputs:
///                 input1:
///                     fn functionName:
///                         ...
///                 input2:
///                     metric sqlSelectMetric:
///                         ...
#[derive(Debug, PartialEq)]
pub struct TaskConfig {
    pub(crate) name: String,
    pub(crate) cycle: i64,
    nodes: HashMap<String, TaskNode>,
    vars: Vec<String>,
}
impl TaskConfig {
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
    pub fn new(confTree: &mut ConfTree) -> TaskConfig {
        println!("\n");
        trace!("TaskConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("TaskConfig.new | FnConf must have single item, additional items was ignored")
        };
        let mut vars = vec![];
        match confTree.next() {
            Some(mut selfConf) => {
                debug!("FnConfig.new | MAPPING VALUE");
                trace!("FnConfig.new | selfConf: {:?}", selfConf);
                let selfName = match FnConfKeywd::from_str(&selfConf.key) {
                    Ok(selfKeyword) => selfKeyword.name(),
                    Err(err) => panic!("TaskConfig.new | Unknown metric name in {:?}\n\tdetales: {:?}", &selfConf.key, err),
                };
                let selfCycle = (&mut selfConf).remove("cycle").unwrap().as_i64().unwrap();

                let mut nodeIndex = 0;
                let mut selfNodes = HashMap::new();
                for selfNodeConf in selfConf.subNodes().unwrap() {
                    match FnConfKeywd::from_str(&selfNodeConf.key) {
                        Ok(selfNodeKeyword) => {
                            nodeIndex += 1;
                            let nodeConf = match selfNodeKeyword {
                                FnConfKeywd::Fn(_) => {
                                    TaskNode::Fn(FnConfig::new(&selfNodeConf, &mut vars))
                                },
                                FnConfKeywd::Var(_) => {
                                    TaskNode::Fn(FnConfig::fromYamlValue(&selfNodeConf.conf, &mut vars))
                                },
                                FnConfKeywd::Const(_) => {
                                    TaskNode::Fn(FnConfig::fromYamlValue(&selfNodeConf.conf, &mut vars))
                                },
                                FnConfKeywd::Metric(_) => {
                                    TaskNode::Metric(MetricConfig::fromYamlValue(&selfNodeConf.conf, &mut vars))
                                },
                                FnConfKeywd::Point(_) => {
                                    panic!("TaskConfig.new | Unknown task keyword 'Point' in {:?}", &selfNodeConf.key)
                                },
                            };
                            selfNodes.insert(
                                format!("{}-{}", selfNodeKeyword.name(), nodeIndex),
                                nodeConf,
                            );
                        },
                        Err(err) => {
                            panic!("TaskConfig.new | Unknown task keyword in {:?}\n\tdetales: {:?}", &selfNodeConf.key, err);
                        },
                    };

                    match selfConf.get("inputs") {
                        Some(inputsNode) => {
                            for inputConf in inputsNode.subNodes().unwrap() {
                                trace!("TaskConfig.new | input conf: {:?}\t|\t{:?}", inputConf.key, inputConf.conf);
                            }
                        },
                        None => {
                            panic!("TaskConfig.new | Metric '{:?}' 'inputs' not found", &selfConf.key)
                        },
                    }
                }
                TaskConfig {
                    name: selfName,
                    cycle: selfCycle,
                    nodes: selfNodes,
                    vars: vars,
                }
            },
            None => {
                panic!("TaskConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> TaskConfig {
        Self::new(&mut ConfTree::new(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> TaskConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        TaskConfig::fromYamlValue(&config)
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

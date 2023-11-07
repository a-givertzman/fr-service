#![allow(non_snake_case)]

use indexmap::IndexMap;
use log::{trace, debug, error};
use std::{fs, str::FromStr, time::Duration};

use crate::core_::conf::{metric_config::MetricConfig, fn_config::FnConfig, conf_tree::ConfTree, conf_keywd::ConfKeywd, conf_duration::{ConfDuration, ConfDurationUnit}};


#[derive(Debug, Clone, PartialEq)]
pub enum TaskConfNode {
    Fn(FnConfig),
    Metric(MetricConfig)
}

impl TaskConfNode {
    pub fn name(&self) -> String {
        match self {
            TaskConfNode::Fn(conf) => conf.name.clone(),
            TaskConfNode::Metric(conf) => conf.name.clone(),
        }
    }
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
#[derive(Debug, PartialEq, Clone)]
pub struct TaskConfig {
    pub(crate) name: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) recvQueue: String,
    pub(crate) nodes: IndexMap<String, FnConfig>,
    pub(crate) vars: Vec<String>,
}
///
/// 
impl TaskConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// task taskName:
    ///     cycle: 100  // ms
    ///     metric sqlUpdateMetric:
    ///         table: "TableName"
    ///         sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
    ///         initial: 123.456
    ///         inputs:
    ///             input1:
    ///                 fn functionName:
    ///                     ...
    ///             input2:
    ///                 metric sqlSelectMetric:
    ///                     ...
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
                debug!("TaskConfig.new | MAPPING VALUE");
                trace!("TaskConfig.new | selfConf: {:?}", selfConf);
                let mut selfNodeNames: Vec<String> = selfConf.subNodes().unwrap().map(|conf| conf.key).collect();
                trace!("TaskConfig.new | selfConf keys: {:?}", selfNodeNames);
                let selfName = match ConfKeywd::from_str(&selfConf.key) {
                    Ok(selfKeyword) => selfKeyword.data(),
                    Err(err) => panic!("TaskConfig.new | Unknown metric name in {:?}\n\tdetales: {:?}", &selfConf.key, err),
                };
                trace!("TaskConfig.new | selfName: {:?}", selfName);
                let selfCycle = match Self::getParam(&mut selfConf, &mut selfNodeNames, "cycle") {
                    Some(value) => {
                        match value.as_str() {
                            Some(value) => {
                                match ConfDuration::from_str(value) {
                                    Ok(confDuration) => {
                                        match confDuration.unit {
                                            ConfDurationUnit::Nanos => Some(Duration::from_nanos(confDuration.value)),
                                            ConfDurationUnit::Micros => Some(Duration::from_micros(confDuration.value)),
                                            ConfDurationUnit::Millis => Some(Duration::from_millis(confDuration.value)),
                                            ConfDurationUnit::Secs => Some(Duration::from_secs(confDuration.value)),
                                            ConfDurationUnit::Mins => Some(Duration::from_secs(confDuration.value)),
                                            ConfDurationUnit::Hours => Some(Duration::from_secs(confDuration.value)),
                                        }
                                    },
                                    Err(err) => panic!("TaskConfig.new | Parse cycle duration '{}' error: {:?}", &value, err),
                                }
                            },
                            None => panic!("TaskConfig.new | Invalid cycle duration format: {:?} \n\tin: {:?}", &value, selfConf),
                        }
                    },
                    None => None,
                };
                trace!("TaskConfig.new | selfCycle: {:?}", selfCycle);
                let selfRecvQueue = Self::getParam(&mut selfConf, &mut selfNodeNames, "recv-queue").unwrap();
                trace!("TaskConfig.new | selfRecvQueue: {:?}", selfRecvQueue);
                let mut nodeIndex = 0;
                let mut selfNodes = IndexMap::new();
                for selfNodeName in selfNodeNames {
                    let selfNodeConf = selfConf.get(&selfNodeName).unwrap();
                    trace!("TaskConfig.new | selfNodeConf: {:?}", selfNodeConf);
                    nodeIndex += 1;
                    let nodeConf = FnConfig::new(&selfNodeConf, &mut vars);
                    selfNodes.insert(
                        format!("{}-{}", nodeConf.name, nodeIndex),
                        nodeConf,
                    );
                }
                TaskConfig {
                    name: selfName,
                    cycle: selfCycle,
                    recvQueue: selfRecvQueue.as_str().unwrap().to_string(),
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
        Self::new(&mut ConfTree::newRoot(value.clone()))
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
    ///
    /// 
    fn getParam(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, name: &str) -> Option<serde_yaml::Value> {
        let index = selfKeys.iter().position(|x| *x == name).unwrap();
        selfKeys.remove(index);
        match selfConf.get(name) {
            Some(confTree) => Some(confTree.conf),
            None => None,
        }
    }
}

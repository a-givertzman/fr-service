#![allow(non_snake_case)]

use indexmap::IndexMap;
use log::{trace, debug, error};
use std::{fs, str::FromStr, time::Duration, net::SocketAddr};

use crate::core_::conf::{metric_config::MetricConfig, fn_config::FnConfig, conf_tree::ConfTree, conf_keywd::ConfKeywd, conf_duration::{ConfDuration, ConfDurationUnit}};


// #[derive(Debug, Clone, PartialEq)]
// pub enum TaskConfNode {
//     Fn(FnConfig),
//     Metric(MetricConfig)
// }

// impl TaskConfNode {
//     pub fn name(&self) -> String {
//         match self {
//             TaskConfNode::Fn(conf) => conf.name.clone(),
//             TaskConfNode::Metric(conf) => conf.name.clone(),
//         }
//     }
// }

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
pub struct ApiClientConfig {
    pub(crate) name: String,
    pub(crate) address: SocketAddr,
    pub(crate) cycle: Option<Duration>,
    pub(crate) recvQueue: String,
    pub(crate) nodes: IndexMap<String, FnConfig>,
    pub(crate) vars: Vec<String>,
}
///
/// 
impl ApiClientConfig {
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
    pub fn new(confTree: &mut ConfTree) -> ApiClientConfig {
        println!("\n");
        trace!("ApiClientConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("ApiClientConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        };
        let mut vars = vec![];
        match confTree.next() {
            Some(mut selfConf) => {
                debug!("ApiClientConfig.new | MAPPING VALUE");
                trace!("ApiClientConfig.new | selfConf: {:?}", selfConf);
                let mut selfNodeNames: Vec<String> = selfConf.subNodes().unwrap().map(|conf| conf.key).collect();
                trace!("ApiClientConfig.new | selfConf keys: {:?}", selfNodeNames);
                let selfName = match ConfKeywd::from_str(&selfConf.key) {
                    Ok(selfKeyword) => selfKeyword.data(),
                    Err(err) => panic!("ApiClientConfig.new | Unknown keyword in {:?}\n\tdetales: {:?}", &selfConf.key, err),
                };
                trace!("ApiClientConfig.new | selfName: {:?}", selfName);
                let selfAddress = Self::getParam(&mut selfConf, &mut selfNodeNames, "address").unwrap();
                let selfAddress = selfAddress.as_str().unwrap().parse().unwrap();
                debug!("ApiClientConfig.new | selfAddress: {:?}", selfAddress);
                let selfCycle = match Self::getParam(&mut selfConf, &mut selfNodeNames, "cycle") {
                    Ok(value) => {
                        match value.as_str() {
                            Some(value) => {
                                match ConfDuration::from_str(value) {
                                    Ok(confDuration) => {
                                        Some(confDuration.toDuration())
                                    },
                                    Err(err) => panic!("ApiClientConfig.new | Parse cycle duration '{}' error: {:?}", &value, err),
                                }
                            },
                            None => panic!("ApiClientConfig.new | Invalid cycle duration format: {:?} \n\tin: {:?}", &value, selfConf),
                        }
                    },
                    Err(_) => None,
                };
                trace!("ApiClientConfig.new | selfCycle: {:?}", selfCycle);
                let selfRecvQueue = Self::getParam(&mut selfConf, &mut selfNodeNames, "recv-queue").unwrap();
                trace!("ApiClientConfig.new | selfRecvQueue: {:?}", selfRecvQueue);
                let mut nodeIndex = 0;
                let mut selfNodes = IndexMap::new();
                for selfNodeName in selfNodeNames {
                    let selfNodeConf = selfConf.get(&selfNodeName).unwrap();
                    trace!("ApiClientConfig.new | selfNodeConf: {:?}", selfNodeConf);
                    nodeIndex += 1;
                    let nodeConf = FnConfig::new(&selfNodeConf, &mut vars);
                    selfNodes.insert(
                        format!("{}-{}", nodeConf.name, nodeIndex),
                        nodeConf,
                    );
                }
                ApiClientConfig {
                    name: selfName,
                    address: selfAddress,
                    cycle: selfCycle,
                    recvQueue: selfRecvQueue.as_str().unwrap().to_string(),
                    nodes: selfNodes,
                    vars: vars,
                }
            },
            None => {
                panic!("ApiClientConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> ApiClientConfig {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> ApiClientConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        ApiClientConfig::fromYamlValue(&config)
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
    fn getParam(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, name: &str) -> Result<serde_yaml::Value, String> {
        match selfKeys.iter().position(|x| *x == name) {
            Some(index) => {
                selfKeys.remove(index);
                match selfConf.get(name) {
                    Some(confTree) => Ok(confTree.conf),
                    None => Err(format!("ApiClientConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
                }
            },
            None => Err(format!("ApiClientConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
        }
    }
}

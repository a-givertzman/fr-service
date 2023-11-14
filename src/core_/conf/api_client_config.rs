#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, str::FromStr, time::Duration, net::SocketAddr};

use crate::core_::conf::{conf_tree::ConfTree, conf_duration::ConfDuration, conf_keywd::ConfKeywd};

use super::conf_keywd::ConfKind;


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
    pub(crate) reconnectCycle: Option<Duration>,
    pub(crate) recvQueue: String,
    pub(crate) recvQueueMaxLength: i64,
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
        match confTree.next() {
            Some(mut selfConf) => {
                debug!("ApiClientConfig.new | MAPPING VALUE");
                trace!("ApiClientConfig.new | selfConf: {:?}", selfConf);
                let mut selfNodeNames: Vec<String> = selfConf.subNodes().unwrap().map(|conf| conf.key).collect();
                trace!("ApiClientConfig.new | selfConf keys: {:?}", selfNodeNames);
                let selfName = match ConfKeywd::from_str(&selfConf.key) {
                    Ok(selfKeyword) => selfKeyword.name(),
                    Err(err) => panic!("ApiClientConfig.new | Unknown keyword in {:?}\n\tdetales: {:?}", &selfConf.key, err),
                };
                trace!("ApiClientConfig.new | selfName: {:?}", selfName);
                let selfAddress = Self::getParam(&mut selfConf, &mut selfNodeNames, "address").unwrap();
                let selfAddress = selfAddress.as_str().unwrap().parse().unwrap();
                debug!("ApiClientConfig.new | selfAddress: {:?}", selfAddress);
                let selfCycle = Self::getDuration(&mut selfConf, &mut selfNodeNames, "cycle");
                let selfReconnectCycle = Self::getDuration(&mut selfConf, &mut selfNodeNames, "reconnect");
                debug!("ApiClientConfig.new | selfCycle: {:?}", selfCycle);
                let (selfRecvQueue, selfRecvQueueMaxLength) = match Self::getParamByKeyword(&mut selfConf, &mut selfNodeNames, "in", ConfKind::Queue) {
                    Some((keyword, mut selfRecvQueue)) => {
                        let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                        debug!("ApiClientConfig.new | self in-queue params {}: {:?}", name, selfRecvQueue);
                        // let mut params = Self::getParam(&mut selfConf, &mut selfNodeNames, &name).unwrap();
                        let maxLength = Self::getParam(&mut selfRecvQueue, &mut vec![String::from("max-length")], "max-length").unwrap().as_i64().unwrap();
                        (keyword.name(), maxLength)
                    },
                    None => panic!("ApiClientConfig.new | in queue - not found in : {:?}", selfConf),
                };
                debug!("ApiClientConfig.new | selfRecvQueue: {},\tmax-length: {}", selfRecvQueue, selfRecvQueueMaxLength);
                ApiClientConfig {
                    name: selfName,
                    address: selfAddress,
                    cycle: selfCycle,
                    reconnectCycle: selfReconnectCycle,
                    recvQueue: selfRecvQueue,
                    recvQueueMaxLength: selfRecvQueueMaxLength,
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
    fn getDuration(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, name: &str) -> Option<Duration> {
        match Self::getParam(selfConf, selfKeys, name) {
            Ok(value) => {
                match value.as_str() {
                    Some(value) => {
                        match ConfDuration::from_str(value) {
                            Ok(confDuration) => {
                                Some(confDuration.toDuration())
                            },
                            Err(err) => panic!("ApiClientConfig.new | Parse {} duration '{}' error: {:?}", &name, &value, err),
                        }
                    },
                    None => panic!("ApiClientConfig.new | Invalid reconnect {} duration format: {:?} \n\tin: {:?}", &name, &value, selfConf),
                }
            },
            Err(_) => None,
        }
    }
    ///
    /// 
    fn getParamByKeyword(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, keywordPrefix: &str, keywordKind: ConfKind) -> Option<(ConfKeywd, ConfTree)> {
        // let mut map = HashMap::new();
        for node in selfConf.subNodes().unwrap() {
            match ConfKeywd::from_str(&node.key) {
                Ok(keyword) => {
                    if keyword.kind() == keywordKind && keyword.prefix() == keywordPrefix {
                        return Some((keyword, node));
                    }
                },
                Err(_) => {},
            }
        }
        None
        // match selfKeys.iter().position(|x| *x == name) {
        //     Some(index) => {
        //         selfKeys.remove(index);
        //         match selfConf.get(name) {
        //             Some(confTree) => Ok(confTree.conf),
        //             None => Err(format!("ApiClientConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
        //         }
        //     },
        //     None => Err(format!("ApiClientConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
        // }
    }
}

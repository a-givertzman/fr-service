#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, str::FromStr, time::Duration, net::SocketAddr};

use crate::conf::{conf_tree::ConfTree, conf_duration::ConfDuration, conf_keywd::ConfKeywd};

use super::conf_keywd::ConfKind;


///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service TcpClient:
///     cycle: 1 ms
///     reconnect: 1 s  # default 3 s
///     address: 127.0.0.1:8080
///     in queue link:
///         max-length: 10000
///     out queue: MultiQueue.queue
///                         ...
#[derive(Debug, PartialEq, Clone)]
pub struct MultiQueueConfig {
    pub(crate) name: String,
    pub(crate) address: SocketAddr,
    pub(crate) cycle: Option<Duration>,
    pub(crate) reconnectCycle: Option<Duration>,
    pub(crate) recvQueue: String,
    pub(crate) recvQueueMaxLength: i64,
    pub(crate) sendQueue: String,
}
///
/// 
impl MultiQueueConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service TcpClient:
    ///     cycle: 1 ms
    ///     reconnect: 1 s  # default 3 s
    ///     address: 127.0.0.1:8080
    ///     in queue link:
    ///         max-length: 10000
    ///     out queue: MultiQueue.queue
    ///                     ...
    pub fn new(confTree: &mut ConfTree) -> MultiQueueConfig {
        println!("\n");
        trace!("MultiQueueConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("MultiQueueConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        };
        match confTree.next() {
            Some(mut selfConf) => {
                debug!("MultiQueueConfig.new | MAPPING VALUE");
                trace!("MultiQueueConfig.new | selfConf: {:?}", selfConf);
                let mut selfNodeNames: Vec<String> = selfConf.subNodes().unwrap().map(|conf| conf.key).collect();
                trace!("MultiQueueConfig.new | selfConf keys: {:?}", selfNodeNames);
                let selfName = match ConfKeywd::from_str(&selfConf.key) {
                    Ok(selfKeyword) => selfKeyword.name(),
                    Err(err) => panic!("MultiQueueConfig.new | Unknown keyword in {:?}\n\tdetales: {:?}", &selfConf.key, err),
                };
                trace!("MultiQueueConfig.new | selfName: {:?}", selfName);
                let selfAddress = Self::getParam(&mut selfConf, &mut selfNodeNames, "address").unwrap();
                let selfAddress = selfAddress.as_str().unwrap().parse().unwrap();
                debug!("MultiQueueConfig.new | selfAddress: {:?}", selfAddress);
                let selfCycle = Self::getDuration(&mut selfConf, &mut selfNodeNames, "cycle");
                let selfReconnectCycle = Self::getDuration(&mut selfConf, &mut selfNodeNames, "reconnect");
                debug!("MultiQueueConfig.new | selfCycle: {:?}", selfCycle);
                let (selfRecvQueue, selfRecvQueueMaxLength) = match Self::getParamByKeyword(&mut selfConf, &mut selfNodeNames, "in", ConfKind::Queue) {
                    Some((keyword, mut selfRecvQueue)) => {
                        let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                        debug!("MultiQueueConfig.new | self in-queue params {}: {:?}", name, selfRecvQueue);
                        let maxLength = Self::getParam(&mut selfRecvQueue, &mut vec![String::from("max-length")], "max-length").unwrap().as_i64().unwrap();
                        (keyword.name(), maxLength)
                    },
                    None => panic!("MultiQueueConfig.new | in queue - not found in : {:?}", selfConf),
                };
                let selfSendQueue = match Self::getParamByKeyword(&mut selfConf, &mut selfNodeNames, "out", ConfKind::Queue) {
                    Some((keyword, selfRecvQueue)) => {
                        let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                        debug!("MultiQueueConfig.new | self out-queue param {}: {:?}", name, selfRecvQueue);
                        selfRecvQueue.conf.as_str().unwrap().to_owned()
                    },
                    None => panic!("MultiQueueConfig.new | in queue - not found in : {:?}", selfConf),
                };
                debug!("MultiQueueConfig.new | selfRecvQueue: {},\tmax-length: {}", selfRecvQueue, selfRecvQueueMaxLength);
                MultiQueueConfig {
                    name: selfName,
                    address: selfAddress,
                    cycle: selfCycle,
                    reconnectCycle: selfReconnectCycle,
                    recvQueue: selfRecvQueue,
                    recvQueueMaxLength: selfRecvQueueMaxLength,
                    sendQueue: selfSendQueue,
                }
            },
            None => {
                panic!("MultiQueueConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> MultiQueueConfig {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> MultiQueueConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        MultiQueueConfig::fromYamlValue(&config)
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
                    None => Err(format!("MultiQueueConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
                }
            },
            None => Err(format!("MultiQueueConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
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
                            Err(err) => panic!("MultiQueueConfig.new | Parse {} duration '{}' error: {:?}", &name, &value, err),
                        }
                    },
                    None => panic!("MultiQueueConfig.new | Invalid reconnect {} duration format: {:?} \n\tin: {:?}", &name, &value, selfConf),
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
        //             None => Err(format!("MultiQueueConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
        //         }
        //     },
        //     None => Err(format!("MultiQueueConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
        // }
    }
}

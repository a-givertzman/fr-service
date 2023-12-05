#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, str::FromStr, time::Duration, net::SocketAddr};

use crate::conf::{conf_tree::ConfTree, conf_duration::ConfDuration, conf_keywd::ConfKeywd};

use super::conf_keywd::ConfKind;


///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service ApiClient:
///     cycle: 1 ms
///     reconnect: 1 s  # default 3 s
///     address: 127.0.0.1:8080
///     in queue api-link:
///         max-length: 10000
///     out queue: MultiQueue.queue
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
    /// service ApiClient:
    ///     cycle: 1 ms
    ///     reconnect: 1 s  # default 3 s
    ///     address: 127.0.0.1:8080
    ///     in queue api-link:
    ///         max-length: 10000
    ///     out queue: MultiQueue.queue
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
            Some(selfConf) => {
                debug!("ApiClientConfig.new | MAPPING VALUE");
                let mut selfConf = ServiceConfig::new(&format!("ApiClientConfig({})", selfConf.key), selfConf);
                trace!("ApiClientConfig.new | selfConf: {:?}", selfConf);
                // let mut selfNodeNames: Vec<String> = selfConf.subNodes().unwrap().map(|conf| conf.key).collect();
                // trace!("ApiClientConfig.new | selfConf keys: {:?}", selfNodeNames);
                let selfName = match ConfKeywd::from_str(&selfConf.key) {
                    Ok(selfKeyword) => selfKeyword.name(),
                    Err(err) => panic!("ApiClientConfig.new | Unknown keyword in {:?}\n\tdetales: {:?}", &selfConf.key, err),
                };
                trace!("ApiClientConfig.new | selfName: {:?}", selfName);
                // let selfAddress = Self::getParam(&mut selfConf, &mut selfNodeNames, "address").unwrap();
                let selfAddress: SocketAddr = selfConf.getParam("address").unwrap().as_str().unwrap().parse().unwrap();
                debug!("ApiClientConfig.new | selfAddress: {:?}", selfAddress);
                // let selfCycle = Self::getDuration(&mut selfConf, &mut selfNodeNames, "cycle");
                let selfCycle = selfConf.getDuration("cycle");
                // let selfReconnectCycle = Self::getDuration(&mut selfConf, &mut selfNodeNames, "reconnect");
                let selfReconnectCycle = selfConf.getDuration("reconnect");
                debug!("ApiClientConfig.new | selfCycle: {:?}", selfCycle);
                // let (selfRecvQueue, selfRecvQueueMaxLength) = match Self::getParamByKeyword(&mut selfConf, &mut selfNodeNames, "in", ConfKind::Queue) {
                //     Some((keyword, mut selfRecvQueue)) => {
                //         let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                //         debug!("ApiClientConfig.new | self in-queue params {}: {:?}", name, selfRecvQueue);
                //         // let mut params = Self::getParam(&mut selfConf, &mut selfNodeNames, &name).unwrap();
                //         let maxLength = Self::getParam(&mut selfRecvQueue, &mut vec![String::from("max-length")], "max-length").unwrap().as_i64().unwrap();
                //         (keyword.name(), maxLength)
                //     },
                //     None => panic!("ApiClientConfig.new | in queue - not found in : {:?}", selfConf),
                // };
                let (selfRecvQueue, selfRecvQueueMaxLength) = selfConf.getInQueue();
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
    }
}


///
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ServiceConfig {
    id: String,
    pub key: String,
    conf: ConfTree,
    pub keys: Vec<String>,
}
///
/// 
impl ServiceConfig {
    ///
    /// Creates new instance of ServiceConfig
    pub fn new(parent: &str, conf: ConfTree) -> Self {
        let keys = conf.subNodes().unwrap().map(|conf| conf.key).collect();
        Self { 
            id: format!("{}/ServiceConfig", parent),
            key: conf.key.clone(),
            conf: conf,
            keys,
        }
    }
    ///
    /// returns first sub node
    pub fn first(&self) -> Option<ConfTree> {
        self.conf.next()
    }
    ///
    /// 
    pub fn get(&self, key: &str) -> Option<ConfTree> {
        self.conf.get(key)
    }
    ///
    /// 
    fn removeKey(&mut self, name: &str) -> Result<(), String> {
        match self.keys.iter().position(|x| *x == name) {
            Some(index) => {
                self.keys.remove(index);
                Ok(())
            },
            None => Err(format!("{}.removeKey | '{}' - not found in: {:?}", self.id, name, self.conf)),
        }
    }
    ///
    /// 
    pub fn getParam(&mut self, name: &str) -> Result<serde_yaml::Value, String> {
        match self.removeKey(name) {
            Ok(_) => {
                match self.conf.get(name) {
                    Some(confTree) => Ok(confTree.conf),
                    None => Err(format!("{}.getParam | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }
            },
            Err(err) => Err(err),
        }
    }
    ///
    /// 
    pub fn getDuration(&mut self, name: &str) -> Option<Duration> {
        match self.getParam(name) {
            Ok(value) => {
                match value.as_str() {
                    Some(value) => {
                        match ConfDuration::from_str(value) {
                            Ok(confDuration) => {
                                Some(confDuration.toDuration())
                            },
                            Err(err) => panic!("{}.getDuration | Parse {} duration '{}' error: {:?}", self.id, &name, &value, err),
                        }
                    },
                    None => panic!("{}.getDuration | Invalid reconnect {} duration format: {:?} \n\tin: {:?}", self.id, &name, &value, self.conf),
                }
            },
            Err(_) => None,
        }
    }
    ///
    /// 
    pub fn getParamByKeyword(&mut self, keywordPrefix: &str, keywordKind: ConfKind) -> Result<(ConfKeywd, ConfTree), String> {
        let selfConf = self.conf.clone();
        for node in selfConf.subNodes().unwrap() {
            match ConfKeywd::from_str(&node.key) {
                Ok(keyword) => {
                    if keyword.kind() == keywordKind && keyword.prefix() == keywordPrefix {
                        match self.removeKey(&node.key) {
                            Ok(_) => return Ok((keyword, node)),
                            Err(err) => return Err(err),
                        };
                    }
                },
                Err(_) => {},
            };
        };
        Err(format!("{}.getParamByKeyword | keyword '{} {:?}' - not found", self.id, keywordPrefix, keywordKind))
    }
    ///
    /// 
    pub fn getInQueue(&mut self) -> (String, i64) {
        match self.getParamByKeyword("in", ConfKind::Queue) {
            Ok((keyword, selfRecvQueue)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                debug!("ApiClientConfig.new | self in-queue params {}: {:?}", name, selfRecvQueue);
                let maxLength = match selfRecvQueue.get("max-length") {
                    Some(confTree) => Ok(confTree.conf),
                    None => Err(format!("ServiceConfig.getParam | '{}' - not found in: {:?}", name, self.conf)),
                }.unwrap().as_i64().unwrap();
                (keyword.name(), maxLength)
            },
            Err(err) => panic!("ApiClientConfig.new | in queue - not found in : {:?}\n\terror: {:?}", self.conf, err),
        }        
    }    
}
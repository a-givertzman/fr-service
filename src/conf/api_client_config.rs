#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, str::FromStr, time::Duration, net::SocketAddr};

use crate::conf::{conf_tree::ConfTree, conf_duration::ConfDuration, conf_keywd::ConfKeywd, service_config::ServiceConfig};

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
                let selfId = format!("ApiClientConfig({})", selfConf.key);
                trace!("{}.new | MAPPING VALUE", selfId);
                let mut selfConf = ServiceConfig::new(&selfId, selfConf);
                trace!("{}.new | selfConf: {:?}", selfId, selfConf);
                let selfName = selfConf.name();
                debug!("{}.new | selfName: {:?}", selfId, selfName);
                let selfAddress: SocketAddr = selfConf.getParam("address").unwrap().as_str().unwrap().parse().unwrap();
                debug!("{}.new | selfAddress: {:?}", selfId, selfAddress);
                let selfCycle = selfConf.getDuration("cycle");
                debug!("{}.new | selfCycle: {:?}", selfId, selfCycle);
                let selfReconnectCycle = selfConf.getDuration("reconnect");
                debug!("{}.new | selfReconnectCycle: {:?}", selfId, selfReconnectCycle);
                let (selfRecvQueue, selfRecvQueueMaxLength) = selfConf.getInQueue().unwrap();
                debug!("{}.new | selfRecvQueue: {},\tmax-length: {}", selfId, selfRecvQueue, selfRecvQueueMaxLength);
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
}

















    // ///
    // /// 
    // fn getParam(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, name: &str) -> Result<serde_yaml::Value, String> {
    //     match selfKeys.iter().position(|x| *x == name) {
    //         Some(index) => {
    //             selfKeys.remove(index);
    //             match selfConf.get(name) {
    //                 Some(confTree) => Ok(confTree.conf),
    //                 None => Err(format!("ApiClientConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
    //             }
    //         },
    //         None => Err(format!("ApiClientConfig.getParam | '{}' - not found in: {:?}", name, selfConf)),
    //     }
    // }
    // fn getDuration(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, name: &str) -> Option<Duration> {
    //     match Self::getParam(selfConf, selfKeys, name) {
    //         Ok(value) => {
    //             match value.as_str() {
    //                 Some(value) => {
    //                     match ConfDuration::from_str(value) {
    //                         Ok(confDuration) => {
    //                             Some(confDuration.toDuration())
    //                         },
    //                         Err(err) => panic!("ApiClientConfig.new | Parse {} duration '{}' error: {:?}", &name, &value, err),
    //                     }
    //                 },
    //                 None => panic!("ApiClientConfig.new | Invalid reconnect {} duration format: {:?} \n\tin: {:?}", &name, &value, selfConf),
    //             }
    //         },
    //         Err(_) => None,
    //     }
    // }
    // ///
    // /// 
    // fn getParamByKeyword(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, keywordPrefix: &str, keywordKind: ConfKind) -> Option<(ConfKeywd, ConfTree)> {
    //     // let mut map = HashMap::new();
    //     for node in selfConf.subNodes().unwrap() {
    //         match ConfKeywd::from_str(&node.key) {
    //             Ok(keyword) => {
    //                 if keyword.kind() == keywordKind && keyword.prefix() == keywordPrefix {
    //                     return Some((keyword, node));
    //                 }
    //             },
    //             Err(_) => {},
    //         }
    //     }
    //     None
    // }
#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::fs;

use crate::conf::{conf_tree::ConfTree, service_config::ServiceConfig};

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
    pub(crate) rx: String,
    pub(crate) rxMaxLength: i64,
    pub(crate) tx: Vec<String>,
}
///
/// 
impl MultiQueueConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service MultiQueue:
    ///     in queue in-queue:
    ///         max-length: 10000
    ///     out queue:
    ///         - Service0.in-queue
    ///         - Service1.in-queue
    ///         ...
    ///         - ServiceN.in-queue
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
            Some(selfConf) => {
                let selfId = format!("MultiQueueConfig({})", selfConf.key);
                trace!("{}.new | MAPPING VALUE", selfId);
                let mut selfConf = ServiceConfig::new(&selfId, selfConf);
                trace!("{}.new | selfConf: {:?}", selfId, selfConf);
                let selfName = selfConf.name();
                debug!("{}.new | selfName: {:?}", selfId, selfName);
                let (rx, rxMaxLength) = selfConf.getInQueue().unwrap();
                debug!("{}.new | RX: {},\tmax-length: {}", selfId, rx, rxMaxLength);
                let tx = match selfConf.getParamByKeyword("out", ConfKind::Queue) {
                    Ok((keyword, queueConf)) => {
                        let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                        trace!("{}.new | self tx-queue param {}: {:?}", selfId, name, queueConf);
                        let queues: Vec<String> = match queueConf.conf.as_sequence() {
                            Some(queues) => {
                                queues.iter().map(|value| {
                                    value.as_str().unwrap().to_owned()
                                }).collect()
                        },
                            None => vec![],
                        };
                        queues
                    },
                    Err(err) => panic!("{}.new | out queue - not found in : {:?}\n\terror: {:?}", selfId, selfConf, err),
                };
                debug!("{}.new | TX: {:?}", selfId, tx);
                MultiQueueConfig {
                    name: selfName,
                    rx,
                    rxMaxLength: rxMaxLength,
                    tx,
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
                        panic!("MultiQueueConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("MultiQueueConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
}

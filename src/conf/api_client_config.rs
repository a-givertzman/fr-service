#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, time::Duration, net::SocketAddr};

use crate::conf::{conf_tree::ConfTree, service_config::ServiceConfig};


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
    pub(crate) rx: String,
    pub(crate) rxMaxLength: i64,
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
    pub fn new(confTree: &mut ConfTree) -> Self {
        println!("\n");
        trace!("ApiClientConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("ApiClientConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        };
        match confTree.next() {
            Some(selfConf) => {
                let self_id = format!("ApiClientConfig({})", selfConf.key);
                trace!("{}.new | MAPPING VALUE", self_id);
                let mut selfConf = ServiceConfig::new(&self_id, selfConf);
                trace!("{}.new | selfConf: {:?}", self_id, selfConf);
                let selfName = selfConf.name();
                debug!("{}.new | name: {:?}", self_id, selfName);
                let address: SocketAddr = selfConf.getParamValue("address").unwrap().as_str().unwrap().parse().unwrap();
                debug!("{}.new | address: {:?}", self_id, address);
                let cycle = selfConf.getDuration("cycle");
                debug!("{}.new | cycle: {:?}", self_id, cycle);
                let reconnectCycle = selfConf.getDuration("reconnect");
                debug!("{}.new | reconnectCycle: {:?}", self_id, reconnectCycle);
                let (rx, rxMaxLength) = selfConf.getInQueue().unwrap();
                debug!("{}.new | RX: {},\tmax-length: {:?}", self_id, rx, rxMaxLength);
                Self {
                    name: selfName,
                    address,
                    cycle,
                    reconnectCycle,
                    rx,
                    rxMaxLength: rxMaxLength,
                }
            },
            None => {
                panic!("ApiClientConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(value: &serde_yaml::Value) -> Self {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    pub fn read(path: &str) -> ApiClientConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        Self::from_yaml(&config)
                    },
                    Err(err) => {
                        panic!("ApiClientConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("ApiClientConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
}

#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, time::Duration, net::SocketAddr};

use crate::conf::{conf_tree::ConfTree, service_config::ServiceConfig};


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
pub struct TcpClientConfig {
    pub(crate) name: String,
    pub(crate) address: SocketAddr,
    pub(crate) cycle: Option<Duration>,
    pub(crate) reconnectCycle: Option<Duration>,
    pub(crate) rx: String,
    pub(crate) rxBuffered: bool,
    pub(crate) rxMaxLength: i64,
    pub(crate) tx: String,
}
///
/// 
impl TcpClientConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service TcpClient:
    ///     cycle: 1 ms
    ///     reconnect: 1 s  # default 3 s
    ///     address: 127.0.0.1:8080
    ///     in queue link:
    ///         buffered: true
    ///         max-length: 10000
    ///     out queue: MultiQueue.queue
    ///                     ...
    pub fn new(confTree: &mut ConfTree) -> TcpClientConfig {
        println!("\n");
        trace!("TcpClientConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("TcpClientConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        };
        match confTree.next() {
            Some(selfConf) => {
                let selfId = format!("TcpClientConfig({})", selfConf.key);
                trace!("{}.new | MAPPING VALUE", selfId);
                let mut selfConf = ServiceConfig::new(&selfId, selfConf);
                trace!("{}.new | selfConf: {:?}", selfId, selfConf);
                let selfName = selfConf.name();
                debug!("{}.new | name: {:?}", selfId, selfName);
                let selfAddress: SocketAddr = selfConf.getParamValue("address").unwrap().as_str().unwrap().parse().unwrap();
                debug!("{}.new | address: {:?}", selfId, selfAddress);
                let cycle = selfConf.getDuration("cycle");
                debug!("{}.new | cycle: {:?}", selfId, cycle);
                let reconnectCycle = selfConf.getDuration("reconnect");
                debug!("{}.new | reconnectCycle: {:?}", selfId, reconnectCycle);
                let (rx, rxMaxLength) = selfConf.getInQueue().unwrap();
                let rxBuffered = rxMaxLength > 0;
                debug!("{}.new | RX: {},\tmax-length: {}", selfId, rx, rxMaxLength);
                let tx = selfConf.getOutQueue().unwrap();
                debug!("{}.new | TX: {}", selfId, tx);
                TcpClientConfig {
                    name: selfName,
                    address: selfAddress,
                    cycle,
                    reconnectCycle,
                    rx,
                    rxBuffered: rxBuffered,
                    rxMaxLength: rxMaxLength,
                    tx,
                }
            },
            None => {
                panic!("TcpClientConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> TcpClientConfig {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> TcpClientConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        TcpClientConfig::fromYamlValue(&config)
                    },
                    Err(err) => {
                        panic!("TcpClientConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("TcpClientConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
}

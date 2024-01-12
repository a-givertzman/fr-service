#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, time::Duration, net::SocketAddr};

use crate::{conf::{conf_tree::ConfTree, service_config::ServiceConfig}, services::tcp_server::tcp_server_auth::TcpServerAuth};


///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service TcpClient:
///     cycle: 1 ms
///     address: 127.0.0.1:8080
///     reconnect: 1 s      # default 3 s
///     keep-timeout: 3s    # timeot keeping lost connection
///     auth: none          # none / secret / ssh
///     in queue link:
///         max-length: 10000
///     out queue: MultiQueue.queue
///                         ...
#[derive(Debug, PartialEq, Clone)]
pub struct TcpServerConfig {
    pub(crate) name: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) address: SocketAddr,
    pub(crate) reconnectCycle: Option<Duration>,
    pub(crate) keepTimeout: Option<Duration>,
    pub(crate) auth: TcpServerAuth,
    pub(crate) rx: String,
    pub(crate) rxMaxLength: i64,
    pub(crate) tx: String,
}
///
/// 
impl TcpServerConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service TcpClient:
    ///     cycle: 1 ms
    ///     address: 127.0.0.1:8080
    ///     reconnect: 1 s      # default 3 s
    ///     keep-timeout: 3s    # timeot keeping lost connection
    ///     auth: none          # none / secret / ssh
    ///     in queue link:
    ///         max-length: 10000
    ///     out queue: MultiQueue.queue
    ///                     ...
    pub fn new(confTree: &mut ConfTree) -> TcpServerConfig {
        println!("\n");
        trace!("TcpServerConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("TcpServerConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        };
        match confTree.next() {
            Some(selfConf) => {
                let selfId = format!("TcpServerConfig({})", selfConf.key);
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
                let keepTimeout = selfConf.getDuration("keep-timeout");
                debug!("{}.new | keepTimeout: {:?}", selfId, reconnectCycle);
                let auth = selfConf.getParamConf("auth");
                let auth = auth.or(selfConf.getParamConf("auth-secret"));
                let auth = auth.or(selfConf.getParamConf("auth-ssh"));
                let auth = auth.expect("{}.new | 'auth' or 'auth-secret' or 'auth-ssh' - not found");
                let auth = TcpServerAuth::new(auth);
                debug!("{}.new | auth: {:?}", selfId, auth);
                let (rx, rxMaxLength) = selfConf.getInQueue().unwrap();
                debug!("{}.new | RX: {},\tmax-length: {}", selfId, rx, rxMaxLength);
                let tx = selfConf.getOutQueue().unwrap();
                debug!("{}.new | TX: {}", selfId, tx);
                TcpServerConfig {
                    name: selfName,
                    cycle,
                    address: selfAddress,
                    reconnectCycle,
                    keepTimeout,
                    auth,
                    rx,
                    rxMaxLength: rxMaxLength,
                    tx,
                }
            },
            None => {
                panic!("TcpServerConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> TcpServerConfig {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> TcpServerConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        TcpServerConfig::fromYamlValue(&config)
                    },
                    Err(err) => {
                        panic!("TcpServerConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("TcpServerConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
}

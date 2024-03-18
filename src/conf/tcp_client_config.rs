use log::{trace, debug};
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
    pub(crate) reconnect_cycle: Option<Duration>,
    pub(crate) rx: String,
    pub(crate) rx_buffered: bool,
    pub(crate) rx_max_len: i64,
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
    pub fn new(conf_tree: &mut ConfTree) -> TcpClientConfig {
        println!();
        trace!("TcpClientConfig.new | confTree: {:?}", conf_tree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        let self_id = format!("TcpClientConfig({})", conf_tree.key);
        trace!("{}.new | MAPPING VALUE", self_id);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | selfConf: {:?}", self_id, self_conf);
        let self_name = self_conf.name();
        debug!("{}.new | name: {:?}", self_id, self_name);
        let self_address: SocketAddr = self_conf.get_param_value("address").unwrap().as_str().unwrap().parse().unwrap();
        debug!("{}.new | address: {:?}", self_id, self_address);
        let cycle = self_conf.get_duration("cycle");
        debug!("{}.new | cycle: {:?}", self_id, cycle);
        let reconnect_cycle = self_conf.get_duration("reconnect");
        debug!("{}.new | reconnectCycle: {:?}", self_id, reconnect_cycle);
        let (rx, rx_max_len) = self_conf.get_in_queue().unwrap();
        let rx_buffered = rx_max_len > 0;
        debug!("{}.new | RX: {},\tmax-length: {}", self_id, rx, rx_max_len);
        let tx = self_conf.get_out_queue().unwrap();
        debug!("{}.new | TX: {}", self_id, tx);
        TcpClientConfig {
            name: self_name,
            address: self_address,
            cycle,
            reconnect_cycle,
            rx,
            rx_buffered,
            rx_max_len,
            tx,
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(value: &serde_yaml::Value) -> TcpClientConfig {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(&mut ConfTree::new(key.as_str().unwrap().to_owned(), value.clone()))
            },
            None => {
                panic!("TcpClientConfig.from_yaml | Format error or empty conf: {:#?}", value)
            },
        }
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> TcpClientConfig {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        TcpClientConfig::from_yaml(&config)
                    },
                    Err(err) => {
                        panic!("TcpClientConfig.read | Error in config: {:?}\n\terror: {:#?}", yaml_string, err)
                    },
                }
            },
            Err(err) => {
                panic!("TcpClientConfig.read | File {} reading error: {:#?}", path, err)
            },
        }
    }
}

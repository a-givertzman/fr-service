use log::{trace, debug};
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
///     debug: false                # API debug mode, optional, default false
///                         ...
#[derive(Debug, PartialEq, Clone)]
pub struct ApiClientConfig {
    pub(crate) name: String,
    pub(crate) address: SocketAddr,
    pub(crate) database: String,
    pub(crate) auth_token: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) reconnect_cycle: Option<Duration>,
    pub(crate) rx: String,
    pub(crate) rx_max_len: i64,
    pub(crate) debug: bool,
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
    ///     debug: false                # API debug mode, optional, default false
    ///                     ...
    pub fn new(conf_tree: &mut ConfTree) -> Self {
        println!();
        trace!("ApiClientConfig.new | confTree: {:?}", conf_tree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        let self_id = format!("ApiClientConfig({})", conf_tree.key);
        trace!("{}.new | MAPPING VALUE", self_id);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | selfConf: {:?}", self_id, self_conf);
        let self_name = self_conf.name();
        debug!("{}.new | name: {:?}", self_id, self_name);
        let address: SocketAddr = self_conf.get_param_value("address").unwrap().as_str().unwrap().parse().unwrap();
        debug!("{}.new | address: {:?}", self_id, address);
        let database = self_conf.get_param_value("database").unwrap().as_str().unwrap().to_string();
        debug!("{}.new | database: {:?}", self_id, database);
        let auth_token = self_conf.get_param_value("auth_token").unwrap_or_default().as_str().unwrap_or("").to_string();
        debug!("{}.new | auth_token: {:?}", self_id, auth_token);
        let cycle = self_conf.get_duration("cycle");
        debug!("{}.new | cycle: {:?}", self_id, cycle);
        let reconnect_cycle = self_conf.get_duration("reconnect");
        debug!("{}.new | reconnectCycle: {:?}", self_id, reconnect_cycle);
        let (rx, rx_max_len) = self_conf.get_in_queue().unwrap();
        debug!("{}.new | RX: {},\tmax-length: {:?}", self_id, rx, rx_max_len);
        let debug: bool = self_conf.get_param_value("debug").unwrap_or_default().as_bool().unwrap_or(false);
        debug!("{}.new | debug: {:?}", self_id, debug);
        Self {
            name: self_name,
            address,
            database,
            auth_token,
            cycle,
            reconnect_cycle,
            rx,
            rx_max_len,
            debug,
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(value: &serde_yaml::Value) -> Self {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(&mut ConfTree::new(key.as_str().unwrap().to_owned(), value.clone()))
            },
            None => {
                panic!("ApiClientConfig.from_yaml | Format error or empty conf: {:#?}", value)
            },
        }        
    }
    ///
    /// reads config from path
    pub fn read(path: &str) -> ApiClientConfig {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        Self::from_yaml(&config)
                    },
                    Err(err) => {
                        panic!("ApiClientConfig.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    },
                }
            },
            Err(err) => {
                panic!("ApiClientConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
}

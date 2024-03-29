use log::{trace, debug};
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
    pub(crate) rx_max_length: i64,
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
    pub fn new(conf_tree: &mut ConfTree) -> MultiQueueConfig {
        println!();
        trace!("MultiQueueConfig.new | confTree: {:?}", conf_tree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        // if confTree.count() > 1 {
        //     error!("MultiQueueConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        // };
        let self_id = format!("MultiQueueConfig({})", conf_tree.key);
        trace!("{}.new | MAPPING VALUE", self_id);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        let self_name = self_conf.name();
        debug!("{}.new | self_name: {:?}", self_id, self_name);
        let (rx, rx_max_length) = self_conf.get_in_queue().unwrap();
        debug!("{}.new | RX: {},\tmax-length: {}", self_id, rx, rx_max_length);
        let tx = match self_conf.get_param_by_keyword("out", ConfKind::Queue) {
            Ok((keyword, queue_conf)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                trace!("{}.new | self tx-queue param {}: {:?}", self_id, name, queue_conf);
                let queues: Vec<String> = match queue_conf.conf.as_sequence() {
                    Some(queues) => {
                        queues.iter().map(|value| {
                            value.as_str().unwrap().to_owned()
                        }).collect()
                },
                    None => vec![],
                };
                queues
            },
            Err(err) => panic!("{}.new | out queue - not found in : {:?}\n\terror: {:?}", self_id, self_conf, err),
        };
        debug!("{}.new | TX: {:?}", self_id, tx);
        MultiQueueConfig {
            name: self_name,
            rx,
            rx_max_length,
            tx,
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(value: &serde_yaml::Value) -> MultiQueueConfig {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(&mut ConfTree::new(key.as_str().unwrap().to_owned(), value.clone()))
            },
            None => {
                panic!("MultiQueueConfig.from_yaml | Format error or empty conf: {:#?}", value)
            },
        }
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> MultiQueueConfig {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        MultiQueueConfig::from_yaml(&config)
                    },
                    Err(err) => {
                        panic!("MultiQueueConfig.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    },
                }
            },
            Err(err) => {
                panic!("MultiQueueConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
}

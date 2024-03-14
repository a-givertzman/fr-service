use indexmap::IndexMap;
use log::{debug, trace};
use std::{fs, str::FromStr, time::Duration};
use crate::conf::{
    conf_tree::ConfTree, 
    point_config::point_config::PointConfig, 
    service_config::ServiceConfig,
    profinet_client_config::keywd::{Keywd, Kind}, 
    profinet_client_config::profinet_db_config::ProfinetDbConfig,
};
///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service ProfinetClient Ied01:          # device will be executed in the independent thread, must have unique name
///    in queue in-queue:
///        max-length: 10000
///    out queue: MultiQueue.in-queue
///    name Ied01:                       
///    cycle: 1 ms                     # operating cycle time of the device
///    protocol: 'profinet'
///    description: 'S7-IED-01.01'
///    ip: '192.168.100.243'
///    rack: 0
///    slot: 1
///    db db899:                       # multiple DB blocks are allowed, must have unique namewithing parent device
///        description: 'db899 | Exhibit - drive data'
///        number: 899
///        offset: 0
///        size: 34
///        delay: 10
///        point Drive.Speed: 
///            type: 'Real'
///            offset: 0
///                 ...
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ProfinetClientConfig {
    pub(crate) name: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) rx: String,
    pub(crate) rx_max_len: i64,
    pub(crate) tx: String,
    pub(crate) protocol: String,
    pub(crate) description: String,
    pub(crate) ip: String,
    pub(crate) rack: u64,
    pub(crate) slot: u64,
    pub(crate) dbs: IndexMap<String, ProfinetDbConfig>,
}
///
/// 
impl ProfinetClientConfig {
    ///
    /// Creates new instance of the [ProfinetClientConfig]:
    pub fn new(conf_tree: &mut ConfTree) -> Self {
        println!();
        debug!("ProfinetClientConfig.new | conf_tree: {:#?}", conf_tree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        let self_id = format!("ProfinetClientConfig({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        let self_name = self_conf.name();
        let device_name = self_conf.sufix();
        debug!("{}.new | name: {:?}", self_id, self_name);
        let cycle = self_conf.get_duration("cycle");
        debug!("{}.new | cycle: {:?}", self_id, cycle);
        let (rx, rx_max_len) = self_conf.get_in_queue().unwrap();
        debug!("{}.new | RX: {},\tmax-length: {}", self_id, rx, rx_max_len);
        let tx = self_conf.get_out_queue().unwrap();
        debug!("{}.new | TX: {}", self_id, tx);
        let protocol = self_conf.get_param_value("protocol").unwrap().as_str().unwrap().to_string();
        debug!("{}.new | protocol: {:?}", self_id, protocol);
        let description = self_conf.get_param_value("description").unwrap().as_str().unwrap().to_string();
        debug!("{}.new | description: {:?}", self_id, description);
        let ip = self_conf.get_param_value("ip").unwrap().as_str().unwrap().to_string();
        debug!("{}.new | ip: {:?}", self_id, ip);
        let rack = self_conf.get_param_value("rack").unwrap().as_u64().unwrap();
        debug!("{}.new | rack: {:?}", self_id, rack);
        let slot = self_conf.get_param_value("slot").unwrap().as_u64().unwrap();
        debug!("{}.new | slot: {:?}", self_id, slot);
        let mut dbs = IndexMap::new();
        for key in &self_conf.keys {
            let keyword = Keywd::from_str(key).unwrap();
            if keyword.kind() == Kind::Db {
                let db_name = keyword.name();
                let mut device_conf = self_conf.get(key).unwrap();
                debug!("{}.new | DB '{}'", self_id, db_name);
                trace!("{}.new | DB '{}'   |   conf: {:?}", self_id, db_name, device_conf);
                let node_conf = ProfinetDbConfig::new(&device_name, &db_name, &mut device_conf);
                dbs.insert(
                    db_name,
                    node_conf,
                );
            } else {
                debug!("{}.new | device expected, but found {:?}", self_id, keyword);
            }
        }
        ProfinetClientConfig {
            name: self_name,
            cycle,
            rx,
            rx_max_len,
            tx,
            protocol,
            description,
            ip,
            rack,
            slot,
            dbs
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(value: &serde_yaml::Value) -> ProfinetClientConfig {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(&mut ConfTree::new(key.as_str().unwrap().to_owned(), value.clone()))
            },
            None => {
                panic!("ProfinetClientConfig.from_yaml | Format error or empty conf: {:#?}", value)
            },
        }
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> ProfinetClientConfig {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        ProfinetClientConfig::from_yaml(&config)
                    },
                    Err(err) => {
                        panic!("ProfinetClientConfig.read | Error in config: {:?}\n\terror: {:#?}", yaml_string, err)
                    },
                }
            },
            Err(err) => {
                panic!("ProfinetClientConfig.read | File {} reading error: {:#?}", path, err)
            },
        }
    }
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.dbs.iter().fold(vec![], |mut points, (_device_name, device_conf)| {
            points.extend(device_conf.points());
            points
        })
    }
}

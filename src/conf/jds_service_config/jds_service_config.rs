use indexmap::IndexMap;
use log::{debug, error, trace};
use std::{fs, str::FromStr, time::Duration};

use crate::conf::{
    conf_tree::ConfTree, 
    point_config::point_config::PointConfig, 
    service_config::ServiceConfig,
    profinet_client_config::keywd::{Keywd, Kind}, 
    profinet_client_config::profinet_db_config::ProfinetDbConfig,
};


///
/// Creates config from serde_yaml::Value of following format:
/// ```yaml
/// service JdsService Id01:          # service unique address used in the point path
///    in queue in-queue:
///        max-length: 10000
///    out queue: MultiQueue.in-queue
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct JdsServiceConfig {
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
impl JdsServiceConfig {
    ///
    /// Creates new instance of the [JdsServiceConfig]:
    pub fn new(conf_tree: &mut ConfTree) -> Self {
        println!("\n");
        trace!("JdsServiceConfig.new | confTree: {:?}", conf_tree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if conf_tree.count() > 1 {
            error!("JdsServiceConfig.new | JdsServiceConfig conf must have single item, additional items was ignored: {:?}", conf_tree)
        };
        match conf_tree.next() {
            Some(self_conf) => {
                let self_id = format!("JdsServiceConfig({})", self_conf.key);
                trace!("{}.new | MAPPING VALUE", self_id);
                let mut self_conf = ServiceConfig::new(&self_id, self_conf);
                trace!("{}.new | selfConf: {:?}", self_id, self_conf);
                let self_name = self_conf.name();
                let device_name = self_conf.sufix();
                debug!("{}.new | name: {:?}", self_id, self_name);
                let cycle = self_conf.getDuration("cycle");
                debug!("{}.new | cycle: {:?}", self_id, cycle);
                let (rx, rx_max_len) = self_conf.getInQueue().unwrap();
                debug!("{}.new | RX: {},\tmax-length: {}", self_id, rx, rx_max_len);
                let tx = self_conf.getOutQueue().unwrap();
                debug!("{}.new | TX: {}", self_id, tx);
                let protocol = self_conf.getParamValue("protocol").unwrap().as_str().unwrap().to_string();
                debug!("{}.new | protocol: {:?}", self_id, protocol);
                let description = self_conf.getParamValue("description").unwrap().as_str().unwrap().to_string();
                debug!("{}.new | description: {:?}", self_id, description);
                let ip = self_conf.getParamValue("ip").unwrap().as_str().unwrap().to_string();
                debug!("{}.new | ip: {:?}", self_id, ip);
                let rack = self_conf.getParamValue("rack").unwrap().as_u64().unwrap();
                debug!("{}.new | rack: {:?}", self_id, rack);
                let slot = self_conf.getParamValue("slot").unwrap().as_u64().unwrap();
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
                JdsServiceConfig {
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
            },
            None => {
                panic!("JdsServiceConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml_value(value: &serde_yaml::Value) -> JdsServiceConfig {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> JdsServiceConfig {
        match fs::read_to_string(&path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        JdsServiceConfig::from_yaml_value(&config)
                    },
                    Err(err) => {
                        panic!("JdsServiceConfig.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    },
                }
            },
            Err(err) => {
                panic!("JdsServiceConfig.read | File {} reading error: {:?}", path, err)
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

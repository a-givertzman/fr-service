#![allow(non_snake_case)]

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
    pub(crate) rxMaxLength: i64,
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
    pub fn new(confTree: &mut ConfTree) -> Self {
        println!("\n");
        trace!("ProfinetClientConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("ProfinetClientConfig.new | ProfinetClientConfig conf must have single item, additional items was ignored: {:?}", confTree)
        };
        match confTree.next() {
            Some(selfConf) => {
                let selfId = format!("ProfinetClientConfig({})", selfConf.key);
                trace!("{}.new | MAPPING VALUE", selfId);
                let mut selfConf = ServiceConfig::new(&selfId, selfConf);
                trace!("{}.new | selfConf: {:?}", selfId, selfConf);
                let selfName = selfConf.name();
                debug!("{}.new | name: {:?}", selfId, selfName);
                let cycle = selfConf.getDuration("cycle");
                debug!("{}.new | cycle: {:?}", selfId, cycle);
                let (rx, rxMaxLength) = selfConf.getInQueue().unwrap();
                debug!("{}.new | RX: {},\tmax-length: {}", selfId, rx, rxMaxLength);
                let tx = selfConf.getOutQueue().unwrap();
                debug!("{}.new | TX: {}", selfId, tx);
                let protocol = selfConf.getParamValue("protocol").unwrap().as_str().unwrap().to_string();
                debug!("{}.new | protocol: {:?}", selfId, protocol);
                let description = selfConf.getParamValue("description").unwrap().as_str().unwrap().to_string();
                debug!("{}.new | description: {:?}", selfId, description);
                let ip = selfConf.getParamValue("ip").unwrap().as_str().unwrap().to_string();
                debug!("{}.new | ip: {:?}", selfId, ip);
                let rack = selfConf.getParamValue("rack").unwrap().as_u64().unwrap();
                debug!("{}.new | rack: {:?}", selfId, rack);
                let slot = selfConf.getParamValue("slot").unwrap().as_u64().unwrap();
                debug!("{}.new | slot: {:?}", selfId, slot);
                let mut dbs = IndexMap::new();
                for key in &selfConf.keys {
                    let keyword = Keywd::from_str(key).unwrap();
                    if keyword.kind() == Kind::Db {
                        let deviceName = keyword.name();
                        let mut deviceConf = selfConf.get(key).unwrap();
                        debug!("{}.new | DB '{}'", selfId, deviceName);
                        trace!("{}.new | DB '{}'   |   conf: {:?}", selfId, deviceName, deviceConf);
                        let nodeConf = ProfinetDbConfig::new(&format!("{}/{}", selfName, deviceName), &mut deviceConf);
                        dbs.insert(
                            deviceName,
                            nodeConf,
                        );
                    } else {
                        debug!("{}.new | device expected, but found {:?}", selfId, keyword);
                    }
                }
                ProfinetClientConfig {
                    name: selfName,
                    cycle,
                    rx,
                    rxMaxLength: rxMaxLength,
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
                panic!("ProfinetClientConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> ProfinetClientConfig {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> ProfinetClientConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        ProfinetClientConfig::fromYamlValue(&config)
                    },
                    Err(err) => {
                        panic!("ProfinetClientConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("ProfinetClientConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.dbs.iter().fold(vec![], |mut points, (_deviceName, deviceConf)| {
            points.extend(deviceConf.points());
            points
        })
    }
}

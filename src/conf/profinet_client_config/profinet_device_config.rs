#![allow(non_snake_case)]

use std::{fs, time::Duration};
use log::{debug, error, trace};
use crate::conf::{
    conf_tree::ConfTree, 
    point_config::point_config::PointConfig, 
    profinet_client_config::profinet_db_config::ProfinetDbConfig, 
    service_config::ServiceConfig,
};

///
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ProfinetDeviceConfig {
    pub(crate) name: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) protocol: String,
    pub(crate) description: String,
    pub(crate) ip: String,
    pub(crate) rack: u64,
    pub(crate) slot: u64,
    pub(crate) dbs: Vec<ProfinetDbConfig>,
}
///
/// 
impl ProfinetDeviceConfig {
    pub fn new(name: &str, confTree: &mut ConfTree) -> Self {
        // println!("\n");
        trace!("ProfinetDeviceConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        // if confTree.count() > 1 {
        //     error!("ProfinetDeviceConfig.new | Device conf must have single item, additional items was ignored: {:?}", confTree)
        // };
        let selfConf = confTree.clone();
        let selfId = format!("ProfinetDeviceConfig({})", selfConf.key);
        trace!("{}.new | MAPPING VALUE", selfId);
        let mut selfConf = ServiceConfig::new(&selfId, selfConf);
        trace!("{}.new | selfConf: {:?}", selfId, selfConf);
        let selfName = name;
        debug!("{}.new | name: {:?}", selfId, selfName);
        let cycle = selfConf.getDuration("cycle");
        debug!("{}.new | cycle: {:?}", selfId, cycle);
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
        let dbs = vec![];
        Self {
            name: selfName.to_string(),
            cycle,
            protocol,
            description,
            ip,
            rack,
            slot,
            dbs,
        }
        // match confTree.next() {
        //     Some(selfConf) => {
        //     },
        //     None => {
        //         panic!("ProfinetDeviceConfig.new | Configuration is empty")
        //     },
        // }
    }
    // ///
    // /// creates config from serde_yaml::Value of following format:
    // pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> Self {
    //     Self::new(&mut ConfTree::newRoot(value.clone()))
    // }
    // ///
    // /// reads config from path
    // #[allow(dead_code)]
    // pub fn read(path: &str) -> Self {
    //     match fs::read_to_string(&path) {
    //         Ok(yamlString) => {
    //             match serde_yaml::from_str(&yamlString) {
    //                 Ok(config) => {
    //                     Self::fromYamlValue(&config)
    //                 },
    //                 Err(err) => {
    //                     panic!("ProfinetDeviceConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
    //                 },
    //             }
    //         },
    //         Err(err) => {
    //             panic!("ProfinetDeviceConfig.read | File {} reading error: {:?}", path, err)
    //         },
    //     }
    // }
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.dbs.iter().fold(vec![], |mut points, dbConf| {
            points.extend(dbConf.points());
            points
        })
    }
}
#![allow(non_snake_case)]

use log::{debug, trace};
use std::time::Duration;
use crate::conf::{conf_tree::ConfTree, point_config::point_config::PointConfig, service_config::ServiceConfig};

///
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ProfinetDbConfig {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) number: u64,
    pub(crate) offset: u64,
    pub(crate) size: u64,
    pub(crate) cycle: Option<Duration>,
    pub(crate) points: Vec<PointConfig>,
}
///
/// 
impl ProfinetDbConfig {
    ///
    /// Creates new instance of the ProfinetDbConfig
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
        let description = selfConf.getParamValue("description").unwrap().as_str().unwrap().to_string();
        debug!("{}.new | description: {:?}", selfId, description);
        let number = selfConf.getParamValue("number").unwrap().as_u64().unwrap();
        debug!("{}.new | ip: {:?}", selfId, number);
        let offset = selfConf.getParamValue("offset").unwrap().as_u64().unwrap();
        debug!("{}.new | rack: {:?}", selfId, offset);
        let size = selfConf.getParamValue("size").unwrap().as_u64().unwrap();
        debug!("{}.new | slot: {:?}", selfId, size);
        let points = vec![];
        Self {
            name: selfName.to_string(),
            description,
            number,
            offset,
            size,
            cycle,
            points,
        }
        // match confTree.next() {
        //     Some(selfConf) => {
        //     },
        //     None => {
        //         panic!("ProfinetDeviceConfig.new | Configuration is empty")
        //     },
        // }
    }    
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.points.iter().fold(vec![], |mut points, conf| {
            points.push(conf.clone());
            points
        })
    }
}
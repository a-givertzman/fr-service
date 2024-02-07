#![allow(non_snake_case)]

use log::{debug, trace};
use std::{str::FromStr, time::Duration};
use crate::conf::{conf_tree::ConfTree, fn_conf_keywd::{FnConfKeywd, FnConfKindName}, point_config::point_config::PointConfig, service_config::ServiceConfig};

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
        let selfConf = confTree.clone();
        let selfId = format!("ProfinetDeviceConfig({})", selfConf.key);
        trace!("{}.new | MAPPING VALUE", selfId);
        let mut selfConf = ServiceConfig::new(&selfId, selfConf);
        trace!("{}.new | selfConf: {:?}", selfId, selfConf);
        let selfName = name;
        debug!("{}.new | name: {:?}", selfId, selfName);
        let cycle = selfConf.getDuration("cycle");
        debug!("{}.new | cycle: {:?}", selfId, cycle);
        let description = selfConf.getParamValue("description").unwrap_or(serde_yaml::Value::String(String::new())).as_str().unwrap().to_string();
        debug!("{}.new | description: {:?}", selfId, description);
        let number = selfConf.getParamValue("number").unwrap().as_u64().unwrap();
        debug!("{}.new | number: {:?}", selfId, number);
        let offset = selfConf.getParamValue("offset").unwrap().as_u64().unwrap();
        debug!("{}.new | offset: {:?}", selfId, offset);
        let size = selfConf.getParamValue("size").unwrap().as_u64().unwrap();
        debug!("{}.new | size: {:?}", selfId, size);
        let mut points = vec![];
        for key in &selfConf.keys {
            let keyword = FnConfKeywd::from_str(key).unwrap();
            if keyword.kind() == FnConfKindName::Point {
                let pointName = keyword.data();
                let deviceConf = selfConf.get(key).unwrap();
                debug!("{}.new | Point '{}'", selfId, pointName);
                trace!("{}.new | Point '{}'   |   conf: {:?}", selfId, pointName, deviceConf);
                let nodeConf = PointConfig::new(&deviceConf);
                points.push(
                    nodeConf,
                );
            } else {
                debug!("{}.new | device expected, but found {:?}", selfId, keyword);
            }
        }
        Self {
            name: selfName.to_string(),
            description,
            number,
            offset,
            size,
            cycle,
            points,
        }
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
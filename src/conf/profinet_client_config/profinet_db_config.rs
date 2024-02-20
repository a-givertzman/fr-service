use log::{debug, trace};
use std::{str::FromStr, time::Duration};
use crate::conf::{conf_tree::ConfTree, fn_conf_keywd::{FnConfKeywd, FnConfKindName}, point_config::{point_config::PointConfig, point_name::PointName}, service_config::ServiceConfig};

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
    pub fn new(parent: &str, name: &str, conf_tree: &mut ConfTree) -> Self {
        // println!("\n");
        trace!("ProfinetDeviceConfig.new | confTree: {:?}", conf_tree);
        let self_conf = conf_tree.clone();
        let self_id = format!("ProfinetDeviceConfig({})", self_conf.key);
        trace!("{}.new | MAPPING VALUE", self_id);
        let mut self_conf = ServiceConfig::new(&self_id, self_conf);
        trace!("{}.new | selfConf: {:?}", self_id, self_conf);
        let self_name = PointName::new(parent, &name).full();
        debug!("{}.new | name: {:?}", self_id, self_name);
        let cycle = self_conf.getDuration("cycle");
        debug!("{}.new | cycle: {:?}", self_id, cycle);
        let description = self_conf.getParamValue("description").unwrap_or(serde_yaml::Value::String(String::new())).as_str().unwrap().to_string();
        debug!("{}.new | description: {:?}", self_id, description);
        let number = self_conf.getParamValue("number").unwrap().as_u64().unwrap();
        debug!("{}.new | number: {:?}", self_id, number);
        let offset = self_conf.getParamValue("offset").unwrap().as_u64().unwrap();
        debug!("{}.new | offset: {:?}", self_id, offset);
        let size = self_conf.getParamValue("size").unwrap().as_u64().unwrap();
        debug!("{}.new | size: {:?}", self_id, size);
        let mut points = vec![];
        for key in &self_conf.keys {
            let keyword = FnConfKeywd::from_str(key).unwrap();
            if keyword.kind() == FnConfKindName::Point {
                let point_name = format!("{}/{}", self_name, keyword.data());
                let point_conf = self_conf.get(key).unwrap();
                debug!("{}.new | Point '{}'", self_id, point_name);
                trace!("{}.new | Point '{}'   |   conf: {:?}", self_id, point_name, point_conf);
                let node_conf = PointConfig::new(&self_name, &point_conf);
                points.push(
                    node_conf,
                );
            } else {
                debug!("{}.new | device expected, but found {:?}", self_id, keyword);
            }
        }
        Self {
            name: self_name,
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
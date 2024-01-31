#![allow(non_snake_case)]

use log::{trace, debug, error};
use std::{fs, time::Duration};

use crate::conf::{
    conf_tree::ConfTree, service_config::ServiceConfig,
    point_config::point_config::PointConfig,
};

use super::profinet_device_config::ProfinetDeviceConfig;


///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service ProfinetClient:
///    cycle: 1 ms             # operating cycle time of the module
///    in queue in-queue:
///        max-length: 10000
///    out queue: MultiQueue.in-queue
///    device Ied01:                       # device will be executed in the independent thread, must have unique name
///        protocol: 'profinet'
///        description: 'S7-IED-01.01'
///        ip: '192.168.100.243'
///        rack: 0
///        slot: 1
///        db db899:                       # multiple DB blocks are allowed, must have unique namewithing parent device
///            description: 'db899 | Exhibit - drive data'
///            number: 899
///            offset: 0
///            size: 34
///            delay: 10
///            point Drive.Speed: 
///                type: 'Real'
///                offset: 0
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ProfinetClientConfig {
    pub(crate) name: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) rx: String,
    pub(crate) rxMaxLength: i64,
    pub(crate) tx: String,
    pub(crate) devices: Vec<ProfinetDeviceConfig>,
}
///
/// 
impl ProfinetClientConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service ProfinetClient:
    ///    cycle: 1 ms             # operating cycle time of the module
    ///    in queue in-queue:
    ///        max-length: 10000
    ///    out queue: MultiQueue.in-queue
    ///    device Ied01:                       # device will be executed in the independent thread, must have unique name
    ///        protocol: 'profinet'
    ///        description: 'S7-IED-01.01'
    ///        ip: '192.168.100.243'
    ///        rack: 0
    ///        slot: 1
    ///        db db899:                       # multiple DB blocks are allowed, must have unique namewithing parent device
    ///            description: 'db899 | Exhibit - drive data'
    ///            number: 899
    ///            offset: 0
    ///            size: 34
    ///            delay: 10
    ///            point Drive.Speed: 
    ///                type: 'Real'
    ///                offset: 0
    ///                     ...
    pub fn new(confTree: &mut ConfTree) -> ProfinetClientConfig {
        println!("\n");
        trace!("ProfinetClientConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("ProfinetClientConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        };
        match confTree.next() {
            Some(selfConf) => {
                let selfId = format!("ProfinetClientConfig({})", selfConf.key);
                trace!("{}.new | MAPPING VALUE", selfId);
                let mut selfConf = ServiceConfig::new(&selfId, selfConf);
                trace!("{}.new | selfConf: {:?}", selfId, selfConf);
                let selfName = selfConf.name();
                debug!("{}.new | name: {:?}", selfId, selfName);
                // let selfAddress: SocketAddr = selfConf.getParamValue("address").unwrap().as_str().unwrap().parse().unwrap();
                // debug!("{}.new | address: {:?}", selfId, selfAddress);
                let cycle = selfConf.getDuration("cycle");
                debug!("{}.new | cycle: {:?}", selfId, cycle);
                let (rx, rxMaxLength) = selfConf.getInQueue().unwrap();
                debug!("{}.new | RX: {},\tmax-length: {}", selfId, rx, rxMaxLength);
                let tx = selfConf.getOutQueue().unwrap();
                debug!("{}.new | TX: {}", selfId, tx);
                let devices = vec![];
                ProfinetClientConfig {
                    name: selfName,
                    cycle,
                    rx,
                    rxMaxLength: rxMaxLength,
                    tx,
                    devices
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
        self.devices.iter().fold(vec![], |mut points, deviceConf| {
            points.extend(deviceConf.points());
            points
        })
    }
}

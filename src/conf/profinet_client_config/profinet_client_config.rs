#![allow(non_snake_case)]

use indexmap::IndexMap;
use log::{debug, error, trace, warn};
use std::{fs, str::FromStr, time::Duration};

use crate::conf::{
    conf_tree::ConfTree, point_config::point_config::PointConfig, profinet_client_config::keywd::{Keywd, Kind}, service_config::ServiceConfig
};

use super::profinet_device_config::ProfinetDeviceConfig;


///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service ProfinetClient:
///    in queue in-queue:
///        max-length: 10000
///    out queue: MultiQueue.in-queue
///    device Ied01:                       # device will be executed in the independent thread, must have unique name
///        cycle: 1 ms                     # operating cycle time of the device
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
    pub(crate) devices: IndexMap<String, ProfinetDeviceConfig>,
}
///
/// 
impl ProfinetClientConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service ProfinetClient:
    ///    in queue in-queue:
    ///        max-length: 10000
    ///    out queue: MultiQueue.in-queue
    ///    device Ied01:                       # device will be executed in the independent thread, must have unique name
    ///        cycle: 1 ms                     # operating cycle time of the device
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
    pub fn new(confTree: &mut ConfTree) -> Self {
        println!("\n");
        trace!("ProfinetClientConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            warn!("ProfinetClientConfig.new | ProfinetClientConfig conf must have single item, additional items was ignored: {:?}", confTree)
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

                let mut devices = IndexMap::new();
                for key in &selfConf.keys {
                    let keyword = Keywd::from_str(key).unwrap();
                    if keyword.kind() == Kind::Device {
                        let deviceName = keyword.name();
                        let mut deviceConf = selfConf.get(key).unwrap();
                        debug!("{}.new | device '{}'", selfId, deviceName);
                        trace!("{}.new | device '{}'   |   conf: {:?}", selfId, deviceName, deviceConf);
                        let nodeConf = ProfinetDeviceConfig::new(&deviceName, &mut deviceConf);
                        devices.insert(
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
        self.devices.iter().fold(vec![], |mut points, (_deviceName, deviceConf)| {
            points.extend(deviceConf.points());
            points
        })
    }
}

#![allow(non_snake_case)]

use indexmap::IndexMap;
use log::{trace, debug, error};
use std::{fs, time::Duration};

use crate::conf::{fn_config::FnConfig, conf_tree::ConfTree, service_config::ServiceConfig};


// #[derive(Debug, Clone, PartialEq)]
// pub enum TaskConfNode {
//     Fn(FnConfig),
//     Metric(MetricConfig)
// }

// impl TaskConfNode {
//     pub fn name(&self) -> String {
//         match self {
//             TaskConfNode::Fn(conf) => conf.name.clone(),
//             TaskConfNode::Metric(conf) => conf.name.clone(),
//         }
//     }
// }

///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// task operatingMetric:
///     cycle: 100 ms
///     metrics:
///         metric sqlUpdateMetric:
///             table: "TableName"
///             sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
///             initial: 123.456
///             inputs:
///                 input1:
///                     fn functionName:
///                         ...
///                 input2:
///                     metric sqlSelectMetric:
///                         ...
#[derive(Debug, PartialEq, Clone)]
pub struct TaskConfig {
    pub(crate) name: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) recvQueue: String,
    pub(crate) recvQueueMaxLength: i64,
    pub(crate) nodes: IndexMap<String, FnConfig>,
    pub(crate) vars: Vec<String>,
}
///
/// 
impl TaskConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// task taskName:
    ///     cycle: 100  // ms
    ///     metric sqlUpdateMetric:
    ///         table: "TableName"
    ///         sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
    ///         initial: 123.456
    ///         inputs:
    ///             input1:
    ///                 fn functionName:
    ///                     ...
    ///             input2:
    ///                 metric sqlSelectMetric:
    ///                     ...
    pub fn new(confTree: &mut ConfTree) -> TaskConfig {
        println!("\n");
        trace!("TaskConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("TaskConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        };
        let mut vars = vec![];
        match confTree.next() {
            Some(selfConf) => {
                let selfId = format!("TaskConfig({})", selfConf.key);
                trace!("{}.new | MAPPING VALUE", selfId);
                let mut selfConf = ServiceConfig::new(&selfId, selfConf);
                trace!("{}.new | selfConf: {:?}", selfId, selfConf);
                let selfName = selfConf.name();
                debug!("{}.new | selfName: {:?}", selfId, selfName);
                let selfCycle = selfConf.getDuration("cycle");
                debug!("{}.new | selfCycle: {:?}", selfId, selfCycle);
                let (selfRecvQueue, selfRecvQueueMaxLength) = selfConf.getInQueue().unwrap();
                debug!("{}.new | selfRecvQueue: {:?}", selfId, selfRecvQueue);
                debug!("{}.new | selfRecvQueue: {},\tmax-length: {}", selfId, selfRecvQueue, selfRecvQueueMaxLength);
                let mut nodeIndex = 0;
                let mut selfNodes = IndexMap::new();
                for key in &selfConf.keys {
                    let selfNodeConf = selfConf.get(key).unwrap();
                    trace!("{}.new | selfNodeConf: {:?}", selfId, selfNodeConf);
                    nodeIndex += 1;
                    let nodeConf = FnConfig::new(&selfNodeConf, &mut vars);
                    selfNodes.insert(
                        format!("{}-{}", nodeConf.name, nodeIndex),
                        nodeConf,
                    );
                }
                TaskConfig {
                    name: selfName,
                    cycle: selfCycle,
                    recvQueue: selfRecvQueue,
                    recvQueueMaxLength: selfRecvQueueMaxLength,
                    nodes: selfNodes,
                    vars: vars,
                }
            },
            None => {
                panic!("TaskConfig.new | Configuration is empty")
            },
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn fromYamlValue(value: &serde_yaml::Value) -> TaskConfig {
        Self::new(&mut ConfTree::newRoot(value.clone()))
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> TaskConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        TaskConfig::fromYamlValue(&config)
                    },
                    Err(err) => {
                        panic!("Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("File {} reading error: {:?}", path, err)
            },
        }
    }
    // ///
    // /// 
    // fn getParam(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, name: &str) -> Option<serde_yaml::Value> {
    //     match selfKeys.iter().position(|x| *x == name) {
    //         Some(index) => {
    //             selfKeys.remove(index);
    //             match selfConf.get(name) {
    //                 Some(confTree) => Some(confTree.conf),
    //                 None => None,
    //             }
    //         },
    //         None => None,
    //     }
    // }
    // ///
    // /// 
    // fn getParamByKeyword(selfConf: &mut ConfTree, selfKeys: &mut Vec<String>, keywordPrefix: &str, keywordKind: ConfKind) -> Option<(ConfKeywd, ConfTree)> {
    //     // let mut map = HashMap::new();
    //     for node in selfConf.subNodes().unwrap() {
    //         match ConfKeywd::from_str(&node.key) {
    //             Ok(keyword) => {
    //                 if keyword.kind() == keywordKind && keyword.prefix() == keywordPrefix {
    //                     return Some((keyword, node));
    //                 }
    //             },
    //             Err(_) => {},
    //         }
    //     }
    //     None
    // }
}

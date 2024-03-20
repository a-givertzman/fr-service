use indexmap::IndexMap;
use log::{trace, debug};
use std::{fs, time::Duration};
use crate::conf::{
    fn_::{
        fn_config::FnConfig,
        fn_conf_kind::FnConfKind,
    },
    conf_tree::ConfTree, service_config::ServiceConfig,
    point_config::point_config::PointConfig,
};

///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service History History1:
///     cycle: 100 ms
///     table: history
///     suscribe:
///         MultiQueue:
///             Inf: *
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct HistoryConfig {
    pub(crate) name: String,
    pub(crate) cycle: Option<Duration>,
    pub(crate) rx: String,
    pub(crate) rx_max_length: i64,
    pub(crate) nodes: IndexMap<String, FnConfKind>,
    pub(crate) vars: Vec<String>,
}
///
/// 
impl HistoryConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// task taskName:
    ///     cycle: 100 ms
    ///     fn sqlUpdateMetric:
    ///         table: "TableName"
    ///         sql: "UPDATE {table} SET kind = '{input1}' WHERE id = '{input2}';"
    ///         initial: 123.456
    ///         inputs:
    ///             input1:
    ///                 fn functionName:
    ///                     ...
    ///             input2:
    ///                 fn SqlMetric:
    ///                     ...
    pub fn new(conf_tree: &mut ConfTree) -> Self {
        println!();
        trace!("HistoryConfig.new | conf_tree: {:?}", conf_tree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        let mut vars = vec![];
        let self_id = format!("HistoryConfig({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        let self_name = self_conf.sufix();
        debug!("{}.new | name: {:?}", self_id, self_name);
        let cycle = self_conf.get_duration("cycle");
        debug!("{}.new | cycle: {:?}", self_id, cycle);
        let (rx, rx_max_length) = self_conf.get_in_queue().unwrap();
        debug!("{}.new | RX: {},\tmax-length: {:?}", self_id, rx, rx_max_length);
        let mut node_index = 0;
        let mut nodes = IndexMap::new();
        for key in &self_conf.keys {
            let node_conf = self_conf.get(key).unwrap();
            trace!("{}.new | nodeConf: {:?}", self_id, node_conf);
            node_index += 1;
            let node_conf = FnConfig::new(&self_name, &node_conf, &mut vars);
            nodes.insert(
                format!("{}-{}", node_conf.name(), node_index),
                node_conf,
            );
        }
        Self {
            name: self_name,
            cycle,
            rx,
            rx_max_length,
            nodes,
            vars,
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(value: &serde_yaml::Value) -> HistoryConfig {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(&mut ConfTree::new(key.as_str().unwrap(), value.clone()))
            },
            None => {
                panic!("HistoryConfig.from_yaml | Format error or empty conf: {:#?}", value)
            },
        }        
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> HistoryConfig {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        HistoryConfig::from_yaml(&config)
                    },
                    Err(err) => {
                        panic!("HistoryConfig.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    },
                }
            },
            Err(err) => {
                panic!("HistoryConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.nodes.iter().fold(vec![], |mut points, (_node_name,node_conf)| {
            points.extend(node_conf.points());
            points
        })
    }
}

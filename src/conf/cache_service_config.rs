use log::{trace, debug};
use std::fs;
use crate::conf::{
    conf_tree::ConfTree, service_config::ServiceConfig,
    point_config::name::Name,
    conf_subscribe::ConfSubscribe,
};
///
/// creates config from serde_yaml::Value of following format:
/// ```yaml
/// service CacheService Cache:
///     retain: true    # true / false - enables storing cache on the disk
///     suscribe:
///         /App/MultiQueue: []
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct CacheServiceConfig {
    pub(crate) name: Name,
    pub(crate) retain: bool,
    pub(crate) subscribe: ConfSubscribe,
}
///
/// 
impl CacheServiceConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// service CacheService Cache:
    ///     retain: true    # true / false - enables storing cache on the disk
    ///     suscribe:
    ///         /App/MultiQueue: []
    /// ````
    pub fn new(parent: impl Into<String>, conf_tree: &mut ConfTree) -> Self {
        println!();
        trace!("CacheServiceConfig.new | conf_tree: {:?}", conf_tree);
        let self_id = format!("CacheServiceConfig({})", conf_tree.key);
        let mut self_conf = ServiceConfig::new(&self_id, conf_tree.clone());
        trace!("{}.new | self_conf: {:?}", self_id, self_conf);
        let self_name = Name::new(parent, self_conf.sufix());
        debug!("{}.new | name: {:?}", self_id, self_name);
        let retain = self_conf.get_param_value("retain").unwrap_or(serde_yaml::Value::Bool(false)).as_bool().unwrap();
        debug!("{}.new | retain: {:?}", self_id, retain);
        let subscribe = ConfSubscribe::new(self_conf.get_param_value("subscribe").unwrap_or(serde_yaml::Value::Null));
        debug!("{}.new | sudscribe: {:?}", self_id, subscribe);
        Self {
            name: self_name,
            retain,
            subscribe,
        }
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub(crate) fn from_yaml(parent: impl Into<String>, value: &serde_yaml::Value) -> CacheServiceConfig {
        match value.as_mapping().unwrap().into_iter().next() {
            Some((key, value)) => {
                Self::new(parent, &mut ConfTree::new(key.as_str().unwrap(), value.clone()))
            },
            None => {
                panic!("CacheServiceConfig.from_yaml | Format error or empty conf: {:#?}", value)
            },
        }        
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(parent: impl Into<String>, path: &str) -> CacheServiceConfig {
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        CacheServiceConfig::from_yaml(parent, &config)
                    },
                    Err(err) => {
                        panic!("CacheServiceConfig.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
                    },
                }
            },
            Err(err) => {
                panic!("CacheServiceConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
}
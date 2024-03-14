use std::{time::Duration, str::FromStr};
use log::{debug, trace};
use super::{conf_tree::ConfTree, conf_duration::ConfDuration, conf_keywd::{ConfKind, ConfKeywd}};
///
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ServiceConfig {
    id: String,
    pub key: String,
    conf: ConfTree,
    pub keys: Vec<String>,
}
///
/// 
impl ServiceConfig {
    ///
    /// Creates new instance of ServiceConfig
    pub fn new(parent: &str, conf: ConfTree) -> Self {
        let keys = conf.subNodes().unwrap().map(|conf| conf.key).collect();
        Self { 
            id: format!("{}/ServiceConfig", parent),
            key: conf.key.clone(),
            conf,
            keys,
        }
    }
    ///
    /// returns first sub node
    pub fn first(&self) -> Option<ConfTree> {
        self.conf.next()
    }
    ///
    /// 
    pub fn get(&self, key: &str) -> Option<ConfTree> {
        self.conf.get(key)
    }
    ///
    /// 
    fn remove_key(&mut self, name: &str) -> Result<(), String> {
        match self.keys.iter().position(|x| *x == name) {
            Some(index) => {
                self.keys.remove(index);
                Ok(())
            },
            None => Err(format!("{}.removeKey | '{}' - not found in: {:?}", self.id, name, self.conf)),
        }
    }
    ///
    /// 
    pub fn name(&self) -> String {
        match ConfKeywd::from_str(&self.conf.key) {
            Ok(self_keyword) => {
                trace!("{}.name | selfKeyword: {:?}", self.id, self_keyword);
                self_keyword.name()
            },
            Err(err) => panic!("{}.name | Keyword error in {:?}\n\tdetales: {:?}", self.id, self.conf.key, err),
        }
    }
    ///
    /// 
    pub fn sufix(&self) -> String {
        match ConfKeywd::from_str(&self.conf.key) {
            Ok(self_keyword) => {
                trace!("{}.sufix | selfKeyword: {:?}", self.id, self_keyword);
                self_keyword.sufix()
            },
            Err(err) => panic!("{}.name | Keyword error in {:?}\n\tdetales: {:?}", self.id, self.conf.key, err),
        }
    }
    ///
    /// 
    pub fn get_param_value(&mut self, name: &str) -> Result<serde_yaml::Value, String> {
        match self.remove_key(name) {
            Ok(_) => {
                match self.conf.get(name) {
                    Some(conf_tree) => Ok(conf_tree.conf),
                    None => Err(format!("{}.getParam | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }
            },
            Err(err) => Err(err),
        }
    }
    ///
    /// 
    pub fn get_param_conf(&mut self, name: &str) -> Result<ConfTree, String> {
        match self.remove_key(name) {
            Ok(_) => {
                match self.conf.get(name) {
                    Some(conf_tree) => Ok(conf_tree),
                    None => Err(format!("{}.getParam | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }
            },
            Err(err) => Err(err),
        }
    }
    ///
    /// 
    pub fn get_duration(&mut self, name: &str) -> Option<Duration> {
        match self.get_param_value(name) {
            Ok(value) => {
                let value = if value.is_u64() {
                    value.as_u64().unwrap().to_string()
                } else if value.is_string() {
                    value.as_str().unwrap().to_string()
                } else {
                    panic!("{}.getDuration | Invalid {} duration format: {:?} \n\tin: {:?}", self.id, &name, &value, self.conf)
                };
                match ConfDuration::from_str(&value) {
                    Ok(conf_duration) => {
                        Some(conf_duration.toDuration())
                    },
                    Err(err) => panic!("{}.getDuration | Parse {} duration '{}' error: {:?}", self.id, &name, &value, err),
                }
            },
            Err(_) => None,
        }
    }
    ///
    /// 
    pub fn get_param_by_keyword(&mut self, keyword_prefix: &str, keyword_kind: ConfKind) -> Result<(ConfKeywd, ConfTree), String> {
        let self_conf = self.conf.clone();
        for node in self_conf.subNodes().unwrap() {
            if let Ok(keyword) = ConfKeywd::from_str(&node.key) {
                if keyword.kind() == keyword_kind && keyword.prefix() == keyword_prefix {
                    match self.remove_key(&node.key) {
                        Ok(_) => return Ok((keyword, node)),
                        Err(err) => return Err(err),
                    };
                }
            };
        };
        Err(format!("{}.getParamByKeyword | keyword '{} {:?}' - not found", self.id, keyword_prefix, keyword_kind))
    }
    ///
    /// 
    pub fn get_in_queue(&mut self) -> Result<(String, i64), String> {
        let prefix = "in";
        let sub_param = "max-length";
        match self.get_param_by_keyword(prefix, ConfKind::Queue) {
            Ok((keyword, self_recv_queue)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                debug!("{}.getQueue | self in-queue params {}: {:?}", self.id, name, self_recv_queue);
                let max_length = match self_recv_queue.get(sub_param) {
                    Some(conf_tree) => Ok(conf_tree.conf),
                    None => Err(format!("{}.getQueue | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }.unwrap().as_i64().unwrap();
                Ok((keyword.name(), max_length))
            },
            Err(err) => Err(format!("{}.getQueue | {} queue - not found in: {:?}\n\terror: {:?}", self.id, prefix, self.conf, err)),
        }        
    }    
    ///
    /// 
    pub fn get_out_queue(&mut self) -> Result<String, String> {
        let prefix = "out";
        match self.get_param_by_keyword(prefix, ConfKind::Queue) {
            Ok((keyword, tx_name)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                debug!("{}.getQueue | self out-queue params {}: {:?}", self.id, name, tx_name);
                Ok(tx_name.conf.as_str().unwrap().to_string())
            },
            Err(err) => Err(format!("{}.getQueue | {} queue - not found in: {:?}\n\terror: {:?}", self.id, prefix, self.conf, err)),
        }        
    }    
}

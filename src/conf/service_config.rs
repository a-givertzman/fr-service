#![allow(non_snake_case)]

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
            conf: conf,
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
    fn removeKey(&mut self, name: &str) -> Result<(), String> {
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
            Ok(selfKeyword) => {
                trace!("{}.name | selfKeyword: {:?}", self.id, selfKeyword);
                selfKeyword.name()
            },
            Err(err) => panic!("{}.name | Unknown metric name in {:?}\n\tdetales: {:?}", self.id, self.conf.key, err),
        }
    }
    ///
    /// 
    pub fn getParam(&mut self, name: &str) -> Result<serde_yaml::Value, String> {
        match self.removeKey(name) {
            Ok(_) => {
                match self.conf.get(name) {
                    Some(confTree) => Ok(confTree.conf),
                    None => Err(format!("{}.getParam | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }
            },
            Err(err) => Err(err),
        }
    }
    ///
    /// 
    pub fn getDuration(&mut self, name: &str) -> Option<Duration> {
        match self.getParam(name) {
            Ok(value) => {
                match value.as_str() {
                    Some(value) => {
                        match ConfDuration::from_str(value) {
                            Ok(confDuration) => {
                                Some(confDuration.toDuration())
                            },
                            Err(err) => panic!("{}.getDuration | Parse {} duration '{}' error: {:?}", self.id, &name, &value, err),
                        }
                    },
                    None => panic!("{}.getDuration | Invalid reconnect {} duration format: {:?} \n\tin: {:?}", self.id, &name, &value, self.conf),
                }
            },
            Err(_) => None,
        }
    }
    ///
    /// 
    pub fn getParamByKeyword(&mut self, keywordPrefix: &str, keywordKind: ConfKind) -> Result<(ConfKeywd, ConfTree), String> {
        let selfConf = self.conf.clone();
        for node in selfConf.subNodes().unwrap() {
            match ConfKeywd::from_str(&node.key) {
                Ok(keyword) => {
                    if keyword.kind() == keywordKind && keyword.prefix() == keywordPrefix {
                        match self.removeKey(&node.key) {
                            Ok(_) => return Ok((keyword, node)),
                            Err(err) => return Err(err),
                        };
                    }
                },
                Err(_) => {},
            };
        };
        Err(format!("{}.getParamByKeyword | keyword '{} {:?}' - not found", self.id, keywordPrefix, keywordKind))
    }
    ///
    /// 
    pub fn getInQueue(&mut self) -> Result<(String, i64), String> {
        let prefix = "in";
        let subParam = "max-length";
        match self.getParamByKeyword(prefix, ConfKind::Queue) {
            Ok((keyword, selfRecvQueue)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                debug!("{}.getQueue | self in-queue params {}: {:?}", self.id, name, selfRecvQueue);
                let maxLength = match selfRecvQueue.get(subParam) {
                    Some(confTree) => Ok(confTree.conf),
                    None => Err(format!("{}.getQueue | '{}' - not found in: {:?}", self.id, name, self.conf)),
                }.unwrap().as_i64().unwrap();
                Ok((keyword.name(), maxLength))
            },
            Err(err) => Err(format!("{}.getQueue | {} queue - not found in: {:?}\n\terror: {:?}", self.id, prefix, self.conf, err)),
        }        
    }    
    ///
    /// 
    pub fn getOutQueue(&mut self) -> Result<String, String> {
        let prefix = "out";
        match self.getParamByKeyword(prefix, ConfKind::Queue) {
            Ok((keyword, txName)) => {
                let name = format!("{} {} {}", keyword.prefix(), keyword.kind().to_string(), keyword.name());
                debug!("{}.getQueue | self out-queue params {}: {:?}", self.id, name, txName);
                Ok(txName.conf.as_str().unwrap().to_string())
            },
            Err(err) => Err(format!("{}.getQueue | {} queue - not found in: {:?}\n\terror: {:?}", self.id, prefix, self.conf, err)),
        }        
    }    
}

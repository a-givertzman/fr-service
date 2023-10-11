#![allow(non_snake_case)]
// use log::debug;


// pub enum ConfTreeNode {
//     String(serde_yaml::Value),
//     Map(serde_yaml::Value),
// }

#[derive(Clone, Debug, PartialEq, Eq)]
///
/// iterate across all yaml config nodes
pub struct ConfTree {
    pub key: String,
    pub conf: serde_yaml::Value,
    // iter: Option<impl Iterator<Item = ConfTree> + '_>,
}
impl ConfTree {
    ///
    /// creates iterotor on the serde_yaml::Value mapping
    pub fn new(conf: serde_yaml::Value) -> Self {
        Self {
            key: String::new(),
            conf,
        }
    }
    ///
    /// 
    fn newSub(key: String, conf: serde_yaml::Value) -> Self {
        Self {
            key: key,
            conf,
        }
    }
    ///
    /// iterates across all sub nodes 
    pub fn next(&self) -> Option<ConfTree> {
        match self.subNodes() {
            Some(mut subNodes) => subNodes.next(),
            None => None,
        }
    }
    ///
    /// returns count of sub nodes
    pub fn count(&self) -> usize {
        match self.subNodes() {
            Some(subNodes) => subNodes.count(),
            None => 0,
        }
    }
    ///
    /// iterate across all sub nodes
    pub fn subNodes(&self) -> Option<impl Iterator<Item = ConfTree> + '_> {
        if self.conf.is_mapping() {
            let iter = self.conf.as_mapping().unwrap().into_iter().map( |(key, value)| {
                ConfTree::newSub(
                    key.as_str().unwrap().to_string(),
                    value.clone(),
                )
            });
            Some(
                iter
            )
        } else {
            None
        }
    }
    ///
    /// returns tree node by it's key if exists
    pub fn get(&self, key: &str) -> Option<ConfTree> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                Some(ConfTree {
                    key: key.to_string(),
                    conf: value.clone(),
                })
            },
            None => None,
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn asBool(&self, key: &str) -> Result<bool, String> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                match value.as_bool() {
                    Some(value) => {Ok(value)},
                    None => Err(format!("error getting bool by key '{:?}' from node '{:?}'", &key, &self.conf)),
                }
            },
            None => Err(format!("key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn asI64(&self, key: &str) -> Result<i64, String> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                match value.as_i64() {
                    Some(value) => {Ok(value)},
                    None => Err(format!("error getting int by key '{:?}' from node '{:?}'", &key, &self.conf)),
                }
            },
            None => Err(format!("key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn asf64(&self, key: &str) -> Result<f64, String> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                match value.as_f64() {
                    Some(value) => {Ok(value)},
                    None => Err(format!("error getting float by key '{:?}' from node '{:?}'", &key, &self.conf)),
                }
            },
            None => Err(format!("key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn asStr(&self, key: &str) -> Result<&str, String> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                match value.as_str() {
                    Some(value) => {Ok(value)},
                    None => Err(format!("error getting float by key '{:?}' from node '{:?}'", &key, &self.conf)),
                }
            },
            None => Err(format!("key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
}

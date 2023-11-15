#![allow(non_snake_case)]
// use log::debug;


// pub enum ConfTreeNode {
//     String(serde_yaml::Value),
//     Map(serde_yaml::Value),
// }

///
/// ConfTree holds sede_yaml::Value and it key
/// for root key = ""
/// Allow to iterate across all yaml config nodes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfTree {
    pub key: String,
    pub conf: serde_yaml::Value,
}
impl ConfTree {
    ///
    /// creates iterotor on the serde_yaml::Value mapping
    pub fn newRoot(conf: serde_yaml::Value) -> Self {
        Self {
            key: String::new(),
            conf,
        }
    }
    ///
    /// creates ConfTree instance holding the key and serde_yaml::Value
    fn newSub(key: String, conf: serde_yaml::Value) -> Self {
        Self {key, conf}
    }
    ///
    /// returns true if holding mapping 
    pub fn isMapping(&self) -> bool {
        self.conf.is_mapping()
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
                    None => Err(format!("error getting BOOL by key '{:?}' from node '{:?}'", &key, value)),
                }
            },
            None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn asI64(&self, key: &str) -> Result<i64, String> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                match value.as_i64() {
                    Some(value) => {Ok(value)},
                    None => Err(format!("error getting INT by key '{:?}' from node '{:?}'", &key, value)),
                }
            },
            None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn asF64(&self, key: &str) -> Result<f64, String> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                match value.as_f64() {
                    Some(value) => {Ok(value)},
                    None => Err(format!("error getting FLOAT by key '{:?}' from node '{:?}'", &key, value)),
                }
            },
            None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
    ///
    /// returns tree node value as bool by it's key if exists
    pub fn asStr(&self, key: &str) -> Result<&str, String> {
        match self.conf.as_mapping().unwrap().get(key) {
            Some(value) => {
                match value.as_str() {
                    Some(value) => {Ok(value)},
                    None => Err(format!("Error getting STRING by key '{:?}' from node '{:?}'", &key, value)),
                }
            },
            None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
    ///
    /// removes node by it's key if exists
    /// returns Result<&Self>
    pub fn remove(&mut self, key: &str) -> Result<serde_yaml::Value, String> {
        match self.conf.as_mapping_mut().unwrap().remove(key) {
            Some(value) => Ok(value),
            None => Err(format!("Key '{:?}' not found in the node '{:?}'", &key, &self.conf)),
        }
    }
}

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
    /// 
    pub fn new(conf: serde_yaml::Value) -> Self {
        Self {
            key: String::new(),
            conf,
        }
    }
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
    /// 
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
}

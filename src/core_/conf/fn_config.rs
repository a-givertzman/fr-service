use log::{trace, debug, error};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core_::{conf::fn_conf_keywd::FnConfKeywd, conf::conf_tree::ConfTree};

use super::fn_config_type::FnConfigType;


#[derive(Debug, PartialEq)]
///
/// creates config read from yaml file of following format:
/// ```yaml
/// let VarName2:
///     input fn functionName:
///         initial: VarName1
///         input functionName:
///             input1: const someValue
///             input2: point '/path/Point.Name/'
///             input fn functionName:
///                 input: point '/path/Point.Name/'```
pub struct FnConfig {
    pub fnType: FnConfigType,
    pub name: String,
    pub inputs: HashMap<String, FnConfig>,
}
///
/// 
impl FnConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// let VarName2:
    ///     input fn functionName:
    ///         initial: VarName1
    ///         input functionName:
    ///             input1: const someValue
    ///             input2: point '/path/Point.Name/'
    ///             input fn functionName:
    ///                 input: point '/path/Point.Name/'```
    pub fn new(confTree: ConfTree, vars: &mut Vec<String>) -> FnConfig {
        println!("\n");
        trace!("FnConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("FnConfig.new | FnConf must have single item, additional items was ignored")
        };
        match confTree.next() {
            Some(selfConf) => {
                debug!("FnConfig.new | MAPPING VALUE");
                trace!("FnConfig.new | selfConf: {:?}", selfConf);
                match FnConfKeywd::from_str(selfConf.key.as_str()) {
                    Ok(selfKeyword) => {
                        trace!("FnConfig.new | selfKeyword parsed: {:?}", selfKeyword);
                        // parse sub nodes
                        // let mut inputs = HashMap::new();
                        trace!("FnConfig.new | build inputs...");
                        if selfKeyword.type_() == FnConfigType::Var {
                            vars.push(selfKeyword.name())
                        }
                        let inputs = Self::buildInputs(selfConf, vars);
                        FnConfig {
                            fnType: selfKeyword.type_(),
                            name: selfKeyword.name(),
                            inputs: inputs,
                        }
                    },
                    Err(err) => {
                        panic!("FnConfig.new | keyword '{:?}' parsing error: {:?}", selfConf, err)
                        // trace!("FnConfig.new | input name detected: {:?}", selfConf.key);
                    },
                }
            },
            None => {
                debug!("FnConfig.new | SINGLE VALUE");
                match FnConfKeywd::from_str(confTree.conf.as_str().unwrap()) {
                    // keyword parsed successefully
                    //  - take input name and input Value / Fn from the keyword
                    Ok(fnKeyword) => {
                        FnConfig {
                            fnType: fnKeyword.type_(),
                            name: fnKeyword.name(),
                            inputs: HashMap::new(),
                        }
                    },
                    // no keyword 
                    //  - current node just an input name
                    //      - take input Value / Fn from first sub node,
                    //          if additional sub nodes prtesent, hit warning: "input must have single Value/Fn"
                    Err(_) => {
                        let varName = confTree.conf.as_str().unwrap().to_string();
                        if vars.contains(&varName) {
                            debug!("FnConfig.new | Variable declared - ok: {:?}", confTree.conf);
                            FnConfig { fnType: FnConfigType::Var, name: varName, inputs: HashMap::new() }
                        } else {
                            panic!("FnConfig.new | Variable not declared: {:?}", confTree.conf)
                        }
                    }
                }
            },
        }
    }
    ///
    /// 
    fn buildInputs(confTree: ConfTree, vars: &mut Vec<String>) ->HashMap<String, FnConfig> {
        let mut inputs = HashMap::new();
        match confTree.subNodes() {
            // has inputs in mapping
            Some(subNodes) => {
                trace!("FnConfig.buildInputs | sub nodes - found");
                for subNode in subNodes {
                    trace!("FnConfig.buildInputs | sub node: {:?}", subNode);
                    match FnConfKeywd::from_str(subNode.key.as_str()) {
                        Ok(keyword) => {
                            trace!("FnConfig.buildInputs | sub node KEYWORD parsed: {:?}", keyword);
                            if !keyword.input().is_empty() {
                                inputs.insert(
                                    keyword.input(),
                                    FnConfig {fnType: keyword.type_(), name: keyword.name(), inputs: Self::buildInputs(subNode, vars)},
                                );
                            }
                        },
                        Err(_) => {
                            trace!("FnConfig.buildInputs | sub node NO KEYWORD");
                            inputs.insert(
                                subNode.key, 
                                FnConfig::fromYamlValue(&subNode.conf, vars),
                            );
                        },
                    };
                }
            },
            None => {
                panic!("FnConfig.buildInputs | sub node not found in confTree: {:?}", confTree);
            },
        }
        inputs
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub fn fromYamlValue(value: &serde_yaml::Value, vars: &mut Vec<String>) -> FnConfig {
        Self::new(ConfTree::new(value.clone()), vars)
    }
    ///
    /// reads config from path
    #[allow(dead_code)]
    pub fn read(path: &str) -> FnConfig {
        let mut vars = vec![];
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        FnConfig::fromYamlValue(&config, &mut vars)
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
}

// #[derive(Debug)]
// pub struct FnVarConfig {
//     pub value: FnConfig,
// }

// #[derive(Debug)]
// pub struct FnConstConfig {
//     pub value: String,
// }

// #[derive(Debug)]
// pub struct FnPointConfig {
//     pub value: String,
// }

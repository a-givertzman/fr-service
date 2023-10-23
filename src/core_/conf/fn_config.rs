#![allow(non_snake_case)]

use log::{trace, debug, error, warn};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core_::{conf::conf_keywd::ConfKeywd, conf::conf_tree::ConfTree};

use super::{fn_conf_kind::FnConfKind, conf_keywd::FnConfPointType};


// enum ValueType<'a> {
//     Single(&'a ConfTree),
//     Mapping(&'a ConfTree),
// }


///
/// creates config read from yaml file of following format:
/// ```yaml
/// let VarName2:
///     input fn functionName:
///         initial: VarName1
///         input fn functionName:
///             input1: const someValue
///             input2: point '/path/Point.Name/'
///             input fn functionName:
///                 input: point '/path/Point.Name/'```
#[derive(Debug, PartialEq, Clone)]
pub struct FnConfig {
    pub fnKind: FnConfKind,
    pub name: String,
    pub inputs: HashMap<String, FnConfig>,
    pub type_: FnConfPointType,
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
    ///         input fn functionName:
    ///             input1: const someValue
    ///             input2: point '/path/Point.Name/'
    ///             input fn functionName:
    ///                 input: point '/path/Point.Name/'```
    pub fn new(confTree: &ConfTree, vars: &mut Vec<String>) -> FnConfig {
        println!("\n");
        trace!("FnConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        if confTree.count() > 1 {
            error!("FnConfig.new | FnConf must have single item, additional items was ignored")
        };
        if confTree.isMapping() {
            debug!("FnConfig.new | MAPPING VALUE");
            trace!("FnConfig.new | confTree: {:?}", confTree);
            match ConfKeywd::from_str(confTree.key.as_str()) {
                Ok(selfKeyword) => {
                    trace!("FnConfig.new | selfKeyword parsed: {:?}", selfKeyword);
                    // parse sub nodes
                    // let mut inputs = HashMap::new();
                    trace!("FnConfig.new | build inputs...");
                    let fnName: String;
                    let inputs: HashMap<String, FnConfig>;
                    match selfKeyword.kind() {
                        FnConfKind::Const => {
                            fnName = if selfKeyword.data().is_empty() {
                                confTree.conf.as_str().unwrap().to_string()
                            } else {
                                selfKeyword.data()
                            };
                            inputs = HashMap::new();
                        },
                        FnConfKind::Var => {
                            vars.push(selfKeyword.data());
                            fnName = selfKeyword.data();
                            inputs = Self::buildInputs(confTree, vars);
                        },
                        _ => {
                            fnName = selfKeyword.data();
                            inputs = Self::buildInputs(confTree, vars);
                        },
                    }
                    FnConfig {
                        fnKind: selfKeyword.kind(),
                        name: fnName,
                        inputs: inputs,
                        type_: selfKeyword.type_(),
                    }
                },
                // no keyword 
                //  - current node just an input name
                //      - take input Value / Fn from first sub node,
                //          if additional sub nodes prtesent, hit warning: "input must have single Value/Fn"
                Err(err) => {
                    panic!("FnConfig.new | keyword '{:?}' parsing error: {:?}", confTree, err)
                    // trace!("FnConfig.new | input name detected: {:?}", confTree.key);
                },
            }
        } else {
            debug!("FnConfig.new | SINGLE VALUE\t{:?}", &confTree.conf);
            if confTree.conf.is_string() {
                match ConfKeywd::from_str(confTree.conf.as_str().unwrap()) {
                    // keyword parsed successefully
                    //  - take input name and input Value / Fn from the keyword
                    Ok(fnKeyword) => {
                        match fnKeyword {
                            // ConfKeywd::Var(_) => {
                                
                            // },
                            ConfKeywd::Const(_) => {
                                FnConfig {
                                    fnKind: fnKeyword.kind(),
                                    name: fnKeyword.data(),
                                    inputs: HashMap::new(),
                                    type_: fnKeyword.type_(),
                                }
    
                            },
                            ConfKeywd::Point(_) => {
                                FnConfig {
                                    fnKind: fnKeyword.kind(),
                                    name: fnKeyword.data(),
                                    inputs: HashMap::new(),
                                    type_: fnKeyword.type_(),
                                }
    
                            },
                            _ => {
                                panic!("FnConfig.new | Unknown keyword: {:?}", confTree.conf)
                            },
                        }
                    },
                    // no keyword 
                    //  - current node just an varible name
                    //  - or custom parameter
                    Err(_) => {
                        let varName = confTree.conf.as_str().unwrap().to_string();
                        if vars.contains(&varName) {
                            debug!("FnConfig.new | Variable declared - ok: {:?}", confTree.conf);
                            FnConfig { 
                                fnKind: FnConfKind::Var, 
                                name: varName, 
                                inputs: HashMap::new(),
                                type_: FnConfPointType::Unknown,
                            }
                        } else {
                            debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                            FnConfig { 
                                fnKind: FnConfKind::Param, 
                                name: varName, 
                                inputs: HashMap::new(),
                                type_: FnConfPointType::Unknown,
                            }
                            // panic!("FnConfig.new | Variable not declared: {:?}", confTree.conf)
                        }
                    }
                }
            } else if confTree.conf.is_bool() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                let varName = confTree.conf.as_bool().unwrap().to_string();
                FnConfig { 
                    fnKind: FnConfKind::Param, 
                    name: varName, 
                    inputs: HashMap::new(),
                    type_: FnConfPointType::Unknown,
                }
            } else if confTree.conf.is_i64() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                let varName = confTree.conf.as_i64().unwrap().to_string();
                FnConfig { 
                    fnKind: FnConfKind::Param, 
                    name: varName, 
                    inputs: HashMap::new(),
                    type_: FnConfPointType::Unknown,
                }
            } else if confTree.conf.is_f64() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                let varName = confTree.conf.as_f64().unwrap().to_string();
                FnConfig { 
                    fnKind: FnConfKind::Param, 
                    name: varName, 
                    inputs: HashMap::new(),
                    type_: FnConfPointType::Unknown,
                }
            } else {
                panic!("FnConfig.new | Custom parameter of unknown type declared, but : {:?}", confTree.conf);
            }
        }
    }
    ///
    /// 
    fn buildInputs(confTree: &ConfTree, vars: &mut Vec<String>) ->HashMap<String, FnConfig> {
        let mut inputs = HashMap::new();
        match confTree.subNodes() {
            // has inputs in mapping
            Some(subNodes) => {
                trace!("FnConfig.buildInputs | sub nodes - found");
                for subNode in subNodes {
                    trace!("FnConfig.buildInputs | sub node: {:?}", subNode);
                    // inputs.insert(
                    //     (&subNode).key.clone(), 
                    //     FnConfig::new(&subNode, vars),
                    // );

                    match ConfKeywd::from_str(subNode.key.as_str()) {
                        Ok(keyword) => {
                            trace!("FnConfig.buildInputs | sub node KEYWORD parsed: {:?}", keyword);
                            if !keyword.input().is_empty() {
                                inputs.insert(
                                    keyword.input(),
                                    FnConfig {
                                        fnKind: keyword.kind(), 
                                        name: keyword.data(), 
                                        inputs: Self::buildInputs(&subNode, vars),
                                        type_: keyword.type_(),
                                    },
                                );
                            }
                        },
                        Err(_) => {
                            trace!("FnConfig.buildInputs | sub node NO KEYWORD");
                            inputs.insert(
                                (&subNode).key.clone(), 
                                FnConfig::new(&subNode, vars),
                            );
                        },
                    };
                }
            },
            None => {
                trace!("FnConfig.buildInputs | sub node not found, possible Const or Var");
                inputs.insert(
                    (&confTree).key.clone(), 
                    FnConfig::new(&confTree, vars),
                );
                // panic!("FnConfig.buildInputs | sub node not found in confTree: {:?}", confTree);
            },
        }
        inputs
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub fn fromYamlValue(value: &serde_yaml::Value, vars: &mut Vec<String>) -> FnConfig {
        Self::new(&ConfTree::newRoot(value.clone()).next().unwrap(), vars)
    }
    ///
    /// reads yaml config from path
    /// ```yaml
    /// let VarName2:
    ///     input fn functionName:
    ///         initial: VarName1
    ///         input fn functionName:
    ///             input1: const someValue
    ///             input2: point '/path/Point.Name/'
    ///             input fn functionName:
    ///                 input: point '/path/Point.Name/'```
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
    ///
    /// returns input config by itc name
    pub fn inputConf<'a>(&'a mut self, inputName: &str) -> &'a mut FnConfig {
        match self.inputs.get_mut(inputName) {
            Some(conf) => conf,
            None => panic!("FnConfig.inputConf | function {:?} must have {:?}", self.name, inputName),
        }
    }
    ///
    /// returns custom parameter by it's name if exists, else none
    pub fn param(&self, name: &str) -> &FnConfig {
        match self.inputs.get(name) {
            Some(param) => param,
            None => {
                panic!("FnConfig.param | parameter {:?} not fount in the {:?}", name, self.name);
            },
        }
    }
}

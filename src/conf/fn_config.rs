#![allow(non_snake_case)]

use indexmap::IndexMap;
use log::{trace, debug};
use std::{fs, str::FromStr};

use crate::conf::{
        fn_conf_keywd::FnConfKeywd, conf_tree::ConfTree, fn_point_config::FnPointConfig,
        fn_conf_kind::FnConfKind, fn_conf_keywd::FnConfPointType, point_config::point_config::PointConfig,
        fn_conf_keywd::FnConfKindName,
    };


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
    pub name: String,
    pub inputs: IndexMap<String, FnConfKind>,
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
    pub fn new(parent: &str, confTree: &ConfTree, vars: &mut Vec<String>) -> FnConfKind {
        println!("\n");
        trace!("FnConfig.new | confTree: {:?}", confTree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        // if confTree.count() > 1 {
        //     error!("FnConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        // };
        if confTree.isMapping() {
            debug!("FnConfig.new | MAPPING VALUE");
            trace!("FnConfig.new | confTree: {:?}", confTree);
            match FnConfKeywd::from_str(confTree.key.as_str()) {
                Ok(selfKeyword) => {
                    trace!("FnConfig.new | selfKeyword parsed: {:?}", selfKeyword);
                    // parse sub nodes
                    // let mut inputs = IndexMap::new();
                    trace!("FnConfig.new | build inputs...");
                    match selfKeyword {
                        FnConfKeywd::Const(value) => {
                            let fnName = if value.data.is_empty() {
                                confTree.conf.as_str().unwrap().to_string()
                            } else {
                                value.data
                            };
                            FnConfKind::Const(
                                FnConfig {
                                    name: fnName,
                                    inputs: IndexMap::new(),
                                    type_: value.type_,
                                }        
                            )
                        },
                        FnConfKeywd::Var(value) => {
                            vars.push(value.data.clone());
                            FnConfKind::Var(
                                FnConfig {
                                    name: value.data,
                                    inputs: Self::buildInputs(parent, confTree, vars),
                                    type_: value.type_,
                                }
                            )        
                        },
                        FnConfKeywd::Fn(value) => {
                            FnConfKind::Fn(
                                FnConfig {
                                    name: value.data,
                                    inputs: Self::buildInputs(parent, confTree, vars),
                                    type_: value.type_,
                                }
                            )
                        },
                        FnConfKeywd::Point(value) => {
                            debug!("FnConfig.new | Point: {:?}", value);
                            let result = Self::getParamByKeyword(confTree, "input", FnConfKindName::Const | FnConfKindName::Fn | FnConfKindName::Var | FnConfKindName::Point);
                            debug!("FnConfig.new | Point input: {:?}", result);
                            let inputConf = match result {
                                Ok(conf) => {
                                    // debug!("FnConfig.new | Point input keyword: {:?}", keyword);
                                    conf
                                    // match conf.get(&keyword.input()) {
                                    //     Some(conf) => conf,
                                    //     None => panic!("FnConfig.new | Point.input - can't be empty in: {:?}", confTree),
                                    // }
                                },
                                Err(_) => panic!("FnConfig.new | Point.input - not found in: {:?}", confTree),
                            };
                            FnConfKind::PointConf(
                                FnPointConfig {
                                    conf: PointConfig::new(parent, confTree),
                                    input: Box::new(FnConfig::new(parent, &inputConf, vars)),
                                }
                            )
                        },
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
                match FnConfKeywd::from_str(confTree.conf.as_str().unwrap()) {
                    // keyword parsed successefully
                    //  - take input name and input Value / Fn from the keyword
                    Ok(fnKeyword) => {
                        match fnKeyword {
                            FnConfKeywd::Const(value) => {
                                FnConfKind::Const(
                                    FnConfig {
                                        name: value.data,
                                        inputs: IndexMap::new(),
                                        type_: value.type_,
                                    }
                                )
                            },
                            FnConfKeywd::Point(value) => {
                                FnConfKind::Point(
                                    FnConfig {
                                        name: value.data,
                                        inputs: IndexMap::new(),
                                        type_: value.type_,
                                    }
                                )
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
                        debug!("FnConfig.new | trying to find Variable: {:?} in vars: \n\t{:?}", &varName, &vars);
                        if vars.contains(&varName) {
                            debug!("FnConfig.new | Variable declared - ok: {:?}", confTree.conf);
                            FnConfKind::Var(
                                FnConfig { 
                                    name: varName, 
                                    inputs: IndexMap::new(),
                                    type_: FnConfPointType::Unknown,
                                }
                            )
                        } else {
                            debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                            FnConfKind::Param(varName)
                            // panic!("FnConfig.new | Variable not declared: {:?}", confTree.conf)
                        }
                    }
                }
            } else if confTree.conf.is_bool() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                let varName = confTree.conf.as_bool().unwrap().to_string();
                FnConfKind::Param(varName)
            } else if confTree.conf.is_i64() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                let varName = confTree.conf.as_i64().unwrap().to_string();
                FnConfKind::Param(varName)
            } else if confTree.conf.is_f64() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", confTree.conf);
                let varName = confTree.conf.as_f64().unwrap().to_string();
                FnConfKind::Param(varName)
            } else {
                panic!("FnConfig.new | Custom parameter of unknown type declared, but : {:?}", confTree.conf);
            }
        }
    }
    ///
    /// 
    fn buildInputs(parent: &str, confTree: &ConfTree, vars: &mut Vec<String>) -> IndexMap<String, FnConfKind> {
        let mut inputs = IndexMap::new();
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
                                    FnConfig::new(parent, &subNode, vars),
                                );
                            }
                        },
                        Err(_) => {
                            trace!("FnConfig.buildInputs | sub node NO KEYWORD");
                            inputs.insert(
                                (&subNode).key.clone(), 
                                FnConfig::new(parent, &subNode, vars),
                            );
                        },
                    };
                }
            },
            None => {
                trace!("FnConfig.buildInputs | sub node not found, possible Const or Var");
                inputs.insert(
                    (&confTree).key.clone(), 
                    FnConfig::new(parent, &confTree, vars),
                );
            },
        }
        inputs
    }
    ///
    /// creates config from serde_yaml::Value of following format:
    pub fn from_yaml(parent: &str, value: &serde_yaml::Value, vars: &mut Vec<String>) -> FnConfKind {
        Self::new(parent, &ConfTree::newRoot(value.clone()).next().unwrap(), vars)
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
    pub fn read(parent: &str, path: &str) -> FnConfKind {
        let mut vars = vec![];
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        FnConfig::from_yaml(parent, &config, &mut vars)
                    },
                    Err(err) => {
                        panic!("FnConfig.read | Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
            },
            Err(err) => {
                panic!("FnConfig.read | File {} reading error: {:?}", path, err)
            },
        }
    }
    ///
    /// returns input config by itc name
    pub fn inputConf<'a>(&'a mut self, inputName: &str) -> &'a mut FnConfKind {
        match self.inputs.get_mut(inputName) {
            Some(conf) => conf,
            None => panic!("FnConfig.inputConf | function {:?} must have {:?}", self.name, inputName),
        }
    }
    ///
    /// returns custom parameter by it's name if exists, else none
    pub fn param(&self, name: &str) -> &FnConfKind {
        match self.inputs.get(name) {
            Some(param) => param,
            None => {
                panic!("FnConfig.param | parameter {:?} not fount in the {:?}", name, self.name);
            },
        }
    }
    ///
    /// 
    fn getParamByKeyword(conf: &ConfTree, input: &str, kind: u8) -> Result<ConfTree, String> {
        debug!("FnConfig.getParamByKeyword | conf: {:?}", conf);
        for node in conf.subNodes().unwrap() {
            debug!("FnConfig.getParamByKeyword | node: {:?}", node);
            match FnConfKeywd::from_str(&node.key) {
                Ok(keyword) => {
                    debug!("FnConfig.getParamByKeyword | keyword: {:?}, kind: {:?}", keyword, keyword.kind());
                    debug!("FnConfig.getParamByKeyword | keyword.kind({}) & kind({}): {:?}", (keyword.kind() as u8), kind, (keyword.kind() as u8) & kind);
                    if ((keyword.kind() as u8) & kind) > 0 && keyword.input() == input {
                        return Ok(node)
                    }
                },
                Err(_) => {
                    if node.key == input {
                        return Ok(node)
                    }
                },
            };
        };
        // Err(format!("{}.getParamByKeyword | keyword '{} {:?}' - not found", self.id, keywordPrefix, keywordKind))
        Err(format!("FnConfig.getParamByKeyword | keyword '{}' kind: {:?} - not found", input, kind))
    }
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        let mut points = vec![];
        for (_, inputKind) in &self.inputs {
            let mut inputPoints = inputKind.points();
            points.append(&mut inputPoints);
        }
        points
    }
}

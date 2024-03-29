use indexmap::IndexMap;
use log::{trace, debug};
use std::{fs, str::FromStr};

use crate::conf::{
    conf_tree::ConfTree, 
    point_config::point_config::PointConfig,
    fn_::{
        fn_conf_keywd::FnConfKeywd, fn_point_config::FnPointConfig,
        fn_conf_kind::FnConfKind, fn_conf_keywd::FnConfPointType, 
        fn_conf_keywd::FnConfKindName,
    },
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
    pub fn new(parent: &str, conf_tree: &ConfTree, vars: &mut Vec<String>) -> FnConfKind {
        println!();
        trace!("FnConfig.new | confTree: {:?}", conf_tree);
        // self conf from first sub node
        //  - if additional sub nodes presents hit warning, FnConf must have single item
        // if confTree.count() > 1 {
        //     error!("FnConfig.new | FnConf must have single item, additional items was ignored: {:?}", confTree)
        // };
        if conf_tree.isMapping() {
            debug!("FnConfig.new | MAPPING VALUE");
            trace!("FnConfig.new | confTree: {:?}", conf_tree);
            match FnConfKeywd::from_str(conf_tree.key.as_str()) {
                Ok(self_keyword) => {
                    trace!("FnConfig.new | selfKeyword parsed: {:?}", self_keyword);
                    // parse sub nodes
                    // let mut inputs = IndexMap::new();
                    trace!("FnConfig.new | build inputs...");
                    match self_keyword {
                        FnConfKeywd::Const(value) => {
                            let fn_name = if value.data.is_empty() {
                                conf_tree.conf.as_str().unwrap().to_string()
                            } else {
                                value.data
                            };
                            FnConfKind::Const(
                                FnConfig {
                                    name: fn_name,
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
                                    inputs: Self::build_inputs(parent, conf_tree, vars),
                                    type_: value.type_,
                                }
                            )        
                        },
                        FnConfKeywd::Fn(value) => {
                            FnConfKind::Fn(
                                FnConfig {
                                    name: value.data,
                                    inputs: Self::build_inputs(parent, conf_tree, vars),
                                    type_: value.type_,
                                }
                            )
                        },
                        FnConfKeywd::Point(value) => {
                            debug!("FnConfig.new | Point: {:?}", value);
                            let result = Self::get_param_by_keyword(conf_tree, "input", FnConfKindName::Const | FnConfKindName::Fn | FnConfKindName::Var | FnConfKindName::Point);
                            debug!("FnConfig.new | Point input: {:?}", result);
                            let input_conf = match result {
                                Ok(conf) => {
                                    // debug!("FnConfig.new | Point input keyword: {:?}", keyword);
                                    conf
                                    // match conf.get(&keyword.input()) {
                                    //     Some(conf) => conf,
                                    //     None => panic!("FnConfig.new | Point.input - can't be empty in: {:?}", confTree),
                                    // }
                                },
                                Err(_) => panic!("FnConfig.new | Point.input - not found in: {:?}", conf_tree),
                            };
                            FnConfKind::PointConf(
                                FnPointConfig {
                                    conf: PointConfig::new(parent, conf_tree),
                                    input: Box::new(FnConfig::new(parent, &input_conf, vars)),
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
                    panic!("FnConfig.new | keyword '{:?}' parsing error: {:?}", conf_tree, err)
                    // trace!("FnConfig.new | input name detected: {:?}", confTree.key);
                },
            }
        } else {
            debug!("FnConfig.new | SINGLE VALUE\t{:?}", &conf_tree.conf);
            if conf_tree.conf.is_string() {
                match FnConfKeywd::from_str(conf_tree.conf.as_str().unwrap()) {
                    // keyword parsed successefully
                    //  - take input name and input Value / Fn from the keyword
                    Ok(fn_keyword) => {
                        match fn_keyword {
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
                                panic!("FnConfig.new | Unknown keyword: {:?}", conf_tree.conf)
                            },
                        }
                    },
                    // no keyword 
                    //  - current node just an varible name
                    //  - or custom parameter
                    Err(_) => {
                        let var_name = conf_tree.conf.as_str().unwrap().to_string();
                        debug!("FnConfig.new | trying to find Variable: {:?} in vars: \n\t{:?}", &var_name, &vars);
                        if vars.contains(&var_name) {
                            debug!("FnConfig.new | Variable declared - ok: {:?}", conf_tree.conf);
                            FnConfKind::Var(
                                FnConfig { 
                                    name: var_name, 
                                    inputs: IndexMap::new(),
                                    type_: FnConfPointType::Unknown,
                                }
                            )
                        } else {
                            debug!("FnConfig.new | Custom parameter declared: {:?}", conf_tree.conf);
                            FnConfKind::Param(var_name)
                            // panic!("FnConfig.new | Variable not declared: {:?}", confTree.conf)
                        }
                    }
                }
            } else if conf_tree.conf.is_bool() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", conf_tree.conf);
                let var_name = conf_tree.conf.as_bool().unwrap().to_string();
                FnConfKind::Param(var_name)
            } else if conf_tree.conf.is_i64() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", conf_tree.conf);
                let var_name = conf_tree.conf.as_i64().unwrap().to_string();
                FnConfKind::Param(var_name)
            } else if conf_tree.conf.is_f64() {
                debug!("FnConfig.new | Custom parameter declared: {:?}", conf_tree.conf);
                let var_name = conf_tree.conf.as_f64().unwrap().to_string();
                FnConfKind::Param(var_name)
            } else {
                panic!("FnConfig.new | Custom parameter of unknown type declared, but : {:?}", conf_tree.conf);
            }
        }
    }
    ///
    /// 
    fn build_inputs(parent: &str, conf_tree: &ConfTree, vars: &mut Vec<String>) -> IndexMap<String, FnConfKind> {
        let mut inputs = IndexMap::new();
        match conf_tree.subNodes() {
            // has inputs in mapping
            Some(sub_nodes) => {
                trace!("FnConfig.buildInputs | sub nodes - found");
                for sub_node in sub_nodes {
                    trace!("FnConfig.buildInputs | sub node: {:?}", sub_node);
                    match FnConfKeywd::from_str(sub_node.key.as_str()) {
                        Ok(keyword) => {
                            trace!("FnConfig.buildInputs | sub node KEYWORD parsed: {:?}", keyword);
                            if !keyword.input().is_empty() {
                                inputs.insert(
                                    keyword.input(),
                                    FnConfig::new(parent, &sub_node, vars),
                                );
                            }
                        },
                        Err(_) => {
                            trace!("FnConfig.buildInputs | sub node NO KEYWORD");
                            inputs.insert(
                                (sub_node).key.clone(), 
                                FnConfig::new(parent, &sub_node, vars),
                            );
                        },
                    };
                }
            },
            None => {
                trace!("FnConfig.buildInputs | sub node not found, possible Const or Var");
                inputs.insert(
                    (conf_tree).key.clone(), 
                    FnConfig::new(parent, conf_tree, vars),
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
        match fs::read_to_string(path) {
            Ok(yaml_string) => {
                match serde_yaml::from_str(&yaml_string) {
                    Ok(config) => {
                        FnConfig::from_yaml(parent, &config, &mut vars)
                    },
                    Err(err) => {
                        panic!("FnConfig.read | Error in config: {:?}\n\terror: {:?}", yaml_string, err)
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
    pub fn input_conf<'a>(&'a mut self, input_name: &str) -> &'a mut FnConfKind {
        match self.inputs.get_mut(input_name) {
            Some(conf) => conf,
            None => panic!("FnConfig.inputConf | function {:?} must have {:?}", self.name, input_name),
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
    fn get_param_by_keyword(conf: &ConfTree, input: &str, kind: u8) -> Result<ConfTree, String> {
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
        for (_, input_kind) in &self.inputs {
            let mut input_points = input_kind.points();
            points.append(&mut input_points);
        }
        points
    }
}

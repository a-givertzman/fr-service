use log::{error, trace, debug};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core::nested_function::fn_conf_keywd::FnConfKeywd;


#[derive(Debug)]
pub struct FnConfig {
    pub fnType: Option<FnConfigType>,
    pub name: String,
    pub inputs: HashMap<String, FnConfig>,
}
impl FnConfig {
    ///
    /// creates config from LinkedHashMap
    pub fn new(conf: &serde_yaml::Value) -> FnConfig {
        // match name {
            
        // }
        // trace!("FnConfig.new | conf: {:?}", conf);
        let mut fnType: Option<FnConfigType> = None;
        let mut fnName: String = String::new(); 
        let mut inputs = HashMap::new();
        if conf.is_string() {
            trace!("FnConfig.new | IS STRING");
        //     let fnConfStr: String = serde_yaml::from_value(conf.clone()).unwrap();
        //     let fnKeyword = FnConfKeywd::from_str(fnConfStr.as_str()).unwrap();
        //     trace!("FnConfig.new | key: '{}'   |   fnKeyword: {:?}   |   fnConfStr: {:?}", "---", fnKeyword, fnConfStr);
        } else {
            trace!("FnConfig.new | IS MAP");
            let fnConfMap: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(conf.clone()).unwrap();
            for (key, inputfnConfMap) in fnConfMap.clone() {
                // trace!("FnConfig.new | key: '{}'    |   conf: {:?}", key, inputfnConfMap);
                match FnConfKeywd::from_str(key.as_str()) {
                    Ok(fnKeyword) => {
                        trace!("FnConfig.new | IS KEYWD");
                        trace!("FnConfig.new | key: '{}'   |   fnKeyword: {:?}   |   inputfnConfMap: {:?}", key, fnKeyword, &inputfnConfMap);
                        fnName = key.clone();
                        let (fnTypeV, fnNameV, inputConfs) = Self::parseFn(fnKeyword, &inputfnConfMap);
                        fnType = Some(fnTypeV);
                        fnName = fnNameV;
                        for (inputName, inputConf) in inputConfs {
                            inputs.insert(inputName, inputConf);
                        }
                    },
                    Err(_) => {
                        trace!("FnConfig.new | NO KEYWD");
                        trace!("FnConfig.new | key: '{}'   |   fnKeyword: {:?}   |   inputfnConfMap: {:?}", key, "---", &inputfnConfMap);
                        // let (inputName, inputfnConf) = Self::parseInput(&inputfnConfMap);
                        // trace!("FnConfig.new | inputName: '{}'", inputName);
                        // inputs.insert(key, inputfnConf);
                    },
                };
            }
        };
        // trace!("FnConfig.new | fnType: {:?} \t|\t fnName: {:?} \t|\t inputs: {:?}", fnType.as_ref(), fnName, inputs);
        FnConfig {
            fnType,
            name: fnName,
            inputs,
        }
    }

    fn parseFn(fnKeyword: FnConfKeywd, conf: &serde_yaml::Value) -> (FnConfigType, String, Vec<(String, FnConfig)>) {
        trace!("FnConfig.parseFn | ENTER");
        let (fnType, fnName) = match fnKeyword {
            // FnConfigKeyword::Const(name) => ( name, FnConfigType::Const(FnConstConfig { value: fnConfig.as_str().unwrap().into() }) ),
            // FnConfigKeyword::Point(name) => ( name, FnConfigType::Point(FnPointConfig { value: fnConfig.as_str().unwrap().into() }) ),
            FnConfKeywd::Var(name) => {
                (FnConfigType::Var, name)
            },
            FnConfKeywd::Fn(name) => {
                (FnConfigType::Fn, name)
            },
            _ => panic!("Unknown config: {:?}", conf),
        };
        let inputs = Self::parseInputs(conf);
        (fnType, fnName, inputs)
    }
    fn parseInputs(conf: &serde_yaml::Value) -> Vec<(String, FnConfig)> {
        trace!("FnConfig.parseInput | ENTER");
        let mut inputs = vec![];
        let inputsConfMap: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(conf.clone()).unwrap();
        for (inputName, inputConfMap) in inputsConfMap {
            let inputConf = FnConfig::new(&inputConfMap);
            inputs.push((inputName, inputConf));
        };
        inputs
        // if conf.is_string() {
        //     trace!("FnConfig.parseInput | IS STRING");
        //     panic!("FnConfig.parseInput | IS STRING: {:?}", conf);
        //     let fnConfStr: String = serde_yaml::from_value(conf.clone()).unwrap();
        //     let fnKeyword = FnConfKeywd::from_str(fnConfStr.as_str()).unwrap();
        //     trace!("FnConfig.parseInput | key: '{}'   |   fnKeyword: {:?}   |   fnConfStr: {:?}", "---", fnKeyword, fnConfStr);
        //     let (fnType, fnName, fnConf) = match fnKeyword {
        //         FnConfKeywd::Const(name) => {
        //             (FnConfigType::Const, name.clone(), FnConfig { fnType: Some(FnConfigType::Const), name, inputs: HashMap::new() })
        //         },
        //         FnConfKeywd::Point(name) => {
        //             (FnConfigType::Point, name.clone(), FnConfig { fnType: Some(FnConfigType::Point), name, inputs: HashMap::new() })
        //         },
        //         FnConfKeywd::Var(name) => {
        //             (FnConfigType::Var, name, FnConfig::new(&conf))
        //         },
        //         FnConfKeywd::Fn(name) => {
        //             (FnConfigType::Fn, name, FnConfig::new(&conf))
        //         },
        //         _ => panic!("FnConfig.parseInput | Unknown config: {:?}", conf),
        //     };
        //     vec![(fnName, fnConf)]
        // } else {
        //     trace!("FnConfig.new | IS MAP");
        //     let mut inputs = vec![];
        //     let inputsConfMap: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(conf.clone()).unwrap();
        //     for (inputName, inputConfMap) in inputsConfMap {
        //         let inputConf = FnConfig::new(&inputConfMap);
        //         inputs.push((inputName, inputConf));
        //     };
        //     inputs
        // }
    }
    ///
    /// reads config from path
    pub fn read(path: &str) -> FnConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        let conf: HashMap<String, serde_yaml::Value> = config;
                        // for (fnName, fnConfig) in conf {
                        //     trace!("FnConfig.new | {}: {:?}", fnName, fnConfig);
                        //     Self::new(&fnName, &conf)
                        // }                
                    },
                    Err(err) => {
                        panic!("Error in config: {:?}\n\terror: {:?}", yamlString, err)
                    },
                }
                // FnConfig::
                FnConfig {
                    fnType: None,
                    name: String::new(),
                    inputs: HashMap::new(),
                }                
            },
            Err(err) => {
                panic!("File {} reading error: {:?}", path, err)
            },
        }
    }
}

#[derive(Debug)]
pub struct FnVarConfig {
    pub value: FnConfig,
}

#[derive(Debug)]
pub struct FnConstConfig {
    pub value: String,
}

#[derive(Debug)]
pub struct FnPointConfig {
    pub value: String,
}


///
/// Config of type of the Function
#[derive(Debug)]
pub enum FnConfigType {
    Fn,
    Var,
    Const,
    Point,
}

// impl FnConfigType {
//     pub fn new(conf: &serde_yaml::Value) -> Result<FnConfigType, String> {
//         let confRef = &conf;
//         if confRef.is_string() {
//             // let confValue = conf.;
//             // let fnConfig: &str = serde_yaml::from_value(confValue).unwrap();
//             let fnKeyword = FnConfigKeyword::from_str(conf.as_str().unwrap()).unwrap();
//             match fnKeyword {
//                 FnConfigKeyword::Var(name) => Ok(FnConfigType::Var(FnVarConfig {  })),
//                 FnConfigKeyword::Const(name) => Ok(FnConfigType::Const(FnConstConfig { value: String::new() })),
//                 FnConfigKeyword::Point(name) => Ok(FnConfigType::Point(FnPointConfig {  })),
//                 _ => panic!("Unknown config: {:?}", conf),
//             }
//         } else {
//             let fnConfig: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(conf.clone()).unwrap();
//             let (fnKeyword, fnConfig) = fnConfig.iter().next().unwrap();
//             let fnKeyword = FnConfigKeyword::from_str(&fnKeyword).unwrap();
//             match fnKeyword {
//                 FnConfigKeyword::Fn(name) => Ok(FnConfigType::Fn(FnConfig::new(&name, &fnConfig))),
//                 _ => panic!("Unknown config: {:?}", confRef),
//             }
//         }
//     }
// }
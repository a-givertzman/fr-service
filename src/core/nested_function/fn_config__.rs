use log::{error, trace, debug};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct FnConfig {
    pub inputs: HashMap<String, FnConfigType>,
}
impl FnConfig {
    ///
    /// creates config from LinkedHashMap
    pub fn new(conf: &serde_yaml::Value) -> FnConfig {
        // match name {
            
        // }
        trace!("FnConfig.new | conf: {:?}", conf);
        let mut res = HashMap::new();
        if conf.is_string() {
            trace!("FnConfig.new | IS STRING");
            let fnConfig: String = serde_yaml::from_value(conf.clone()).unwrap();
            let fnKeyword = FnConfigKeyword::from_str(fnConfig.as_str()).unwrap();
            let (key, fnConfType) = match fnKeyword {
                FnConfigKeyword::Const(name) => ( name, FnConfigType::Const(FnConstConfig { value: fnConfig }) ),
                FnConfigKeyword::Point(name) => ( name, FnConfigType::Point(FnPointConfig { value: fnConfig }) ),
                _ => panic!("Unknown config: {:?}", conf),
            };
            trace!("FnConfig.new | key: '{}'    |   conf: {:?}", key, fnConfType);
            res.insert(key, fnConfType);
        } else {
            trace!("FnConfig.new | IS MAP");
            let fnConfig: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(conf.clone()).unwrap();
            for (key, fnConfig) in fnConfig {
                trace!("FnConfig.new | key: '{}'    |   conf: {:?}", key, fnConfig);
                match FnConfigKeyword::from_str(key.as_str()) {
                    Ok(fnKeyword) => {
                        trace!("FnConfig.new | fnKeyword: {:?}", fnKeyword);
                        let (_, fnConfType) = match fnKeyword {
                            FnConfigKeyword::Var(name) => ( name, FnConfigType::Var(FnVarConfig { value: FnConfig::new(&fnConfig) }) ),
                            // FnConfigKeyword::Const(name) => ( name, FnConfigType::Const(FnConstConfig { value: fnConfig.as_str().unwrap().into() }) ),
                            // FnConfigKeyword::Point(name) => ( name, FnConfigType::Point(FnPointConfig { value: fnConfig.as_str().unwrap().into() }) ),
                            FnConfigKeyword::Fn(name) => ( name, FnConfigType::Fn(FnConfig::new(&fnConfig)) ),
                            _ => panic!("Unknown config: {:?}", conf),
                        };
                        res.insert(key, fnConfType);
                    },
                    Err(_) => {
                        res.insert(key, FnConfigType::Fn(FnConfig::new(&fnConfig)));
                    },
                };
            }
        };
        trace!("FnConfig.new | result: {:?}", res);

        FnConfig {
            inputs: res,
        }
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

#[derive(Debug, Deserialize, PartialEq)]
pub enum FnConfigKeyword {
    Fn(String),
    Var(String),
    Const(String),
    Point(String),
}

impl FromStr for FnConfigKeyword {
    type Err = String;
    fn from_str(input: &str) -> Result<FnConfigKeyword, String> {
        match Regex::new(r#"^\s*([a-z]+)[^\S\r\n]+['""]{0,1}([^'":\n\s]+)['"]{0,1}"#).unwrap().captures(input) {
            Some(caps) => {
                match &caps.get(1) {
                    Some(fnPrefix) => {
                        match &caps.get(2) {
                            Some(name) => {
                                let name = name.as_str();
                                match fnPrefix.as_str() {
                                    "fn"  => Ok(FnConfigKeyword::Fn(name.into())),
                                    "var"  => Ok(FnConfigKeyword::Var(name.into())),
                                    "const"  => Ok(FnConfigKeyword::Const(name.into())),
                                    "point" => Ok(FnConfigKeyword::Point(name.into())),
                                    _      => Err(format!("Unknown keyword '{}'", input)),
                                }
                            },
                            None => {
                                Err(format!("Error reading argument of keyword '{}'", input))
                            },
                        }
                    },
                    None => {
                        Err(format!("Unknown keyword '{}'", input))
                    },
                }
            },
            None => {
                Err(format!("Unknown keyword '{}'", input))
            },
        }
    }
}

///
/// Config of type of the Function
#[derive(Debug)]
pub enum FnConfigType {
    Fn(FnConfig),
    Var(FnVarConfig),
    Const(FnConstConfig),
    Point(FnPointConfig),
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
use log::{error, trace, debug};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap, str::FromStr};

#[derive(Deserialize, Debug)]
pub struct FnConfig {
    pub inputs: Vec<FnConfig>,
}
impl FnConfig {
    ///
    /// creates config from LinkedHashMap
    pub fn new(name: &String, conf: &HashMap<String, serde_yaml::Value>) -> FnConfig {
        // match name {
            
        // }
        for (key, fnConfig) in conf {
            trace!("FnConfig.new | {}: {:?}", key, fnConfig)
        }

        FnConfig {
            inputs: vec![],
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
                    inputs: vec![],
                }                
            },
            Err(err) => {
                panic!("File {} reading error: {:?}", path, err)
            },
        }
    }
}

pub struct FnVarConfig {

}

pub struct FnConstConfig {
    value: String
}

pub struct FnPointConfig {
    
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum FnConfigKeyword {
    Fn,
    Var,
    Const,
    Point,
}

impl FromStr for FnConfigKeyword {
    type Err = String;
    fn from_str(input: &str) -> Result<FnConfigKeyword, String> {
        match Regex::new(r"^\s*([a-z]+)\s+\w+").unwrap().captures(input) {
            Some(caps) => {            
                match caps.get(1) {
                    Some(fnPrefix) => {
                        match fnPrefix.as_str() {
                            "fn"  => Ok(FnConfigKeyword::Fn),
                            "var"  => Ok(FnConfigKeyword::Var),
                            "const"  => Ok(FnConfigKeyword::Const),
                            "point" => Ok(FnConfigKeyword::Point),
                            _      => Err(format!("Unknown keyword '{}'", input)),
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
enum FnConfigType {
    Fn(FnConfig),
    Var(FnVarConfig),
    Const(FnConstConfig),
    Point(FnPointConfig),
}

impl FnConfigType {
    // type Err = String;
    fn new(input: &str) -> Result<FnConfigType, String> {
        match Regex::new(r"^\s*([a-z]+)\s+\w+").unwrap().captures(input) {
            Some(caps) => {            
                match caps.get(1) {
                    Some(fnPrefix) => {
                        match fnPrefix.as_str() {
                            "fn"  => Ok(FnConfigType::Fn(FnConfig { inputs: vec![] })),
                            "var"  => Ok(FnConfigType::Var(FnVarConfig {  })),
                            "const"  => Ok(FnConfigType::Const(FnConstConfig { value: String::new() })),
                            "point" => Ok(FnConfigType::Point(FnPointConfig {  })),
                            _      => Err(format!("Unknown keyword '{}'", input)),
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
use log::{error, trace, debug};
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap, str::FromStr};

use crate::core::nested_function::fn_conf_keywd::FnConfKeywd;


#[derive(Debug, PartialEq)]
pub struct FnConfig {
    pub fnType: FnConfigType,
    pub name: String,
    pub inputs: HashMap<String, FnConfig>,
}
impl FnConfig {
    ///
    /// creates config from serde_yaml::Value of following format:
    /// ```yaml
    /// let VarName2:
    ///     input: 
    ///     fn functionName:
    ///         initial: VarName1
    ///         input: 
    ///             functionName:
    ///             input1: const someValue
    ///             input2: point '/path/Point.Name/'
    ///             input: 
    ///                 fn functionName:
    ///                 input: point '/path/Point.Name/'```
    pub fn new(conf: &serde_yaml::Value) -> FnConfig {
        trace!("FnConfig.new | conf: {:?}", conf);
        if conf.is_string() {
            trace!("FnConfig.new | IS STRING");
            let fnConfStr: String = serde_yaml::from_value(conf.clone()).unwrap();
            let fnKeyword = FnConfKeywd::from_str(fnConfStr.as_str()).unwrap();
            trace!("FnConfig.new | key: '{}'   |   fnKeyword: {:?}   |   fnConfStr: {:?}", "---", fnKeyword, fnConfStr);
            match fnKeyword {
                FnConfKeywd::Const(name) => {
                    FnConfig { fnType: FnConfigType::Const, name: name.name, inputs: HashMap::new() }
                },
                FnConfKeywd::Point(name) => {
                    FnConfig { fnType: FnConfigType::Point, name: name.name, inputs: HashMap::new() }
                },
                _ => panic!("FnConfig.parseInput | Unknown config: {:?}", conf),
            }
        } else {
            trace!("FnConfig.new | IS MAP");
            let mut fnType: FnConfigType = FnConfigType::Unknown;
            let mut fnName: String = String::new(); 
            let mut inputs = HashMap::new();
            let fnConfMap: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(conf.clone()).unwrap();
            for (key, inputfnConfMap) in fnConfMap.clone() {
                // trace!("FnConfig.new | key: '{}'    |   conf: {:?}", key, inputfnConfMap);
                match FnConfKeywd::from_str(key.as_str()) {
                    Ok(fnKeyword) => {
                        trace!("FnConfig.new | IS KEYWD");
                        trace!("FnConfig.new | key: '{}'   |   fnKeyword: {:?}   |   inputfnConfMap: {:?}", key, fnKeyword, &inputfnConfMap);
                        let (fnTypeV, fnNameV, inputConfs) = Self::parseFn(fnKeyword, &inputfnConfMap);
                        fnType = fnTypeV;
                        fnName = fnNameV;
                        for (inputName, inputConf) in inputConfs {
                            inputs.insert(inputName, inputConf);
                        }
                    },
                    Err(err) => {
                        panic!("FnConfig.new | NO KEYWD\n\tkey: {}  conf: {:?}\n\t error: {}", key, inputfnConfMap, err);
                        // trace!("FnConfig.new | key: '{}'   |   fnKeyword: {:?}   |   inputfnConfMap: {:?}", key, "---", &inputfnConfMap);
                        // let (inputName, inputfnConf) = Self::parseInput(&inputfnConfMap);
                        // trace!("FnConfig.new | inputName: '{}'", inputName);
                        // inputs.insert(key, inputfnConf);
                    },
                };
            }
            trace!("FnConfig.new | fnType: {:?} \t|\t fnName: {:?} \t|\t inputs: {:?}", &fnType, fnName, inputs);
            FnConfig {fnType, name: fnName, inputs}
        }
    }
    ///
    /// parsing input sintax like:
    /// ```yaml
    /// fn fnName:
    ///         input1...
    ///         input2...
    /// ```
    /// or
    /// ```yaml
    /// let varNamne:
    ///         input...
    /// ```
    /// or
    /// ```yaml
    /// const 'val...'
    /// ```
    /// or
    /// ```yaml
    /// point '/path...'
    /// ```
    fn parseFn(fnKeyword: FnConfKeywd, conf: &serde_yaml::Value) -> (FnConfigType, String, Vec<(String, FnConfig)>) {
        trace!("FnConfig.parseFn | ENTER");
        let (fnType, fnName) = match fnKeyword {
            FnConfKeywd::Var(name) => {
                (FnConfigType::Var, name)
            },
            FnConfKeywd::Fn(name) => {
                (FnConfigType::Fn, name)
            },
            _ => panic!("Unknown config: {:?}", conf),
        };
        let inputs = Self::parseInputs(conf);
        (fnType, fnName.name, inputs)
    }
    ///
    /// parsing input sintax like:
    /// - input1:
    /// 
    ///         fn fnName...
    /// - input1: const 'val...'
    /// - input1: point '/path...'
    fn parseInputs(conf: &serde_yaml::Value) -> Vec<(String, FnConfig)> {
        trace!("FnConfig.parseInput | ENTER");
        let mut inputs = vec![];
        let inputsConfMap: HashMap<String, serde_yaml::Value> = serde_yaml::from_value(conf.clone()).unwrap();
        for (inputName, inputConfMap) in inputsConfMap {
            let inputConf = FnConfig::new(&inputConfMap);
            inputs.push((inputName, inputConf));
        };
        inputs
    }
    ///
    /// reads config from path
    pub fn read(path: &str) -> FnConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        FnConfig::new(&config)
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
#[derive(Debug, PartialEq)]
pub enum FnConfigType {
    Fn,
    Var,
    Const,
    Point,
    Unknown,
}

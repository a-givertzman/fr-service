use log::{error, trace, debug};
use serde::{Serialize, Deserialize};
use std::{fs, collections::HashMap};

#[derive(Deserialize, Debug)]
pub struct FnConfig {
    pub inputs: Vec<FnConfig>,
}
impl FnConfig {
    ///
    /// creates config from LinkedHashMap
    // pub fn new(&LinkedHashMap<Yaml, Yaml>) -> {
    //     FnConfig {
    //         inputs: vec![],
    //     }
    // }
    ///
    /// reads config from path
    pub fn read(path: &str) -> FnConfig {
        match fs::read_to_string(&path) {
            Ok(yamlString) => {
                match serde_yaml::from_str(&yamlString) {
                    Ok(config) => {
                        let fnConfig: FnConfig = config;
                    },
                    Err(err) => {
                        error!("Error in config: {:?}\n\terror: {:?}", yamlString, err)
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


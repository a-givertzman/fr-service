use std::str::FromStr;

use log::debug;
use regex::Regex;
use serde::Deserialize;


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FnConfKeywdValue {
    pub input: String,
    pub name: String,
}


#[derive(Debug, Deserialize, PartialEq)]
pub enum FnConfKeywd {
    Fn(FnConfKeywdValue),
    Var(FnConfKeywdValue),
    Const(FnConfKeywdValue),
    Point(FnConfKeywdValue),
}

impl FromStr for FnConfKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<FnConfKeywd, String> {
        // let re = r#"\s*([a-z]+)[^\S\r\n]+['""]{0,1}([^'":\n\s]+)['"]{0,1}"#;
        let re = r#"^\s*((\w+):)*\s*([a-z]+)[^\S\r\n]+['""]{0,1}([^'":\n\s]+)['"]{0,1}"#;
        match Regex::new(re).unwrap().captures(input) {
            Some(caps) => {
                let input = match &caps.get(2) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                match &caps.get(3) {
                    Some(fnPrefix) => {
                        match &caps.get(4) {
                            Some(name) => {
                                let name = name.as_str();
                                match fnPrefix.as_str() {
                                    "fn"  => Ok( FnConfKeywd::Fn( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "let"  => Ok( FnConfKeywd::Var( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "const"  => Ok( FnConfKeywd::Const( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "point" => Ok( FnConfKeywd::Point( FnConfKeywdValue { input: input, name: name.into() } )),
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

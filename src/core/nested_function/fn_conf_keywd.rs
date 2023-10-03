use std::str::FromStr;

use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum FnConfKeywd {
    Fn(String),
    Var(String),
    Const(String),
    Point(String),
}

impl FromStr for FnConfKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<FnConfKeywd, String> {
        match Regex::new(r#"\s*([a-z]+)[^\S\r\n]+['""]{0,1}([^'":\n\s]+)['"]{0,1}"#).unwrap().captures(input) {
            Some(caps) => {
                match &caps.get(1) {
                    Some(fnPrefix) => {
                        match &caps.get(2) {
                            Some(name) => {
                                let name = name.as_str();
                                match fnPrefix.as_str() {
                                    "fn"  => Ok(FnConfKeywd::Fn(name.into())),
                                    "let"  => Ok(FnConfKeywd::Var(name.into())),
                                    "const"  => Ok(FnConfKeywd::Const(name.into())),
                                    "point" => Ok(FnConfKeywd::Point(name.into())),
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

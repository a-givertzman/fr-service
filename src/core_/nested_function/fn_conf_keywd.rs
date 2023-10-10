use std::str::FromStr;

use log::trace;
use regex::Regex;
use serde::Deserialize;

use super::fn_config_type::FnConfigType;

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
    Metric(FnConfKeywdValue),
}

impl FnConfKeywd {
    pub fn input(&self) -> String {
        match self {
            FnConfKeywd::Fn(v) => v.input.clone(),
            FnConfKeywd::Var(v) => v.input.clone(),
            FnConfKeywd::Const(v) => v.input.clone(),
            FnConfKeywd::Point(v) => v.input.clone(),
            FnConfKeywd::Metric(v) => v.input.clone(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            FnConfKeywd::Fn(v) => v.name.clone(),
            FnConfKeywd::Var(v) => v.name.clone(),
            FnConfKeywd::Const(v) => v.name.clone(),
            FnConfKeywd::Point(v) => v.name.clone(),
            FnConfKeywd::Metric(v) => v.name.clone(),
        }
    }
    pub fn type_(&self) -> FnConfigType {
        match self {
            FnConfKeywd::Fn(_) => FnConfigType::Fn,
            FnConfKeywd::Var(_) => FnConfigType::Var,
            FnConfKeywd::Const(_) => FnConfigType::Const,
            FnConfKeywd::Point(_) => FnConfigType::Point,
            FnConfKeywd::Metric(_) => FnConfigType::Metric,
        }
    }
}

impl FromStr for FnConfKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<FnConfKeywd, String> {
        trace!("FnConfKeywd.from_str | input: {}", input);
        // let re = r#"\s*([a-z]+)[^\S\r\n]+['""]{0,1}([^'":\n\s]+)['"]{0,1}"#;
        // let re = r#"^\s*((\w+):)*\s*([a-z]+)[^\S\r\n]+['""]{0,1}([^'":\n\s]+)['"]{0,1}"#;
        let re = r#"[ \t]*(?:(\w+)[ \t]+)*(?:(let|fn|const|point|metric|task)+[ \t]+)(?:['"]*([\w/.]+)['"]*)"#;
        match Regex::new(re).unwrap().captures(input) {
            Some(caps) => {
                let input = match &caps.get(1) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                match &caps.get(2) {
                    Some(fnPrefix) => {
                        match &caps.get(3) {
                            Some(name) => {
                                let name = name.as_str();
                                match fnPrefix.as_str() {
                                    "fn"  => Ok( FnConfKeywd::Fn( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "let"  => Ok( FnConfKeywd::Var( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "const"  => Ok( FnConfKeywd::Const( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "point" => Ok( FnConfKeywd::Point( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "metric" => Ok( FnConfKeywd::Metric( FnConfKeywdValue { input: input, name: name.into() } )),
                                    "task" => Ok( FnConfKeywd::Metric( FnConfKeywdValue { input: input, name: name.into() } )),
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

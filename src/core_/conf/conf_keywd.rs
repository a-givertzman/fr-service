use std::str::FromStr;

use log::trace;
use regex::{Regex, RegexBuilder};
use serde::Deserialize;

use super::fn_config_type::FnConfigType;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FnConfKeywdValue {
    pub input: String,
    pub name: String,
}


#[derive(Debug, Deserialize, PartialEq)]
pub enum ConfKeywd {
    Fn(FnConfKeywdValue),
    Var(FnConfKeywdValue),
    Const(FnConfKeywdValue),
    Point(FnConfKeywdValue),
    Metric(FnConfKeywdValue),
}

impl ConfKeywd {
    pub fn input(&self) -> String {
        match self {
            ConfKeywd::Fn(v) => v.input.clone(),
            ConfKeywd::Var(v) => v.input.clone(),
            ConfKeywd::Const(v) => v.input.clone(),
            ConfKeywd::Point(v) => v.input.clone(),
            ConfKeywd::Metric(v) => v.input.clone(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            ConfKeywd::Fn(v) => v.name.clone(),
            ConfKeywd::Var(v) => v.name.clone(),
            ConfKeywd::Const(v) => v.name.clone(),
            ConfKeywd::Point(v) => v.name.clone(),
            ConfKeywd::Metric(v) => v.name.clone(),
        }
    }
    pub fn type_(&self) -> FnConfigType {
        match self {
            ConfKeywd::Fn(_) => FnConfigType::Fn,
            ConfKeywd::Var(_) => FnConfigType::Var,
            ConfKeywd::Const(_) => FnConfigType::Const,
            ConfKeywd::Point(_) => FnConfigType::Point,
            ConfKeywd::Metric(_) => FnConfigType::Metric,
        }
    }
}

impl FromStr for ConfKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<ConfKeywd, String> {
        trace!("FnConfKeywd.from_str | input: {}", input);
        let re = r#"[ \t]*(?:(\w+)[ \t]+)*(?:(let|fn|const|point|metric|task){1}(?:$|(?:[ \t]+['"]*([\w/.]+)['"]*)))"#;
        // let re = Regex::new(re).unwrap();
        let re = RegexBuilder::new(re).multi_line(true).build().unwrap();
        match re.captures(input) {
            Some(caps) => {
                let input = match &caps.get(1) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                let argument = match &caps.get(3) {
                    Some(arg) => {
                        Ok(arg.as_str().to_string())
                    },
                    None => {
                        if input.is_empty() {                            
                            Err(format!("Error reading argument of keyword '{}'", input))
                        } else {
                            Ok(String::new())
                        }
                    },
                };
                match argument {
                    Ok(argument) => {
                        match &caps.get(2) {
                            Some(keyword) => {
                                match keyword.as_str() {
                                    "fn"  => Ok( ConfKeywd::Fn( FnConfKeywdValue { input: input, name: argument } )),
                                    "let"  => Ok( ConfKeywd::Var( FnConfKeywdValue { input: input, name: argument } )),
                                    "const"  => Ok( ConfKeywd::Const( FnConfKeywdValue { input: input, name: argument } )),
                                    "point" => Ok( ConfKeywd::Point( FnConfKeywdValue { input: input, name: argument } )),
                                    "metric" => Ok( ConfKeywd::Metric( FnConfKeywdValue { input: input, name: argument } )),
                                    "task" => Ok( ConfKeywd::Metric( FnConfKeywdValue { input: input, name: argument } )),
                                    _      => Err(format!("Unknown keyword '{}'", input)),
                                }
                            },
                            None => {
                                Err(format!("Unknown keyword '{}'", input))
                            },
                        }
                    },
                    Err(err) => Err(err),
                }
            },
            None => {
                Err(format!("Unknown keyword '{}'", input))
            },
        }
    }
}

#![allow(non_snake_case)]

use std::str::FromStr;
use log::{trace, warn};
use regex::RegexBuilder;
use serde::Deserialize;

use super::fn_conf_kind::FnConfKind;

///
/// Represents type of Point / Const in the configuration
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum FnConfPointType {
    Bool,
    Int,
    Float,
    String,
    Unknown,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FnConfKeywdValue {
    pub input: String,
    pub type_: FnConfPointType,
    pub data: String,
}

///
/// keyword konsists of 4 fields:
/// ```
/// | input  |  kind  | type  |  data               |
/// | name   |        |       |                     |
/// |--------|--------|-------|---------------------|
/// | opt    | requir | opt   |                     |
/// |--------|--------|-------|---------------------|
/// | input  |  point | float | '/path/Point.name'  |
/// | input  |  const | int   | 17                  |
/// |        |  let   |       | varName             |
/// |        |  fn    |       | fnName              |
/// ````
#[derive(Debug, Deserialize, PartialEq)]
pub enum ConfKeywd {
    Fn(FnConfKeywdValue),
    Var(FnConfKeywdValue),
    Const(FnConfKeywdValue),
    Point(FnConfKeywdValue),
    Metric(FnConfKeywdValue),
}
///
/// 
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
    pub fn kind(&self) -> FnConfKind {
        match self {
            ConfKeywd::Fn(_) => FnConfKind::Fn,
            ConfKeywd::Var(_) => FnConfKind::Var,
            ConfKeywd::Const(_) => FnConfKind::Const,
            ConfKeywd::Point(_) => FnConfKind::Point,
            ConfKeywd::Metric(_) => FnConfKind::Metric,
        }
    }
    pub fn type_(&self) -> FnConfPointType {
        match self {
            ConfKeywd::Fn(v) => v.type_.clone(),
            ConfKeywd::Var(v) => v.type_.clone(),
            ConfKeywd::Const(v) => v.type_.clone(),
            ConfKeywd::Point(v) => v.type_.clone(),
            ConfKeywd::Metric(v) => v.type_.clone(),
        }
    }
    pub fn data(&self) -> String {
        match self {
            ConfKeywd::Fn(v) => v.data.clone(),
            ConfKeywd::Var(v) => v.data.clone(),
            ConfKeywd::Const(v) => v.data.clone(),
            ConfKeywd::Point(v) => v.data.clone(),
            ConfKeywd::Metric(v) => v.data.clone(),
        }
    }
    fn matchType(typeName: &str) -> Result<FnConfPointType, String> {
        match typeName {
            "bool" => Ok(FnConfPointType::Bool),
            "int" => Ok(FnConfPointType::Int),
            "float" => Ok(FnConfPointType::Float),
            "string" => Ok(FnConfPointType::String),
            _ => Err(format!("Unknown keyword '{}'", typeName))
        }
    }
}

impl FromStr for ConfKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<ConfKeywd, String> {
        trace!("FnConfKeywd.from_str | input: {}", input);
        // let re = r#"[ \t]*(?:(\w+)[ \t]+)*(?:(let|fn|const|point|metric|task){1}(?:$|(?:[ \t]+['"]*([\w/.]+)['"]*)))"#;
        let re = r#"[ \t]*(?:(\w+)[ \t]+)*(?:(let|fn|const|point|metric|task){1}(?:[ \t](bool|int|float|string))*(?:$|(?:[ \t]+['"]*([\w/.]+)['"]*)))"#;
        // let re = Regex::new(re).unwrap();
        let re = RegexBuilder::new(re).multi_line(true).build().unwrap();
        let groupInput = 1;
        let groupKind = 2;
        let groupType = 3;
        let groupData = 4;
        match re.captures(input) {
            Some(caps) => {
                let input = match &caps.get(groupInput) {
                    Some(first) => String::from(first.as_str()),
                    None => String::new(),
                };
                let type_ = match &caps.get(groupType) {
                    Some(arg) => {
                        match ConfKeywd::matchType(&arg.as_str().to_lowercase()) {
                            Ok(type_) => type_,
                            Err(err) => {
                                warn!("ConfKeywd.from_str | Error reading type of keyword '{}'", &input);
                                FnConfPointType::Unknown
                            },
                        }
                    },
                    None => FnConfPointType::Unknown,
                };
                let data = match &caps.get(groupData) {
                    Some(arg) => {
                        Ok(arg.as_str().to_string())
                    },
                    None => {
                        if input.is_empty() {                            
                            Err(format!("Error reading data of keyword '{}'", &input))
                        } else {
                            Ok(String::new())
                        }
                    },
                };
                match data {
                    Ok(data) => {
                        match &caps.get(groupKind) {
                            Some(keyword) => {
                                match keyword.as_str() {
                                    "fn"  => Ok( ConfKeywd::Fn( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "let"  => Ok( ConfKeywd::Var( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "const"  => Ok( ConfKeywd::Const( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "point" => Ok( ConfKeywd::Point( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "metric" => Ok( ConfKeywd::Metric( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "task" => Ok( ConfKeywd::Metric( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    _      => Err(format!("Unknown keyword '{}'", &input)),
                                }
                            },
                            None => {
                                Err(format!("Unknown keyword '{}'", &input))
                            },
                        }
                    },
                    Err(err) => Err(err),
                }
            },
            None => {
                Err(format!("Unknown keyword '{}'", &input))
            },
        }
    }
}

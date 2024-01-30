#![allow(non_snake_case)]

use std::str::FromStr;
use log::{trace, warn};
use regex::RegexBuilder;
use serde::Deserialize;

// use super::fn_conf_kind::FnConfKind;

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
pub enum FnConfKeywd {
    Fn(FnConfKeywdValue),
    Var(FnConfKeywdValue),
    Const(FnConfKeywdValue),
    Point(FnConfKeywdValue),
    // Metric(FnConfKeywdValue),
}
///
/// 
impl FnConfKeywd {
    pub fn input(&self) -> String {
        match self {
            FnConfKeywd::Fn(v) => v.input.clone(),
            FnConfKeywd::Var(v) => v.input.clone(),
            FnConfKeywd::Const(v) => v.input.clone(),
            FnConfKeywd::Point(v) => v.input.clone(),
        }
    }
    // pub fn kind(&self) -> FnConfKind {
    //     match self {
    //         FnConfKeywd::Fn(_) => FnConfKind::Fn,
    //         FnConfKeywd::Var(_) => FnConfKind::Var,
    //         FnConfKeywd::Const(_) => FnConfKind::Const,
    //         FnConfKeywd::Point(_) => FnConfKind::Point,
    //     }
    // }
    pub fn type_(&self) -> FnConfPointType {
        match self {
            FnConfKeywd::Fn(v) => v.type_.clone(),
            FnConfKeywd::Var(v) => v.type_.clone(),
            FnConfKeywd::Const(v) => v.type_.clone(),
            FnConfKeywd::Point(v) => v.type_.clone(),
        }
    }
    pub fn data(&self) -> String {
        match self {
            FnConfKeywd::Fn(v) => v.data.clone(),
            FnConfKeywd::Var(v) => v.data.clone(),
            FnConfKeywd::Const(v) => v.data.clone(),
            FnConfKeywd::Point(v) => v.data.clone(),
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

impl FromStr for FnConfKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<FnConfKeywd, String> {
        trace!("FnConfKeywd.from_str | input: {}", input);
        let re = r#"[ \t]*(?:(\w+)[ \t]+)*(?:(let|fn|const|point){1}(?:[ \t](bool|int|float|string))*(?:$|(?:[ \t]+['"]*([\w/.]+)['"]*)))"#;
        // let re = r#"[ \t]*(?:(\w+)[ \t]+)*(?:(let|fn|const|point|metric){1}(?:[ \t](bool|int|float|string))*(?:$|(?:[ \t]+['"]*([\w/.]+)['"]*)))"#;
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
                        match FnConfKeywd::matchType(&arg.as_str().to_lowercase()) {
                            Ok(type_) => type_,
                            Err(_err) => {
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
                                    "fn"  => Ok( FnConfKeywd::Fn( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "let"  => Ok( FnConfKeywd::Var( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "const"  => Ok( FnConfKeywd::Const( FnConfKeywdValue { input: input, type_: type_, data } )),
                                    "point" => Ok( FnConfKeywd::Point( FnConfKeywdValue { input: input, type_: type_, data } )),
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

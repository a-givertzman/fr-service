#![allow(non_snake_case)]

use std::str::FromStr;

use chrono::DateTime;
use log::trace;
use regex::RegexBuilder;

use crate::core_::types::bool::Bool;

use super::point::Point;


///
/// enum container for Point<T>
/// - supported types: Bool, Int, Float
#[derive(Debug, Clone)]
pub enum PointType {
    Bool(Point<Bool>),
    Int(Point<i64>),
    Float(Point<f64>),
    // String()
}
impl PointType {
    pub fn name(&self) -> String {
        match self {
            PointType::Bool(point) => point.name.clone(),
            PointType::Int(point) => point.name.clone(),
            PointType::Float(point) => point.name.clone(),
        }
    }
    pub fn asBool(&self) -> Point<Bool> {
        match self {
            PointType::Bool(point) => point.clone(),
            _ => panic!("PointType.asBool | Invalid point type Bool"),
        }
    }
    pub fn asInt(&self) -> Point<i64> {
        match self {
            PointType::Int(point) => point.clone(),
            _ => panic!("PointType.asInt | Invalid point type Int"),
        }
    }
    pub fn asFloat(&self) -> Point<f64> {
        match self {
            PointType::Float(point) => point.clone(),
            _ => panic!("PointType.asFloat | Invalid point type Float"),
        }
    }
    pub fn status(&self) -> u8 {
        match self {
            PointType::Bool(point) => point.status,
            PointType::Int(point) => point.status,
            PointType::Float(point) => point.status,
        }
    }
    pub fn timestamp(&self) -> DateTime<chrono::Utc> {
        match self {
            PointType::Bool(point) => point.timestamp,
            PointType::Int(point) => point.timestamp,
            PointType::Float(point) => point.timestamp,
        }
    }
}

impl FromStr for PointType {
    type Err = String;
    fn from_str(input: &str) -> Result<PointType, String> {
        trace!("PointType.from_str | input: {}", input);
        let re = r#"(bool|int|float){1}"#;
        let re = RegexBuilder::new(re).multi_line(false).build().unwrap();
        match re.captures(input) {
            Some(caps) => {
                match &caps.get(1) {
                    Some(keyword) => {
                        match keyword.as_str() {
                            "bool"  => Ok( PointType::Bool(Point::newBool("bool", false)) ),
                            "int"  => Ok( PointType::Int(Point::newInt("int", 0)) ),
                            "float"  => Ok( PointType::Float(Point::newFloat("float", 0.0)) ),
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

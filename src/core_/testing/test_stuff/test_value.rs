#![allow(non_snake_case)]

use crate::core_::point::point_type::{PointType, ToPoint};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}
impl Value {
    pub fn toString(&self) -> String {
        match &self {
            Value::Bool(v) => v.to_string(),
            Value::Int(v) => v.to_string(),
            Value::Float(v) => v.to_string(),
            Value::String(v) => v.to_string(),
        }
    }
    pub fn toPoint(&self, txId: usize, name: &str) -> PointType {
        match self {
            Value::Bool(value) => value.toPoint(txId, name),
            Value::Int(value) => value.toPoint(txId, name),
            Value::Float(value) => value.toPoint(txId, name),
            Value::String(value) => value.clone().toPoint(txId, name),
        }
    }
}

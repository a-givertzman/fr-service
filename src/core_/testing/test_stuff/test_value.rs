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
    pub fn toPoint(&self, name: &str) -> PointType {
        match self {
            Value::Bool(value) => value.toPoint(0, name),
            Value::Int(value) => value.toPoint(0, name),
            Value::Float(value) => value.toPoint(0, name),
            Value::String(value) => value.clone().toPoint(0, name),
        }
    }
}

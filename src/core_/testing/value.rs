#![allow(non_snake_case)]
use testing::entities::test_value::Value;

use crate::core_::point::point_type::{PointType, ToPoint};

impl ToPoint for Value {
    fn toPoint(&self, txId: usize, name: &str) -> PointType {
        match self {
            Value::Bool(value) => value.toPoint(txId, name),
            Value::Int(value) => value.toPoint(txId, name),
            Value::Float(value) => value.toPoint(txId, name),
            Value::String(value) => value.clone().toPoint(txId, name),
        }
    }
}
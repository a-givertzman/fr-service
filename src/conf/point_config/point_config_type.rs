use serde::{Serialize, Deserialize};

///
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PointConfigType {
    #[serde(rename = "Bool")]
    #[serde(alias = "bool", alias = "Bool")]
    Bool,
    #[serde(rename = "Int")]
    #[serde(alias = "int", alias = "Int")]
    Int,
    #[serde(rename = "Real")]
    #[serde(alias = "real", alias = "Real")]
    Real,
    #[serde(rename = "Double")]
    #[serde(alias = "double", alias = "Double")]
    Double,
    #[serde(rename = "String")]
    #[serde(alias = "string", alias = "String")]
    String,
    #[serde(rename = "Json")]
    #[serde(alias = "json", alias = "Json")]
    Json,
}

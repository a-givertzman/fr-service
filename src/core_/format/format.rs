use std::collections::HashMap;

use log::{warn, debug, trace};
use regex::RegexBuilder;

///
/// Replaces markers by the string with the concrete values
/// - string: "insert into {table} (id, value) values ({id}, {value})"
/// - values: table = "temperature"; id = 1; value = 19,7
/// - out   : "insert into temperature (id, value) values (1, 19,7)"
pub struct Format {
    input: String,
    values: HashMap<String, String>,
}
///
/// 
impl Format {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.into(),
            values: HashMap::new(),

        }
    }
    pub fn insert(&mut self, key: &str, value: impl ToString) {
        self.values.insert(key.into(), value.to_string());
    }
    pub fn out(&self) -> String {
        let mut input = self.input.clone();
        for (key, value) in self.values.iter() {
            let pattern = format!("{{{}}}", key);
            trace!("replacing pattern {:?} with value: {:?}", pattern, value);
            input = input.replace(&pattern, value);
            trace!("input: {:?}", input);
        };
        input
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.out())
    }
}

pub enum FormatValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}
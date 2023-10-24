#![allow(non_snake_case)]

use std::collections::HashMap;

use log::trace;
use regex::RegexBuilder;

///
/// Replaces markers {marker name} with the concrete values
/// - values can be added using insert method format.insert(name, value)
/// - string: "insert into {table} (id, value) values ({id}, {value})"
/// - values: table = "temperature"; id = 1; value = 19,7
/// - out   : "insert into temperature (id, value) values (1, 19,7)"
pub struct Format {
    input: String,
    names: HashMap<String, (String, Option<String>)>,
    values: HashMap<String, String>,
}
///
/// 
impl Format {
    pub fn new(input: &str) -> Self {
        let re = r#"\{(.*?)\}"#;
        let re = RegexBuilder::new(re).multi_line(true).build().unwrap();
        let names = re.captures_iter(&input).map(|cap| {
            let fullName = cap.get(1).unwrap().as_str().to_string();
            let mut parts = fullName.split(".").map(|part| part.into());
            let name = parts.next().unwrap();
            let sufix = parts.next();
            (fullName, (name, sufix))
        }).collect();        
        trace!("Format.new | names {:?}", &names);
        Self {
            input: input.into(),
            names: names,
            values: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: &str, value: impl ToString) {
        self.values.insert(key.into(), value.to_string());
    }
    pub fn out(&self) -> String {
        let mut input = self.input.clone();
        for (fullName, (name, sufix)) in &self.names {
            trace!("Format.out | fullName {:?}", fullName);
            match self.values.get(fullName) {
                Some(value) => {
                    let pattern = format!("{{{}}}", fullName);
                    trace!("Format.out | replacing pattern {:?} with value: {:?}", pattern, value);
                    input = input.replace(&pattern, value);
                    trace!("Format.out | result: {:?}", input);
                },
                None => {},
            };
        };
        input
    }
    ///
    /// 
    pub fn names(&self) -> HashMap<String, (String, Option<String>)> {
        self.names.clone()
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.out())
    }
}
///
/// 
impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.out())
        // f.debug_struct("Format").field("input", &self.input).field("values", &self.values).finish()
    }
}
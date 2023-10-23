#![allow(non_snake_case)]

use std::str::FromStr;

use log::trace;

const INPUT: &str = "input";
const COUNT: &str = "count";
const ADD: &str = "add";
const TO_API_QUEUE: &str = "toApiQueue";
const TIMER: &str = "timer";


///
/// Entair list of all supported functions
#[derive(Debug)]
pub enum Functions {
    Input,
    Count,
    Add,
    ToApiQueue,
    Timer,
}
///
/// 
impl Functions {
    pub fn name(&self) -> &str {
        match self {
            Functions::Input => INPUT,
            Functions::Count => COUNT,
            Functions::Add => ADD,
            Functions::ToApiQueue => TO_API_QUEUE,
            Functions::Timer => TIMER,
        }
    }
}



impl FromStr for Functions {
    type Err = String;
    fn from_str(input: &str) -> Result<Functions, String> {
        trace!("Functions.from_str | input: {}", input);
        match input.to_lowercase().as_str() {
            INPUT           => Ok( Functions::Input),
            COUNT           => Ok( Functions::Count),
            ADD             => Ok( Functions::Add),
            TO_API_QUEUE    => Ok( Functions::ToApiQueue),
            TIMER           => Ok( Functions::Timer ),
            _ => Err(format!("Functions.from_str | Unknown function name '{}'", &input)),
        }
    }
}

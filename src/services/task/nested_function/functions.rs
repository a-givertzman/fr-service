#![allow(non_snake_case)]

use std::str::FromStr;

use log::trace;

const ADD: &str = "add";
const CONST: &str = "const";
const COUNT: &str = "count";
const GE: &str = "ge";
const INPUT: &str = "input";
const TIMER: &str = "timer";
const VAR: &str = "var";
const TO_API_QUEUE: &str = "ToApiQueue";
const TO_MULTI_QUEUE: &str = "ToMultiQueue";
const SQL_METRIC: &str = "SqlMetric";


///
/// Entair list of public functions
/// supported by NestedFn builder
#[derive(Debug)]
pub enum Functions {
    Add,
    Const,
    Count,
    Ge,
    Input,
    Timer,
    Var,
    ToApiQueue,
    ToMultiQueue,
    SqlMetric,
}
///
/// 
impl Functions {
    pub fn name(&self) -> &str {
        match self {
            Functions::Add => ADD,
            Functions::Const => CONST,
            Functions::Count => COUNT,
            Functions::Ge => GE,
            Functions::Input => INPUT,
            Functions::Timer => TIMER,
            Functions::Var => VAR,
            Functions::ToApiQueue => TO_API_QUEUE,
            Functions::ToMultiQueue => TO_MULTI_QUEUE,
            Functions::SqlMetric => SQL_METRIC,
        }
    }
}



impl FromStr for Functions {
    type Err = String;
    fn from_str(input: &str) -> Result<Functions, String> {
        trace!("Functions.from_str | input: {}", input);
        match input {
            ADD             => Ok( Functions::Add),
            CONST           => Ok( Functions::Const),
            COUNT           => Ok( Functions::Count),
            GE              => Ok( Functions::Ge ),
            INPUT           => Ok( Functions::Input),
            TIMER           => Ok( Functions::Timer ),
            VAR             => Ok( Functions::Var ),
            TO_API_QUEUE    => Ok( Functions::ToApiQueue),
            TO_MULTI_QUEUE  => Ok( Functions::ToMultiQueue),
            SQL_METRIC      => Ok( Functions::SqlMetric ),
            _ => Err(format!("Functions.from_str | Unknown function name '{}'", &input)),
        }
    }
}

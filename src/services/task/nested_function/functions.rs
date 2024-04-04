//!
//! Here must be defined all functions to be awalible in the Task -> NestedFn
use std::str::FromStr;
use log::trace;
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
    PointId,
    Debug,
    ToInt,
}
///
/// 
impl Functions {
    const ADD: &'static str = "add";
    const CONST: &'static str = "const";
    const COUNT: &'static str = "count";
    const GE: &'static str = "ge";
    const INPUT: &'static str = "input";
    const TIMER: &'static str = "timer";
    const VAR: &'static str = "var";
    const TO_API_QUEUE: &'static str = "ToApiQueue";
    const TO_MULTI_QUEUE: &'static str = "ToMultiQueue";
    const SQL_METRIC: &'static str = "SqlMetric";
    const POINT_ID: &'static str = "PointId";
    const DEBUG: &'static str = "debug";
    const TO_INT: &'static str = "ToInt";
    ///
    ///     
    pub fn name(&self) -> &str {
        match self {
            Functions::Add              => Self::ADD,
            Functions::Const            => Self::CONST,
            Functions::Count            => Self::COUNT,
            Functions::Ge               => Self::GE,
            Functions::Input            => Self::INPUT,
            Functions::Timer            => Self::TIMER,
            Functions::Var              => Self::VAR,
            Functions::ToApiQueue       => Self::TO_API_QUEUE,
            Functions::ToMultiQueue     => Self::TO_MULTI_QUEUE,
            Functions::SqlMetric        => Self::SQL_METRIC,
            Functions::PointId          => Self::POINT_ID,
            Functions::Debug            => Self::DEBUG,
            Functions::ToInt            => Self::TO_INT,
        }
    }
}
///
/// 
impl FromStr for Functions {
    type Err = String;
    fn from_str(input: &str) -> Result<Functions, String> {
        trace!("Functions.from_str | input: {}", input);
        match input {
            Self::ADD               => Ok( Functions::Add ),
            Self::CONST             => Ok( Functions::Const ),
            Self::COUNT             => Ok( Functions::Count ),
            Self::GE                => Ok( Functions::Ge ),
            Self::INPUT             => Ok( Functions::Input ),
            Self::TIMER             => Ok( Functions::Timer ),
            Self::VAR               => Ok( Functions::Var ),
            Self::TO_API_QUEUE      => Ok( Functions::ToApiQueue ),
            Self::TO_MULTI_QUEUE    => Ok( Functions::ToMultiQueue ),
            Self::SQL_METRIC        => Ok( Functions::SqlMetric ),
            Self::POINT_ID          => Ok( Functions::PointId ),
            Self::DEBUG             => Ok( Functions::Debug ),
            Self::TO_INT            => Ok( Functions::ToInt ),
            _ => Err(format!("Functions.from_str | Unknown function name '{}'", &input)),
        }
    }
}

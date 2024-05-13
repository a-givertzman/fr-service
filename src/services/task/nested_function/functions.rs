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
    Previous
}
///
/// 
impl Functions {
    const ADD               : &'static str = "add";
    const CONST             : &'static str = "const";
    const COUNT             : &'static str = "count";
    const GE                : &'static str = "ge";
    const INPUT             : &'static str = "input";
    const TIMER             : &'static str = "timer";
    const VAR               : &'static str = "var";
    const TO_API_QUEUE      : &'static str = "ToApiQueue";
    const TO_MULTI_QUEUE    : &'static str = "ToMultiQueue";
    const SQL_METRIC        : &'static str = "SqlMetric";
    const POINT_ID          : &'static str = "PointId";
    const DEBUG             : &'static str = "debug";
    const TO_INT            : &'static str = "ToInt";
    const PREVIOUS          : &'static str = "Previous";
    ///
    ///     
    pub fn name(&self) -> &str {
        match self {
            Self::Add              => Self::ADD,
            Self::Const            => Self::CONST,
            Self::Count            => Self::COUNT,
            Self::Ge               => Self::GE,
            Self::Input            => Self::INPUT,
            Self::Timer            => Self::TIMER,
            Self::Var              => Self::VAR,
            Self::ToApiQueue       => Self::TO_API_QUEUE,
            Self::ToMultiQueue     => Self::TO_MULTI_QUEUE,
            Self::SqlMetric        => Self::SQL_METRIC,
            Self::PointId          => Self::POINT_ID,
            Self::Debug            => Self::DEBUG,
            Self::ToInt            => Self::TO_INT,
            Self::Previous         => Self::PREVIOUS,
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
            Self::ADD               => Ok( Self::Add ),
            Self::CONST             => Ok( Self::Const ),
            Self::COUNT             => Ok( Self::Count ),
            Self::GE                => Ok( Self::Ge ),
            Self::INPUT             => Ok( Self::Input ),
            Self::TIMER             => Ok( Self::Timer ),
            Self::VAR               => Ok( Self::Var ),
            Self::TO_API_QUEUE      => Ok( Self::ToApiQueue ),
            Self::TO_MULTI_QUEUE    => Ok( Self::ToMultiQueue ),
            Self::SQL_METRIC        => Ok( Self::SqlMetric ),
            Self::POINT_ID          => Ok( Self::PointId ),
            Self::DEBUG             => Ok( Self::Debug ),
            Self::TO_INT            => Ok( Self::ToInt ),
            Self::PREVIOUS          => Ok( Self::Previous ),
            _ => Err(format!("Functions.from_str | Unknown function name '{}'", &input)),
        }
    }
}

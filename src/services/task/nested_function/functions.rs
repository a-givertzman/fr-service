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
    Export,
    Filter,
    RisingEdge
}
///
///
impl Functions {
    /// embedded functions
    const INPUT             : &'static str = "input";
    const CONST             : &'static str = "const";
    const VAR               : &'static str = "var";
    /// user defined functions
    const ADD               : &'static str = "Add";
    const COUNT             : &'static str = "Count";
    const DEBUG             : &'static str = "Debug";
    const GE                : &'static str = "Ge";
    const TIMER             : &'static str = "Timer";
    const TO_API_QUEUE      : &'static str = "ToApiQueue";
    const TO_MULTI_QUEUE    : &'static str = "ToMultiQueue";
    const SQL_METRIC        : &'static str = "SqlMetric";
    const POINT_ID          : &'static str = "PointId";
    const TO_INT            : &'static str = "ToInt";
    const EXPORT            : &'static str = "Export";
    const FILTER            : &'static str = "Filter";
    const RISING_EDGE       : &'static str = "RisingEdge";
    ///
    ///
    pub fn name(&self) -> &str {
        match self {
            Self::Add               => Self::ADD,
            Self::Const             => Self::CONST,
            Self::Count             => Self::COUNT,
            Self::Ge                => Self::GE,
            Self::Input             => Self::INPUT,
            Self::Timer             => Self::TIMER,
            Self::Var               => Self::VAR,
            Self::ToApiQueue        => Self::TO_API_QUEUE,
            Self::ToMultiQueue      => Self::TO_MULTI_QUEUE,
            Self::SqlMetric         => Self::SQL_METRIC,
            Self::PointId           => Self::POINT_ID,
            Self::Debug             => Self::DEBUG,
            Self::ToInt             => Self::TO_INT,
            Self::Export            => Self::EXPORT,
            Self::Filter            => Self::FILTER,
            Self::RisingEdge        => Self::RISING_EDGE,
        }
    }
    ///
    /// 
    fn match_name(input: &str) -> Result<Functions, String> {
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
            Self::EXPORT            => Ok( Self::Export ),
            Self::FILTER            => Ok( Self::Filter ),
            Self::RISING_EDGE       => Ok( Self::RisingEdge ),
            _ => Err(format!("Functions.from_str | Unknown function name '{}'", &input)),
        }
    }
}
///
///
impl FromStr for Functions {
    type Err = String;
    fn from_str(input: &str) -> Result<Functions, String> {
        trace!("Functions.from_str | input: {}", input);
        Self::match_name(input)
    }
}

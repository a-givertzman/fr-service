use std::{fmt::Display, str::FromStr};
use log::trace;
///
/// The defination of all diagnosis signals
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DiagKeywd {
    Status,
    Connection,
}
///
/// 
impl DiagKeywd {
    const STATUS: &'static str = "status";
    const CONNECTION: &'static str = "connection";
    fn as_str(&self) -> &str {
        match self {
            DiagKeywd::Status           => DiagKeywd::STATUS,
            DiagKeywd::Connection       => DiagKeywd::CONNECTION,
        }
    }
}
///
/// 
impl FromStr for DiagKeywd {
    type Err = String;
    fn from_str(input: &str) -> Result<DiagKeywd, String> {
        trace!("DiagKeywd.from_str | input: {}", input);
        match input.to_lowercase().as_str() {
            Self::STATUS        if input.ends_with(&Self::STATUS) => Ok(Self::Status),
            Self::CONNECTION    if input.ends_with(&Self::CONNECTION) => Ok(Self::Connection),
            _ => panic!("DiagKeywd.from_str | Diagnosis point '{}' - does not supported", input)
        }
    }
}
// ///
// /// 
// impl<'a> Into<&'a str> for DiagKeywd {
//     fn into(self) -> &'a str {
//         match self {
//             DiagKeywd::Status           => &DiagKeywd::STATUS,
//             DiagKeywd::Connection       => &DiagKeywd::CONNECTION,
//         }
//     }
// }
///
/// 
impl Display for DiagKeywd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

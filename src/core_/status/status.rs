use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Status {
    Ok = 0,
    Obsolete = 2,
    TimeInvalid = 3,
    Invalid = 10,
    Uncnown = 99,
}
///
/// 
impl Ord for Status {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(&other)
    }
}

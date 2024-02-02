use std::cmp::Ordering;

const OK            : i64 = 0;
const OBSOLETE      : i64 = 2;
const TIME_INVALID  : i64 = 3;
const INVALID       : i64 = 10;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
#[repr(u32)]
pub enum Status {
    Ok              = OK as u32,
    Obsolete        = OBSOLETE as u32,
    TimeInvalid     = TIME_INVALID as u32,
    Invalid         = INVALID as u32,
    Unknown(i64),
}
///
/// 
impl Ord for Status {
    fn cmp(&self, other: &Self) -> Ordering {
        Into::<u32>::into(*self).cmp(&Into::<u32>::into(*other))
    }
}
///
/// 
impl ToString for Status {
    fn to_string(&self) -> String {
        Into::<u32>::into(*self).to_string()
    }
}
///
/// 
impl From<i64> for Status {
    fn from(value: i64) -> Self {
        match value {
            OK              => Status::Ok,
            OBSOLETE        => Status::Obsolete,
            TIME_INVALID    => Status::TimeInvalid,
            INVALID         => Status::Invalid,
            _               => Status::Unknown(value),
        } 
    }
}
///
/// 
impl From<u64> for Status {
    fn from(value: u64) -> Self {
        Self::from(value as i64)
    }
}
///
/// 
impl Into<u64> for Status {
    fn into(self) -> u64 {
        Into::<u32>::into(self) as u64
    }
}
///
/// 
impl Into<u32> for Status {
    fn into(self) -> u32 {
        match self {
            Status::Ok              => OK as u32,
            Status::Obsolete        => OBSOLETE as u32,
            Status::TimeInvalid     => TIME_INVALID as u32,
            Status::Invalid         => INVALID as u32,
            Status::Unknown(value) => value as u32,
        } 
    }
}

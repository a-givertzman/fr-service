#![allow(non_snake_case)]

use chrono::DateTime;

use crate::core_::types::bool::Bool;


#[derive(Clone, Debug, PartialEq)]
pub struct Point<T> {
    pub name: String,
    pub value: T,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}
impl<T> Point<T> {
    pub fn new(name: &str, value: T, status: u8, timestamp: DateTime<chrono::Utc>,) -> Point<T> {
        Self {
            name: name.to_owned(),
            value,
            status,
            timestamp,
        }
    }
}
///
/// 
impl Point<Bool> {
    ///
    /// creates Point<Bool> with given name & value, taking current timestamp
    pub fn newBool(name: &str, value: bool) -> Point<Bool> {
        Point {
            name: name.into(),
            value: Bool(value),
            status: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl Point<i64> {
    ///
    /// creates Point<i64> with given name & value, taking current timestamp
    pub fn newInt(name: &str, value: i64) -> Point<i64> {
        Point {
            name: name.into(),
            value: value,
            status: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl Point<f64> {
    ///
    /// creates Point<f64> with given name & value, taking current timestamp
    pub fn newFloat(name: &str, value: f64) -> Point<f64> {
        Point {
            name: name.into(),
            value: value,
            status: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl Point<String> {
    ///
    /// creates Point<String> with given name & value, taking current timestamp
    pub fn newString(name: &str, value: impl Into<String>) -> Point<String> {
        Point {
            name: name.into(),
            value: value.into(),
            status: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
///
/// 
impl<T: std::ops::Add<Output = T> + Clone> std::ops::Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let timestamp = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => rhs.timestamp,
            std::cmp::Ordering::Equal => self.timestamp,
            std::cmp::Ordering::Greater => self.timestamp,
        };
        Point {
            name: String::from("Point.Add"),
            value: self.value + rhs.value,
            status: status,
            timestamp: timestamp,
        }
    }
}


impl<T: std::ops::BitOr<Output = T>> std::ops::BitOr for Point<T> {
    type Output = Point<T>;
    fn bitor(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let timestamp = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => rhs.timestamp,
            std::cmp::Ordering::Equal => self.timestamp,
            std::cmp::Ordering::Greater => self.timestamp,
        };
        Point {
            name: String::from("Point.BitOr"),
            value: self.value | rhs.value,
            status: status,
            timestamp: timestamp,
        }        
    }
}


impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for Point<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.value.partial_cmp(&other.value) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.status.partial_cmp(&other.status) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.timestamp.partial_cmp(&other.timestamp)
    }
}

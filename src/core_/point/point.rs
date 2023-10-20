#![allow(non_snake_case)]

use std::str::FromStr;

use chrono::DateTime;
use log::trace;
use regex::RegexBuilder;

use crate::core_::types::bool::Bool;


#[derive(Clone, Debug)]
pub struct Point<T> {
    pub name: String,
    pub value: T,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}


impl Point<bool> {
    pub fn newBool(name: &str, value: bool) -> Point<Bool> {
        Point {
            name: name.into(),
            value: Bool(value),
            status: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
impl Point<i64> {
    pub fn newInt(name: &str, value: i64) -> Point<i64> {
        Point {
            name: name.into(),
            value: value,
            status: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}
impl Point<f64> {
    pub fn newFloat(name: &str, value: f64) -> Point<f64> {
        Point {
            name: name.into(),
            value: value,
            status: 0,
            timestamp: chrono::offset::Utc::now(),
        }
    }
}

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

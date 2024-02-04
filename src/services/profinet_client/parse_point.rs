#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use crate::core_::point::point_type::PointType;

///
/// Returns updated points parsed from the data slice from the S7 device,
pub trait ParsePoint {
    ///
    /// Returns new point parsed from the data slice [bytes] with current timestamp and Status::Ok
    fn nextSimple(&mut self, bytes: &Vec<u8>) -> Option<PointType>;
    ///
    /// Returns new point parsed from the data slice [bytes] with the given [timestamp] and Status::Ok
    fn next(&mut self, bytes: &Vec<u8>, timestamp: DateTime<Utc>) -> Option<PointType>;
    ///
    /// Returns new point (prevously parsed) with the given [status]
    fn nextStatus(&mut self, bytes: &Vec<u8>, timestamp: DateTime<Utc>) -> Option<PointType>;
    ///
    /// Returns true if value or status was updated since last call [addRaw()]
    fn isChanged(&self) -> bool;
}

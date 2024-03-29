#![allow(non_snake_case)]

use log::{debug, warn};
use std::array::TryFromSliceError;
use chrono::{DateTime, Utc};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_address::PointConfigAddress, point_config_history::PointConfigHistory},
    core_::{cot::cot::Cot, filter::filter::Filter, point::{point::Point, point_type::PointType}, status::status::Status}, 
    services::profinet_client::parse_point::ParsePoint,
};


///
///
#[derive(Debug)]
pub struct S7ParseInt {
    pub txId: usize,
    pub path: String,
    pub name: String,
    pub value: Box<dyn Filter<Item = i64>>,
    pub status: Status,
    pub offset: Option<u32>,
    pub history: PointConfigHistory,
    pub alarm: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
    isChanged: bool,
}
///
/// 
impl S7ParseInt {
    ///
    /// 
    pub fn new(
        path: String,
        name: String,
        config: &PointConfig,
        filter: Box<dyn Filter<Item = i64>>,
    ) -> S7ParseInt {
        S7ParseInt {
            txId: 0,
            path,
            name,
            value: filter,
            status: Status::Invalid,
            isChanged: false,
            offset: config.clone().address.unwrap_or(PointConfigAddress::empty()).offset,
            history: config.history.clone(),
            alarm: config.alarm,
            comment: config.comment.clone(),
            timestamp: Utc::now(),
        }
    }
    //
    //
    fn convert(
        &self,
        bytes: &[u8],
        start: usize,
        _bit: usize,
    ) -> Result<i16, TryFromSliceError> {
        // debug!("[S7ParsePoint<i16>.convert] start: {},  end: {:?}", start, start + 2);
        // let raw: [u8; 2] = (bytes[start..(start + 2)]).try_into().unwrap();
        // debug!("[S7ParsePoint<i16>.convert] raw: {:?}", raw);
        match bytes[start..(start + 2)].try_into() {
            Ok(v) => Ok(i16::from_be_bytes(v)),
            Err(e) => {
                debug!("[S7ParsePoint<i16>.convert] error: {}", e);
                Err(e)
            }
        }
    }
    ///
    /// 
    fn toPoint(&self) -> Option<PointType> {
        if self.isChanged {
            Some(PointType::Int(Point::new(
                self.txId, 
                &self.name, 
                self.value.value(),
                self.status, 
                Cot::Inf,
                self.timestamp,
            )))
            // debug!("{} point Bool: {:?}", self.id, dsPoint.value);
        } else {
            None
        }
    }
    //
    // 
    fn addRawSimple(&mut self, bytes: &[u8]) {
        self.addRaw(bytes, Utc::now())
    }
    //
    //
    fn addRaw(&mut self, bytes: &[u8], timestamp: DateTime<Utc>) {
        let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
        match result {
            Ok(newVal) => {
                let newVal = newVal as i64;
                if newVal != self.value.value() {
                    self.value.add(newVal);
                    self.status = Status::Ok;
                    self.timestamp = timestamp;
                    self.isChanged = true;
                }
            }
            Err(e) => {
                self.status = Status::Invalid;
                warn!("[S7ParsePoint<i16>.addRaw] convertion error: {:?}", e);
            }
        }
    }
}
///
/// 
impl ParsePoint for S7ParseInt {
    //
    //
    fn next_simple(&mut self, bytes: &[u8]) -> Option<PointType> {
        self.addRawSimple(bytes);
        self.toPoint()
    }
    //
    //
    fn next(&mut self, bytes: &[u8], timestamp: DateTime<Utc>) -> Option<PointType> {
        self.addRaw(bytes, timestamp);
        self.toPoint()
    }
    //
    //
    fn next_status(&mut self, status: Status) -> Option<PointType> {
        self.status = status;
        self.timestamp = Utc::now();
        self.toPoint()
    }
    //
    //
    fn is_changed(&self) -> bool {
        self.isChanged
    }
    //
    //
    fn address(&self) -> PointConfigAddress {
        PointConfigAddress { offset: self.offset, bit: None }
    }
}

#![allow(non_snake_case)]

use log::{debug, warn};
use std::array::TryFromSliceError;
use chrono::{DateTime, Utc};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_address::PointConfigAddress, point_config_history::PointConfigHistory}, 
    core_::{cot::cot::Cot, point::{point::Point, point_type::PointType}, status::status::Status, types::bool::Bool}, 
    services::profinet_client::parse_point::ParsePoint,
};

///
///
#[derive(Debug, Clone)]
pub struct S7ParseBool {
    pub txId: usize,
    pub path: String,
    pub name: String,
    pub value: bool,
    pub status: Status,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub history: PointConfigHistory,
    pub alarm: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
    isChanged: bool,
}
impl S7ParseBool {
    ///
    /// 
    pub fn new(
        path: String,
        name: String,
        config: &PointConfig,
        // filter: Filter<T>,
    ) -> S7ParseBool {
        S7ParseBool {
            txId: 0,
            path,
            name,
            value: false,
            status: Status::Invalid,
            isChanged: false,
            offset: config.clone().address.unwrap_or(PointConfigAddress::empty()).offset,
            bit: config.clone().address.unwrap_or(PointConfigAddress::empty()).bit,
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
        bit: usize,
    ) -> Result<bool, TryFromSliceError> {
        // debug!("[S7ParsePoint<bool>.convert] start: {},  end: {:?}", start, start + 2);
        // let raw: [u8; 2] = (bytes[start..(start + 2)]).try_into().unwrap();
        // debug!("[S7ParsePoint<bool>.convert] raw: {:?}", raw);
        match bytes[start..(start + 2)].try_into() {
            Ok(v) => {
                let i = i16::from_be_bytes(v);
                let b: i16 = i >> bit & 1;
                Ok(b > 0)
            }
            Err(e) => {
                debug!("[S7ParsePoint<bool>.convert] error: {}", e);
                Err(e)
            }
        }
    }
    ///
    /// 
    fn toPoint(&self) -> Option<PointType> {
        if self.isChanged {
            Some(PointType::Bool(Point::new(
                self.txId, 
                &self.name, 
                Bool(self.value), 
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
        let result = self.convert(
            bytes,
            self.offset.unwrap() as usize,
            self.bit.unwrap() as usize,
        );
        match result {
            Ok(newVal) => {
                if newVal != self.value {
                    self.value = newVal;
                    self.status = Status::Ok;
                    self.timestamp = timestamp;
                    self.isChanged = true;
                }
            }
            Err(e) => {
                self.status = Status::Invalid;
                warn!("[S7ParsePoint<bool>.addRaw] convertion error: {:?}", e);
            }
        }
    }    
}
///
impl ParsePoint for S7ParseBool {
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
        PointConfigAddress { offset: self.offset, bit: self.bit }
    }
}

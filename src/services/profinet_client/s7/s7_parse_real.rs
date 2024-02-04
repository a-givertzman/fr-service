#![allow(non_snake_case)]

use log::{debug, warn};
use std::array::TryFromSliceError;
use chrono::{DateTime, Utc};
use crate::{
    core_::{point::{point::Point, point_type::PointType}, status::status::Status},
    conf::point_config::{point_config::PointConfig, point_config_address::PointConfigAddress}, 
    services::profinet_client::parse_point::ParsePoint,
};

///
///
#[derive(Debug, Clone)]
pub struct S7ParseReal {
    pub txId: usize,
    pub path: String,
    pub name: String,
    pub value: f32,
    pub status: Status,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub h: Option<u8>,
    pub a: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
    isChanged: bool,
}
///
/// 
impl S7ParseReal {
    ///
    /// 
    pub fn new(
        path: String,
        name: String,
        config: &PointConfig,
        // filter: Filter<T>,
    ) -> S7ParseReal {
        S7ParseReal {
            txId: 0,
            value: 0.0,
            status: Status::Invalid,
            isChanged: false,
            path: path,
            name: name,
            offset: config.clone().address.unwrap_or(PointConfigAddress::empty()).offset,
            bit: config.clone().address.unwrap_or(PointConfigAddress::empty()).bit,
            h: config.history,
            a: config.alarm,
            comment: config.comment.clone(),
            timestamp: Utc::now(),
        }
    }
    //
    //
    fn convert(
        &self,
        bytes: &Vec<u8>,
        start: usize,
        _bit: usize,
    ) -> Result<f32, TryFromSliceError> {
        // debug!("[S7ParsePoint<f32>.convert] start: {},  end: {:?}", start, start + 4);
        // let raw: [u8; 4] = (bytes[start..(start + 4)]).try_into().unwrap();
        // debug!("[S7ParsePoint<f32>.convert] raw: {:?}", raw);
        match bytes[start..(start + 4)].try_into() {
            // Ok(v) => Ok(f32::from_le_bytes(v)),
            Ok(v) => Ok(f32::from_be_bytes(v)),
            Err(e) => {
                debug!("[S7ParsePoint<f32>.convert] error: {}", e);
                Err(e)
            }
        }
    }
    ///
    /// 
    fn toPoint(&self) -> Option<PointType> {
        if self.isChanged {
            Some(PointType::Float(Point::new(
                self.txId, 
                &self.name, 
                self.value as f64,
                self.status, 
                self.timestamp
            )))
            // debug!("{} point Bool: {:?}", self.id, dsPoint.value);
        } else {
            None
        }
    }
    //
    //
    fn addRawSimple(&mut self, bytes: &Vec<u8>) {
        self.addRaw(bytes, Utc::now())
    }    
    //
    //
    fn addRaw(&mut self, bytes: &Vec<u8>, timestamp: DateTime<Utc>) {
        let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
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
                warn!("[S7ParsePoint<f32>.addRaw] convertion error: {:?}", e);
            }
        }
    }    
}
///
/// 
impl ParsePoint for S7ParseReal {
    //
    //
    fn nextSimple(&mut self, bytes: &Vec<u8>) -> Option<PointType> {
        self.addRawSimple(bytes);
        self.toPoint()
    }
    //
    //
    fn next(&mut self, bytes: &Vec<u8>, timestamp: DateTime<Utc>) -> Option<PointType> {
        self.addRaw(bytes, timestamp);
        self.toPoint()
    }
    //
    //
    fn nextStatus(&mut self, bytes: &Vec<u8>, timestamp: DateTime<Utc>) -> Option<PointType> {
        self.status = Status::Ok;
        self.timestamp = Utc::now();
        self.toPoint()
    }
    //
    //
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::array::TryFromSliceError;
use chrono::{DateTime, Utc};
use log::{
    // info,
    debug,
    warn,
    // error,
};
use crate::{
    core_::{point::{point::Point, point_type::PointType}, status::status::Status, types::bool::Bool},
    conf::point_config::{point_config::PointConfig, point_config_address::PointConfigAddress, point_config_type::PointConfigType},
};

///
/// 
#[derive(Debug, Clone)]
pub enum ParsePointType {
    Bool(ParsePointBool),
    Int(ParsePointInt),
    Real(ParsePointReal),
}

pub trait ParsePoint<T> {
    fn next(&mut self, bytes: &Vec<u8>) -> Option<PointType>;
    fn addRaw(&mut self, bytes: &Vec<u8>);
    ///
    /// Returns true if value or status was updated since last call [addRaw()]
    fn isChanged(&self) -> bool;
}

///
///
#[derive(Debug, Clone)]
pub struct ParsePointBool {
    pub txId: usize,
    pub path: String,
    pub name: String,
    pub value: bool,
    pub status: Status,
    pub dataType: PointConfigType,
    pub offset: Option<u32>,
    pub bit: Option<u8>,
    pub h: Option<u8>,
    pub a: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
    isChanged: bool,
}
impl ParsePointBool {
    ///
    /// 
    pub fn new(
        path: String,
        name: String,
        config: &PointConfig,
        // filter: Filter<T>,
    ) -> ParsePointBool {
        ParsePointBool {
            txId: 0,
            path: path,
            name: name,
            value: false,
            status: Status::Invalid,
            isChanged: false,
            dataType: config._type.clone(),
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
}
///
impl ParsePoint<bool> for ParsePointBool {
    //
    //
    fn next(&mut self, bytes: &Vec<u8>) -> Option<PointType> {
        self.addRaw(bytes);
        if self.isChanged {
            Some(PointType::Bool(Point::new(
                self.txId, 
                &self.name, 
                Bool(self.value), 
                self.status as u8, 
                self.timestamp
            )))
            // debug!("{} point Bool: {:?}", self.id, dsPoint.value);
        } else {
            None
        }
    }
    //
    //
    fn addRaw(&mut self, bytes: &Vec<u8>) {
        let timestamp = Utc::now();
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
    //
    //
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}
///
///
#[derive(Debug, Clone)]
pub struct ParsePointInt {
    pub txId: usize,
    pub path: String,
    pub name: String,
    pub value: i64,
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
impl ParsePointInt {
    ///
    /// 
    pub fn new(
        path: String,
        name: String,
        config: &PointConfig,
        // filter: Filter<T>,
        // convert: Function,
    ) -> ParsePointInt {
        ParsePointInt {
            txId: 0,
            path: path,
            name: name,
            value: 0,
            status: Status::Invalid,
            isChanged: false,
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
}
///
/// 
impl ParsePoint<i16> for ParsePointInt {
    //
    //
    fn next(&mut self, bytes: &Vec<u8>) -> Option<PointType> {
        self.addRaw(bytes);
        if self.isChanged {
            Some(PointType::Int(Point::new(
                self.txId, 
                &self.name, 
                self.value,
                self.status as u8, 
                self.timestamp
            )))
            // debug!("{} point Bool: {:?}", self.id, dsPoint.value);
        } else {
            None
        }
    }
    //
    // 
    fn addRaw(&mut self, bytes: &Vec<u8>) {
        let timestamp = Utc::now();
        let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
        match result {
            Ok(newVal) => {
                let newVal = newVal as i64;
                if newVal != self.value {
                    self.value = newVal as i64;
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
    //
    //
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}
///
///
#[derive(Debug, Clone)]
pub struct ParsePointReal {
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
impl ParsePointReal {
    ///
    /// 
    pub fn new(
        path: String,
        name: String,
        config: &PointConfig,
        // filter: Filter<T>,
    ) -> ParsePointReal {
        ParsePointReal {
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
}
///
/// 
impl ParsePoint<f32> for ParsePointReal {
    //
    //
    fn next(&mut self, bytes: &Vec<u8>) -> Option<PointType> {
        self.addRaw(bytes);
        if self.isChanged {
            Some(PointType::Float(Point::new(
                self.txId, 
                &self.name, 
                self.value as f64,
                self.status as u8, 
                self.timestamp
            )))
            // debug!("{} point Bool: {:?}", self.id, dsPoint.value);
        } else {
            None
        }
    }    
    //
    //
    fn addRaw(&mut self, bytes: &Vec<u8>) {
        let timestamp = Utc::now();
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
    //
    //
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}

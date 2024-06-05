use log::{debug, trace, warn};
use std::array::TryFromSliceError;
use chrono::{DateTime, Utc};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_address::PointConfigAddress, point_config_history::PointConfigHistory, point_config_type::PointConfigType},
    core_::{cot::cot::Cot, filter::filter::Filter, point::{point::Point, point_type::PointType}, status::status::Status},
    services::slmp_client::parse_point::ParsePoint,
};
///
/// Used for parsing configured point from slice of bytes read from device
#[derive(Debug)]
pub struct SlmpParseInt {
    pub type_: PointConfigType,
    pub tx_id: usize,
    pub name: String,
    pub value: Box<dyn Filter<Item = i64> + Sync + Send>,
    pub status: Status,
    pub offset: Option<u32>,
    pub history: PointConfigHistory,
    pub alarm: Option<u8>,
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
    is_changed: bool,
}
//
//
impl SlmpParseInt {
    ///
    ///
    pub fn new(
        tx_id: usize,
        name: String,
        config: &PointConfig,
        filter: Box<dyn Filter<Item = i64> + Sync + Send>,
    ) -> SlmpParseInt {
        SlmpParseInt {
            type_: config.type_.clone(),
            tx_id,
            name,
            value: filter,
            status: Status::Invalid,
            is_changed: false,
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
        trace!("SlmpParseInt.convert | start: {},  end: {:?}", start, start + 2);
        trace!("SlmpParseInt.convert | raw: {:02X?}", &bytes[start..(start + 2)]);
        trace!("SlmpParseInt.convert | converted i16: {:?}", i16::from_le_bytes(bytes[start..(start + 2)].try_into().unwrap()));
        match bytes[start..(start + 2)].try_into() {
            Ok(v) => Ok(i16::from_le_bytes(v)),
            Err(e) => {
                debug!("SlmpParseInt.convert | error: {}", e);
                Err(e)
            }
        }
    }
    ///
    ///
    fn to_point(&self) -> Option<PointType> {
        if self.is_changed {
            Some(PointType::Int(Point::new(
                self.tx_id,
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
    fn add_raw_simple(&mut self, bytes: &[u8]) {
        self.add_raw(bytes, Utc::now())
    }
    //
    //
    fn add_raw(&mut self, bytes: &[u8], timestamp: DateTime<Utc>) {
        let result = self.convert(bytes, self.offset.unwrap() as usize, 0);
        match result {
            Ok(new_val) => {
                let status = Status::Ok;
                let new_val = new_val as i64;
                if new_val != self.value.value() || self.status != status {
                    self.value.add(new_val);
                    self.status = status;
                    self.timestamp = timestamp;
                    self.is_changed = true;
                }
            }
            Err(e) => {
                self.status = Status::Invalid;
                warn!("SlmpParseInt.addRaw | convertion error: {:?}", e);
            }
        }
    }
}
//
//
impl ParsePoint for SlmpParseInt {
    //
    //
    fn type_(&self) -> PointConfigType {
        self.type_.clone()
    }
    //
    //
    fn next_simple(&mut self, bytes: &[u8]) -> Option<PointType> {
        self.add_raw_simple(bytes);
        self.to_point()
    }
    //
    //
    fn next(&mut self, bytes: &[u8], timestamp: DateTime<Utc>) -> Option<PointType> {
        self.add_raw(bytes, timestamp);
        self.to_point().map(|point| {
            self.is_changed = false;
            point
        })
    }
    //
    //
    fn next_status(&mut self, status: Status) -> Option<PointType> {
        if self.status != status {
            self.status = status;
            self.timestamp = Utc::now();
            self.is_changed = true;
        }
        self.to_point().map(|point| {
            self.is_changed = false;
            point
        })
    }
    //
    //
    fn is_changed(&self) -> bool {
        self.is_changed
    }
    //
    //
    fn address(&self) -> PointConfigAddress {
        PointConfigAddress { offset: self.offset, bit: None }
    }
}

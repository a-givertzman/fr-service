#![allow(non_snake_case)]

use std::{sync::mpsc::Sender, time::Duration};

use chrono::Utc;
use indexmap::IndexMap;
use log::{debug, warn};

use crate::{
    conf::{
        point_config::{point_config::PointConfig, point_config_filters::PointConfigFilter, point_config_type::PointConfigType}, 
        profinet_client_config::profinet_db_config::ProfinetDbConfig
    }, 
    core_::{
        filter::{filter::{Filter, FilterEmpty}, filter_threshold::FilterThreshold}, 
        point::point_type::PointType, status::status::Status
    }, 
    services::profinet_client::{
        parse_point::ParsePoint,
        s7::{
            s7_client::S7Client,
            s7_parse_bool::S7ParseBool,
            s7_parse_int::S7ParseInt,
            s7_parse_real::S7ParseReal,
        }
    }
};

///
/// Represents PROFINET DB - a collection of the PROFINET addresses
pub struct ProfinetDb {
    id: String,
    pub name: String,
    pub description: String,
    pub number: u32,
    pub offset: u32,
    pub size: u32,
    pub cycle: Option<Duration>,
    pub points: IndexMap<String, Box<dyn ParsePoint>>,
}
///
/// 
impl ProfinetDb {
    ///
    /// Creates new instance of the [ProfinetDb]
    pub fn new(parent: impl Into<String>, conf: ProfinetDbConfig) -> Self {
        let selfId = format!("{}/ProfinetDb({})", parent.into(), conf.name);
        Self {
            points: Self::configureParsePoints(&selfId, &conf),
            id: selfId.clone(),
            name: conf.name,
            description: conf.description,
            number: conf.number as u32,
            offset: conf.offset as u32,
            size: conf.size as u32,
            cycle: conf.cycle,
        }
    }
    ///
    /// Returns updated points from the current DB
    ///     - reads data slice from the S7 device,
    ///     - parses raw data into the configured points
    ///     - returns only points with updated value or status
    pub fn read(&mut self, client: &S7Client, txSend: &Sender<PointType>) -> Result<(), String> {
        if client.isConnected {
            debug!(
                "{}.read | reading DB: {:?}, offset: {:?}, size: {:?}",
                self.id, self.number, self.offset, self.size
            );
            match client.read(self.number, self.offset, self.size) {
                Ok(bytes) => {
                    let timestamp = Utc::now();
                    // let bytes = client.read(899, 0, 34).unwrap();
                    // print!("\x1B[2J\x1B[1;1H");
                    // debug!("{:?}", bytes);
                    let mut message = String::new();
                    for (_key, parsePoint) in &mut self.points {
                        if let Some(point) = parsePoint.next(&bytes, timestamp) {
                            match txSend.send(point) {
                                Ok(_) => {},
                                Err(err) => {
                                    message = format!("{}.read | send error: {}", self.id, err);
                                    warn!("{}", message);
                                },
                            }
                        }
                    }
                    if message.is_empty() {
                        Ok(())
                    } else {
                        Err(message)
                    }
                }
                Err(err) => {
                    let message = format!("{}.read | read error: {}", self.id, err);
                    warn!("{}", message);
                    Err(message)
                }
            }
        } else {
            let message = format!("{}.read | read error: {} - is not connected", self.id, client.id);
            warn!("{}", message);
            Err(message)
        }        
    }
    ///
    /// Returns updated points from the current DB
    ///     - reads data slice from the S7 device,
    ///     - parses raw data into the configured points
    ///     - returns only points with updated value or status
    pub fn yieldStatus(&mut self, status: Status, txSend: &Sender<PointType>) -> Result<(), String> {
        let mut message = String::new();
        for (_key, parsePoint) in &mut self.points {
            if let Some(point) = parsePoint.nextStatus(status) {
                match txSend.send(point) {
                    Ok(_) => {},
                    Err(err) => {
                        message = format!("{}.sendStatus | send error: {}", self.id, err);
                        warn!("{}", message);
                    },
                }
            }
        }
        if message.is_empty() {
            return Ok(())
        }
        Err(message)
    }
    ///
    /// Writes point to the current DB
    ///     - Returns Ok() if succeed, Err(message) on fail
    pub fn write() {

    }
    ///
    /// Configuring ParsePoint objects depending on point configurations coming from [conf]
    fn configureParsePoints(selfId: &str, conf: &ProfinetDbConfig) -> IndexMap<String, Box<dyn ParsePoint>> {
        conf.points.iter().map(|pointConf| {
            // (pointConf.name.clone(), pointConf.clone())
            let path = String::new();
            match pointConf._type {
                PointConfigType::Bool => {
                    (pointConf.name.clone(), Self::boxBool(path, pointConf.name.clone(), pointConf))
                },
                PointConfigType::Int => {
                    (pointConf.name.clone(), Self::boxInt(path, pointConf.name.clone(), pointConf))
                },
                PointConfigType::Float => {
                    (pointConf.name.clone(), Self::boxFloat(path, pointConf.name.clone(), pointConf))
                },
                _ => panic!("{}.configureParsePoints | Unknown type '{:?}' for S7 Device", selfId, pointConf._type)
                // PointConfigType::String => {
                    
                // },
                // PointConfigType::Json => {
                    
                // },
            }
        }).collect()
    }
    ///
    /// 
    fn boxBool(path: String, name: String, config: &PointConfig) -> Box<dyn ParsePoint> {
        Box::new(S7ParseBool::new(path, name, config))
    }
    ///
    /// 
    fn boxInt(path: String, name: String, config: &PointConfig) -> Box<dyn ParsePoint> {
        Box::new(S7ParseInt::new(
            path, 
            name, 
            config,
            Self::intFilter(config.filters.clone()),
        ))
    }
    ///
    /// 
    fn boxFloat(path: String, name: String, config: &PointConfig) -> Box<dyn ParsePoint> {
        Box::new(S7ParseReal::new(
            path, 
            name, 
            config,
            Self::floatFilter(config.filters.clone()),
        ))
    }
    ///
    /// 
    fn intFilter(conf: Option<PointConfigFilter>) -> Box<dyn Filter<Item = i64>> {
        match conf {
            Some(conf) => {
                Box::new(
                    FilterThreshold::new(0, conf.threshold, conf.factor.unwrap_or(0.0))
                )
            },
            None => Box::new(FilterEmpty::new(0)),
        }
    }
    ///
    /// 
    fn floatFilter(conf: Option<PointConfigFilter>) -> Box<dyn Filter<Item = f64>> {
        match conf {
            Some(conf) => {
                Box::new(
                    FilterThreshold::new(0.0, conf.threshold, conf.factor.unwrap_or(0.0))
                )
            },
            None => Box::new(FilterEmpty::<f64>::new(0.0)),
        }
    }
}
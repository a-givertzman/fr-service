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
    pub fn new(parent: impl Into<String>, conf: &ProfinetDbConfig) -> Self {
        let self_id = format!("{}/ProfinetDb({})", parent.into(), conf.name);
        Self {
            points: Self::configure_parse_points(&self_id, conf),
            id: self_id.clone(),
            name: conf.name.clone(),
            description: conf.description.clone(),
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
    pub fn read(&mut self, client: &S7Client, tx_send: &Sender<PointType>) -> Result<(), String> {
        match client.is_connected() {
            Ok(is_connected) => {
                if is_connected {
                    debug!(
                        "{}.read | reading DB: {:?}, offset: {:?}, size: {:?}",
                        self.id, self.number, self.offset, self.size
                    );
                    match client.read(self.number, self.offset, self.size) {
                        Ok(bytes) => {
                            let timestamp = Utc::now();
                            let mut message = String::new();
                            for (_key, parse_point) in &mut self.points {
                                if let Some(point) = parse_point.next(&bytes, timestamp) {
                                    match tx_send.send(point) {
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
                    let message = format!("{}.read | read error: Is not connected", self.id);
                    warn!("{}", message);
                    Err(message)
                }        
            },
            Err(err) => {
                let message = format!("{}.read | read error: {}", self.id, err);
                warn!("{}", message);
                Err(message)
            },
        }
    }
    ///
    /// Returns updated points from the current DB
    ///     - reads data slice from the S7 device,
    ///     - parses raw data into the configured points
    ///     - returns only points with updated value or status
    pub fn yield_status(&mut self, status: Status, tx_send: &Sender<PointType>) -> Result<(), String> {
        let mut message = String::new();
        for (_key, parse_point) in &mut self.points {
            if let Some(point) = parse_point.next_status(status) {
                match tx_send.send(point) {
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
    pub fn write(&mut self, client: &S7Client, point: PointType) -> Result<(), String> {
        let mut message = String::new();
        match self.points.get(&point.name()) {
            Some(parse_point) => {
                let address = parse_point.address();
                match point {
                    PointType::Bool(point) => {
                        // !!! Not implemented because before write byte of the bool bits, that byte must be read from device
                        // let mut buf = [0; 16];
                        // let index = address.offset.unwrap() as usize;
                        // buf[index] = point.value.0 as u8;
                        // client.write(self.number, address.offset.unwrap(), 2, &mut buf)
                        message = format!("{}.write | Write 'Bool' to the S7 Device - not implemented, point: {:?}", self.id, point.name);
                        Err(message)
                    },
                    PointType::Int(point) => {
                        client.write(self.number, address.offset.unwrap(), 2, &mut point.value.to_be_bytes())
                    },
                    PointType::Float(point) => {
                        client.write(self.number, address.offset.unwrap(), 4, &mut point.value.to_be_bytes())
                    },
                    PointType::String(point) => {
                        message = format!("{}.write | Write 'String' to the S7 Device - not implemented, point: {:?}", self.id, point.name);
                        Err(message)
                    },
                }
            },
            None => {
                Err(message)
            },
        }
    }
    ///
    /// Configuring ParsePoint objects depending on point configurations coming from [conf]
    fn configure_parse_points(self_id: &str, conf: &ProfinetDbConfig) -> IndexMap<String, Box<dyn ParsePoint>> {
        conf.points.iter().map(|point_conf| {
            // (pointConf.name.clone(), pointConf.clone())
            let path = String::new();
            match point_conf._type {
                PointConfigType::Bool => {
                    (point_conf.name.clone(), Self::box_bool(path, point_conf.name.clone(), point_conf))
                },
                PointConfigType::Int => {
                    (point_conf.name.clone(), Self::box_int(path, point_conf.name.clone(), point_conf))
                },
                PointConfigType::Float => {
                    (point_conf.name.clone(), Self::box_float(path, point_conf.name.clone(), point_conf))
                },
                _ => panic!("{}.configureParsePoints | Unknown type '{:?}' for S7 Device", self_id, point_conf._type)
                // PointConfigType::String => {
                    
                // },
                // PointConfigType::Json => {
                    
                // },
            }
        }).collect()
    }
    ///
    /// 
    fn box_bool(path: String, name: String, config: &PointConfig) -> Box<dyn ParsePoint> {
        Box::new(S7ParseBool::new(path, name, config))
    }
    ///
    /// 
    fn box_int(path: String, name: String, config: &PointConfig) -> Box<dyn ParsePoint> {
        Box::new(S7ParseInt::new(
            path, 
            name, 
            config,
            Self::int_filter(config.filters.clone()),
        ))
    }
    ///
    /// 
    fn box_float(path: String, name: String, config: &PointConfig) -> Box<dyn ParsePoint> {
        Box::new(S7ParseReal::new(
            path, 
            name, 
            config,
            Self::float_filter(config.filters.clone()),
        ))
    }
    ///
    /// 
    fn int_filter(conf: Option<PointConfigFilter>) -> Box<dyn Filter<Item = i64>> {
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
    fn float_filter(conf: Option<PointConfigFilter>) -> Box<dyn Filter<Item = f64>> {
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
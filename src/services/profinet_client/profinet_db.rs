#![allow(non_snake_case)]

use std::time::Duration;

use indexmap::IndexMap;
use log::{debug, warn};

use crate::{conf::{point_config::point_config_type::PointConfigType, profinet_client_config::profinet_db_config::ProfinetDbConfig}, core_::point::point_type::PointType};

use super::s7::{s7_client::S7Client, s7_parse_point::{ParsePointBool, ParsePointInt, ParsePointReal, ParsePointType}};

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
    pub points: IndexMap<String, ParsePointType>,
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
    pub fn read(&self, client: &S7Client) -> Result<Vec<PointType>, String> {
        if client.isConnected {
            debug!(
                "{}.read | reading DB: {:?}, offset: {:?}, size: {:?}",
                self.id, self.number, self.offset, self.size
            );
            match client.read(self.number, self.offset, self.size) {
                Ok(bytes) => {
                    // let bytes = client.read(899, 0, 34).unwrap();
                    // print!("\x1B[2J\x1B[1;1H");
                    // debug!("{:?}", bytes);
                    for (_key, pointType) in &self.points {
                        // match pointType.clone() {
                        //     ParsePointType::Bool(mut point) => {
                        //         point.addRaw(&bytes);
                        //         // debug!("{} parsed point Bool: {:?}", logPref, point);
                        //         if point.isChanged() {
                        //             let dsPoint = DsPoint::newBool(
                        //                 point.name.as_str(),
                        //                 false,
                        //                 DsStatus::Ok,
                        //                 point.timestamp,
                        //                 point.h,
                        //                 point.a,
                        //             );
                        //             // sender.push(value)
                        //             debug!(
                        //                 "{} point Bool: {:?}",
                        //                 logPref, dsPoint.value
                        //             );
                        //         }
                        //     }
                        //     ParsePointType::Int(mut point) => {
                        //         point.addRaw(&bytes);
                        //         // debug!("{} parsed point Int: {:?}", logPref, point);
                        //         if point.isChanged() {
                        //             let dsPoint = DsPoint::newInt(
                        //                 point.name.as_str(),
                        //                 0,
                        //                 DsStatus::Ok,
                        //                 point.timestamp,
                        //                 point.h,
                        //                 point.a,
                        //             );
                        //             // sender.push(value)
                        //             debug!(
                        //                 "{} point Int: {:?}",
                        //                 logPref, dsPoint.value
                        //             );
                        //         }
                        //     }
                        //     ParsePointType::Real(mut point) => {
                        //         point.addRaw(&bytes);
                        //         // debug!("{} parsed point Real: {:?}", logPref, point);
                        //         if point.isChanged() {
                        //             let dsPoint = DsPoint::newReal(
                        //                 point.name.as_str(),
                        //                 point.value,
                        //                 DsStatus::Ok,
                        //                 point.timestamp,
                        //                 point.h,
                        //                 point.a,
                        //             );
                        //             // debug!("{} point (Real): {:?} {:?}", logPref, dsPoint.name, dsPoint.value);
                        //             sender.push(dsPoint).unwrap();
                        //         }
                        //     }
                        // }
                    }
                    Ok(vec![])
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
    /// 
    fn configureParsePoints(selfId: &str, conf: &ProfinetDbConfig) -> IndexMap<String, ParsePointType> {
        conf.points.iter().map(|pointConf| {
            // (pointConf.name.clone(), pointConf.clone())
            let path = String::new();
            match pointConf._type {
                PointConfigType::Bool => {
                    (pointConf.name.clone(), ParsePointType::Bool(ParsePointBool::new(path, pointConf.name.clone(), pointConf)))
                },
                PointConfigType::Int => {
                    (pointConf.name.clone(), ParsePointType::Int(ParsePointInt::new(path, pointConf.name.clone(), pointConf)))
                },
                PointConfigType::Float => {
                    (pointConf.name.clone(), ParsePointType::Real(ParsePointReal::new(path, pointConf.name.clone(), pointConf)))
                },
                _ => panic!("{}.configureParsePoints | Unknown type '{:?}' for S7 Device", selfId, pointConf._type)
                // PointConfigType::String => {
                    
                // },
                // PointConfigType::Json => {
                    
                // },
            }
        }).collect()
        // match &conf.points {
        //     None => (),
        //     Some(confPoints) => {
        //         for (pointKey, point) in confPoints {
        //             // debug!("\t\t\tdb {:?}: {:?}", pointKey, &point);
        //             let dataType = &point.dataType.clone().unwrap();
        //             if *dataType == "Bool".to_string() {
        //                 dbPoints.insert(
        //                     pointKey.clone(),
        //                     ParsePointType::Bool(S7ParsePointBool::new(
        //                         pointKey.clone(),
        //                         pointKey.clone(),
        //                         point,
        //                     )),
        //                 );
        //             } else if *dataType == "Int".to_string() {
        //                 dbPoints.insert(
        //                     pointKey.clone(),
        //                     ParsePointType::Int(S7ParsePointInt::new(
        //                         pointKey.clone(),
        //                         pointKey.clone(),
        //                         point,
        //                     )),
        //                 );
        //             } else if *dataType == "Real".to_string() {
        //                 dbPoints.insert(
        //                     pointKey.clone(),
        //                     ParsePointType::Real(S7ParsePointReal::new(
        //                         pointKey.clone(),
        //                         pointKey.clone(),
        //                         point,
        //                     )),
        //                 );
        //             } else {
        //                 error!(
        //                     "{} point {:?}: uncnoun data type {:?}",
        //                     logPref, pointKey, dataType
        //                 );
        //             }
        //         }
        //     }
        // }
    }
}
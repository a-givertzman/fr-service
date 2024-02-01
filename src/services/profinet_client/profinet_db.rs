#![allow(non_snake_case)]

use std::time::Duration;

use indexmap::IndexMap;
use log::{debug, warn};

use crate::{conf::{point_config::point_config::PointConfig, profinet_client_config::profinet_db_config::ProfinetDbConfig}, core_::point::point_type::PointType};

use super::s7::s7_client::S7Client;

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
    pub points: IndexMap<String, PointConfig>,
}
///
/// 
impl ProfinetDb {
    ///
    /// Creates new instance of the [ProfinetDb]
    pub fn new(parent: impl Into<String>, conf: ProfinetDbConfig) -> Self {
        Self {
            id: format!("{}/ProfinetDb({})", parent.into(), conf.name),
            name: conf.name,
            description: conf.description,
            number: conf.number as u32,
            offset: conf.offset as u32,
            size: conf.size as u32,
            cycle: conf.cycle,
            points: conf.points.iter().map(|pointConf| {
                (pointConf.name.clone(), pointConf.clone())
            }).collect(),
        }
    }
    ///
    /// 
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
}
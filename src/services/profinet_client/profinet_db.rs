#![allow(non_snake_case)]

use std::time::Duration;

use indexmap::IndexMap;

use crate::conf::{point_config::point_config::PointConfig, profinet_client_config::profinet_db_config::ProfinetDbConfig};

///
/// 
pub struct ProfinetDb {
    id: String,
    pub name: String,
    pub description: String,
    pub number: u64,
    pub offset: u64,
    pub size: u64,
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
            number: conf.number,
            offset: conf.offset,
            size: conf.size,
            cycle: conf.cycle,
            points: conf.points.iter().map(|pointConf| {
                (pointConf.name.clone(), pointConf.clone())
            }).collect(),
        }
    }
}
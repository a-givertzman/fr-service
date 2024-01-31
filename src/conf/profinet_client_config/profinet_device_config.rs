use crate::conf::{
    point_config::point_config::PointConfig,
    profinet_client_config::profinet_db_config::ProfinetDbConfig,
};

///
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ProfinetDeviceConfig {
    pub(crate) dbs: Vec<ProfinetDbConfig>,
}
///
/// 
impl ProfinetDeviceConfig {
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.dbs.iter().fold(vec![], |mut points, dbConf| {
            points.extend(dbConf.points());
            points
        })
    }
}
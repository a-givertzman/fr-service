use crate::conf::point_config::point_config::PointConfig;

///
/// 
#[derive(Debug, PartialEq, Clone)]
pub struct ProfinetDbConfig {
    pub(crate) points: Vec<PointConfig>,
}
///
/// 
impl ProfinetDbConfig {
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        self.points.iter().fold(vec![], |mut points, conf| {
            points.push(conf.clone());
            points
        })
    }
}
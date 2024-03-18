use crate::conf::{
    point_config::point_config::PointConfig,
    fn_::fn_conf_kind::FnConfKind,
};


///
/// Reperesents configuration of the point with it input as point value source 
#[derive(Debug, PartialEq, Clone)]
pub struct FnPointConfig {
    pub conf: PointConfig,
    pub input: Box<FnConfKind>,
}
///
/// 
impl FnPointConfig {
    ///
    /// Returns list of configurations of the defined points
    pub fn points(&self) -> Vec<PointConfig> {
        let mut points = self.input.points();
        points.push(self.conf.clone());
        points
    }
}
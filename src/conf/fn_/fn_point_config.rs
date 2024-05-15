use crate::conf::{
    point_config::point_config::PointConfig,
    fn_::fn_conf_kind::FnConfKind,
};


///
/// Represents configuration of the point in the NestedFn
///  - send-to - Service.Queue where the point will be sent
///  - input - the source of the point value  
#[derive(Debug, PartialEq, Clone)]
pub struct FnPointConfig {
    pub conf: PointConfig,
    pub send_to: Option<String>,
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
use chrono::Utc;
use crate::{
    conf::point_config::point_config::PointConfig,
    core_::{
        cot::cot::Cot,
        point::{point::Point, point_type::PointType},
        status::status::Status,
    }
};
///
/// Provides state for diagnosis points
pub struct DiagPoint {
    tx_id: usize,
    conf: PointConfig,
    value: Status,
}
///
///
impl DiagPoint {
    ///
    ///
    pub fn new(tx_id: usize, conf: PointConfig) -> Self {
        Self {
            tx_id,
            conf,
            value: Status::Unknown(-1),
        }
    }
    ///
    /// 
    fn point(&self, value: Status) -> PointType {
        PointType::Int(Point::new(
            self.tx_id,
            &self.conf.name,
            i64::from(value),
            Status::Ok,
            Cot::Inf,
            Utc::now(),
        ))
    }
    ///
    /// Returns updated point with
    pub fn next(&mut self, value: Status) -> Option<PointType> {
        if value != self.value {
            self.value = value;
            Some(self.point(value))
        } else {
            None
        }
    }
}
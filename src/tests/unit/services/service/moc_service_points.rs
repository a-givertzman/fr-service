//!
//! MockServicePoints implements points() method only.
//! Which returns exactly the vector from which it was created
use log::debug;
use crate::{conf::point_config::point_config::PointConfig, core_::object::object::Object, services::service::{service::Service, service_handles::ServiceHandles}};
///
/// MockServicePoints implements points() method only.
/// Which returns exactly the vector from which it was created
pub struct MockServicePoints {
    id: String,
    points: Vec<PointConfig>,
}
///
/// 
impl MockServicePoints {
    ///
    /// 
    pub fn new(parent: impl Into<String>, points: Vec<PointConfig>) -> Self {
        Self {
            id: format!("{}/MockServicePoints", parent.into()),
            points,
        }
    }
}
///
/// 
impl Object for MockServicePoints {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl Service for MockServicePoints {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        let message = format!("{}.run | Not implemented", self.id);
        debug!("{}", message);
        Err(message)
    }
    ///
    /// 
    fn exit(&self) {
        debug!("{}.run | Not implemented", self.id);
    }    
    fn points(&self) -> Vec<PointConfig> {
        debug!("{}.points | Returning: {:#?}", self.id, self.points);
        self.points.clone()
    }
}
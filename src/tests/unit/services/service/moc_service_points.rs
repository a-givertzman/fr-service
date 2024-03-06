//!
//! MockServicePoints implements points() method only.
//! Which returns exactly the vector from which it was created
use std::thread::JoinHandle;
use log::debug;
use crate::{conf::point_config::point_config::PointConfig, core_::object::object::Object, services::service::service::Service};
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
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        let message = format!("{}.run | Not implemented", self.id);
        let error = std::io::Error::new(std::io::ErrorKind::Unsupported, message);
        debug!("{}", error);
        Err(error)
    }
    ///
    /// 
    fn exit(&self) {
        debug!("{}.run | Not implemented", self.id);
    }    
    fn points(&self) -> Vec<PointConfig> {
        self.points.clone()
    }
}
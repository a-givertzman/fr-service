use crate::core_::{object::object::Object, point::point_type::PointType};

pub trait StreamRead<T: Sync, E>: Sync + Object {
    fn read(&mut self) -> Result<T, E>;
    fn read_filtered(&mut self, filter: &StreamFilter) -> Result<Option<T>, E> {
        let _ = filter;
        panic!("{}.read_filtered | Does not supported", self.id())
    }
}
///
/// 
#[derive(Debug, Clone)]
pub struct StreamFilter {
    cot: Option<u32>,
    name: Option<String>,
}
///
/// 
impl StreamFilter {
    ///
    /// Creates new instance
    /// - cot - [Cot] - bit mask wich will be passed
    /// - name - exact name wich passed
    pub fn allow(cot: Option<u32>, name: Option<String>) -> Self {
        Self { cot, name }
    }
    ///
    /// Returns true if any filter creteria matched
    pub fn pass(&self, point: &PointType) -> bool {
        match &self.cot {
            Some(cot) => {
                if *cot & point.cot() > 0 {
                    true
                } else {
                    match &self.name {
                        Some(name) => {
                            if name == &point.name() {
                                true
                            } else {
                                false
                            }
                        },
                        None => {
                            false
                        },
                    }
                }
            },
            None => {
                match &self.name {
                    Some(name) => {
                        if name == &point.name() {
                            true
                        } else {
                            false
                        }
                    },
                    None => false,
                }
            },
        }
    }
}
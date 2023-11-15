use std::sync::mpsc::Sender;

use crate::core_::point::point_type::PointType;

pub trait Service {
    fn getLink(&self, name: impl Into<String>) -> Sender<PointType>;
    fn run(&mut self);
    fn exit(&self);
}
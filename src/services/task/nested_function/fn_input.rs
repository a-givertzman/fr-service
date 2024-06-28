use log::{error, trace};
use concat_string::concat_string;
use std::{fmt::Debug, sync::atomic::{AtomicUsize, Ordering}};
use crate::{conf::fn_::fn_conf_keywd::FnConfPointType, core_::{point::{point::Point, point_type::PointType}, types::bool::Bool}};
use super::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind, fn_result::FnResult};
///
/// 
#[derive(Debug, Clone)]
pub struct FnInput {
    id: String,
    kind: FnKind,
    name: String,
    type_: FnConfPointType,
    point: Option<PointType>,
    initial: Option<PointType>,
}
//
// 
impl FnInput {
    pub fn new(parent: &str, name: impl Into<String>, initial: Option<PointType>, type_: FnConfPointType) -> Self {
        Self {
            id: format!("{}/FnInput{}", parent, COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Input,
            name: name.into(),
            type_,
            point: initial.clone(), 
            initial
        }
    }
}
//
// 
impl FnIn for FnInput {
    fn add(&mut self, point: PointType) {
        trace!("{}.add | value: {:?}", self.id, &self.point);
        let point = match self.type_ {
            FnConfPointType::Bool => {
                match point {
                    PointType::Bool(_) => point,
                    PointType::Int(p) => PointType::Bool(Point::new(p.tx_id, &p.name, Bool(p.value > 0), p.status, p.cot, p.timestamp)),
                    PointType::Real(p) => PointType::Bool(Point::new(p.tx_id, &p.name, Bool(p.value > 0.0), p.status, p.cot, p.timestamp)),
                    PointType::Double(p) => PointType::Bool(Point::new(p.tx_id, &p.name, Bool(p.value > 0.0), p.status, p.cot, p.timestamp)),
                    PointType::String(p) => {
                        match p.value.parse() {
                            Ok(value) => PointType::Bool(Point::new(p.tx_id, &p.name, Bool(value), p.status, p.cot, p.timestamp)),
                            Err(err) => {
                                error!("{}.add | Error conversion into<bool> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                                return;
                            }
                        }
                    }
                }
            }
            FnConfPointType::Int => {
                match point {
                    PointType::Bool(p) => PointType::Int(Point::new(p.tx_id, &p.name, if p.value.0 {1} else {0}, p.status, p.cot, p.timestamp)),
                    PointType::Int(p) => PointType::Int(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp)),
                    PointType::Real(p) => PointType::Int(Point::new(p.tx_id, &p.name, p.value.round() as i64, p.status, p.cot, p.timestamp)),
                    PointType::Double(p) => PointType::Int(Point::new(p.tx_id, &p.name, p.value.round() as i64, p.status, p.cot, p.timestamp)),
                    PointType::String(p) => {
                        match p.value.parse() {
                            Ok(value) => PointType::Int(Point::new(p.tx_id, &p.name, value, p.status, p.cot, p.timestamp)),
                            Err(err) => {
                                error!("{}.add | Error conversion into<i64> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                                return;
                            }
                        }
                    }
                }
            }
            FnConfPointType::Real => {
                match point {
                    PointType::Bool(p) => {
                        PointType::Real(Point::new(p.tx_id, &p.name, if p.value.0 {1.0} else {0.0}, p.status, p.cot, p.timestamp))
                    }
                    PointType::Int(p) => {
                        PointType::Real(Point::new(p.tx_id, &p.name, p.value as f32, p.status, p.cot, p.timestamp))
                    }
                    PointType::Real(p) => {
                        PointType::Real(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp))
                    }
                    PointType::Double(p) => {
                        PointType::Real(Point::new(p.tx_id, &p.name, p.value as f32, p.status, p.cot, p.timestamp))
                    }
                    PointType::String(p) => {
                        match p.value.parse() {
                            Ok(value) => PointType::Real(Point::new(p.tx_id, &p.name, value, p.status, p.cot, p.timestamp)),
                            Err(err) => {
                                error!("{}.add | Error conversion into<f32> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                                return;
                            }
                        }
                    }
                }
            }
            FnConfPointType::Double => {
                match point {
                    PointType::Bool(p) => {
                        PointType::Double(Point::new(p.tx_id, &p.name, if p.value.0 {1.0} else {0.0}, p.status, p.cot, p.timestamp))
                    }
                    PointType::Int(p) => {
                        PointType::Double(Point::new(p.tx_id, &p.name, p.value as f64, p.status, p.cot, p.timestamp))
                    }
                    PointType::Real(p) => {
                        PointType::Double(Point::new(p.tx_id, &p.name, p.value as f64, p.status, p.cot, p.timestamp))
                    }
                    PointType::Double(p) => {
                        PointType::Double(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp))
                    }
                    PointType::String(p) => {
                        match p.value.parse() {
                            Ok(value) => PointType::Double(Point::new(p.tx_id, &p.name, value, p.status, p.cot, p.timestamp)),
                            Err(err) => {
                                error!("{}.add | Error conversion into<f64> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                                return;
                            }
                        }
                    }
                }
            }
            FnConfPointType::String => {
                match point {
                    PointType::Bool(p) => {
                        PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp))
                    }
                    PointType::Int(p) => {
                        PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp))
                    }
                    PointType::Real(p) => {
                        PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp))
                    }
                    PointType::Double(p) => {
                        PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp))
                    }
                    PointType::String(p) => {
                        PointType::String(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp))
                    }
                }
            }
            FnConfPointType::Any => {
                point
            }
            FnConfPointType::Unknown => {
                panic!("{}.add | Error. FnInput does not supports unknown type, but configured in: {:#?}", self.id, self);
            }
        };
        self.point = Some(point)
    }
}
//
// 
impl FnOut for FnInput {
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    fn kind(&self) -> &FnKind {
        &self.kind
    }
    //
    fn inputs(&self) -> Vec<String> {
        vec![self.name.clone()]
    }
    //
    fn out(&mut self) -> FnResult<PointType, String> {
        trace!("{}.out | value: {:?}", self.id, &self.point);
        match &self.point {
            Some(point) => FnResult::Ok(point.to_owned()),
            None => FnResult::Err(concat_string!(self.id, ".out | Not initialized")),
        }
    }
    //
    fn reset(&mut self) {
        self.point = self.initial.clone();
    }
}
//
// 
impl FnInOut for FnInput {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

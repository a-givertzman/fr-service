use log::{error, trace};
use std::{fmt::Debug, sync::atomic::{AtomicUsize, Ordering}};
use crate::{
    core_::{point::{point::Point, point_type::PointType}, types::bool::Bool},
    conf::fn_::fn_conf_keywd::FnConfPointType,
    services::task::nested_function::{
        fn_::{FnIn, FnInOut, FnOut, FnResult},
        fn_kind::FnKind,
    }
};
///
/// Function | Provide receiving Point into the computing node
///  - Returns Point if it was received in the current cycle
///  - Returns None if nothing received
#[derive(Debug, Clone)]
pub struct FnInput {
    id: String,
    name: String,
    kind: FnKind,
    type_: FnConfPointType,
    point: FnResult,
    initial: FnResult,
}
///
/// 
impl FnInput {
    ///
    /// - name: &str - name of the associated point
    /// - initial: Option<PointType> - initial value of the input
    pub fn new(parent: &str, name: &str, initial: Option<PointType>, type_: FnConfPointType) -> Self {
        let initial = initial.map_or(FnResult::None, |point| FnResult::Ok(point));
        Self {
            id: format!("{}/FnInput{}", parent, COUNT.fetch_add(1, Ordering::Relaxed)),
            name: name.to_owned(),
            kind: FnKind::Input,
            type_,
            point: initial.clone(),
            initial,
        }
    }
}
///
/// 
impl FnIn for FnInput {
    fn add(&mut self, point: PointType) {
        trace!("{}.add | value: {:?}", self.id, &self.point);
        self.point = match self.type_ {
            FnConfPointType::Bool => {
                match point {
                    PointType::Bool(_) => FnResult::Ok(point),
                    PointType::Int(p) => FnResult::Ok(PointType::Bool(Point::new(p.tx_id, &p.name, Bool(p.value > 0), p.status, p.cot, p.timestamp))),
                    PointType::Real(p) => FnResult::Ok(PointType::Bool(Point::new(p.tx_id, &p.name, Bool(p.value > 0.0), p.status, p.cot, p.timestamp))),
                    PointType::Double(p) => FnResult::Ok(PointType::Bool(Point::new(p.tx_id, &p.name, Bool(p.value > 0.0), p.status, p.cot, p.timestamp))),
                    PointType::String(p) => match p.value.parse() {
                        Ok(value) => FnResult::Ok(PointType::Bool(Point::new(p.tx_id, &p.name, Bool(value), p.status, p.cot, p.timestamp))),
                        Err(err) => {
                            let message = format!("{}.add | Error conversion into<bool> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                            error!("{}", message);
                            FnResult::Err(message)
                        }
                    }
                }
            }
            FnConfPointType::Int => {
                match point {
                    PointType::Bool(p) => FnResult::Ok(PointType::Int(Point::new(p.tx_id, &p.name, if p.value.0 {1} else {0}, p.status, p.cot, p.timestamp))),
                    PointType::Int(p) => FnResult::Ok(PointType::Int(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp))),
                    PointType::Real(p) => FnResult::Ok(PointType::Int(Point::new(p.tx_id, &p.name, p.value.round() as i64, p.status, p.cot, p.timestamp))),
                    PointType::Double(p) => FnResult::Ok(PointType::Int(Point::new(p.tx_id, &p.name, p.value.round() as i64, p.status, p.cot, p.timestamp))),
                    PointType::String(p) => match p.value.parse() {
                        Ok(value) => FnResult::Ok(PointType::Int(Point::new(p.tx_id, &p.name, value, p.status, p.cot, p.timestamp))),
                        Err(err) => {
                            let message = format!("{}.add | Error conversion into<i64> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                            error!("{}", message);
                            FnResult::Err(message)
                        }
                    }
                }
            }
            FnConfPointType::Real => {
                match point {
                    PointType::Bool(p) => {
                        FnResult::Ok(PointType::Real(Point::new(p.tx_id, &p.name, if p.value.0 {1.0} else {0.0}, p.status, p.cot, p.timestamp)))
                    }
                    PointType::Int(p) => {
                        FnResult::Ok(PointType::Real(Point::new(p.tx_id, &p.name, p.value as f32, p.status, p.cot, p.timestamp)))
                    }
                    PointType::Real(p) => {
                        FnResult::Ok(PointType::Real(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp)))
                    }
                    PointType::Double(p) => {
                        FnResult::Ok(PointType::Real(Point::new(p.tx_id, &p.name, p.value as f32, p.status, p.cot, p.timestamp)))
                    }
                    PointType::String(p) => match p.value.parse() {
                        Ok(value) => FnResult::Ok(PointType::Real(Point::new(p.tx_id, &p.name, value, p.status, p.cot, p.timestamp))),
                        Err(err) => {
                            let message = format!("{}.add | Error conversion into<f32> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                            error!("{}", message);
                            FnResult::Err(message)
                        }
                    }
                }
            }
            FnConfPointType::Double => {
                match point {
                    PointType::Bool(p) => {
                        FnResult::Ok(PointType::Double(Point::new(p.tx_id, &p.name, if p.value.0 {1.0} else {0.0}, p.status, p.cot, p.timestamp)))
                    }
                    PointType::Int(p) => {
                        FnResult::Ok(PointType::Double(Point::new(p.tx_id, &p.name, p.value as f64, p.status, p.cot, p.timestamp)))
                    }
                    PointType::Real(p) => {
                        FnResult::Ok(PointType::Double(Point::new(p.tx_id, &p.name, p.value as f64, p.status, p.cot, p.timestamp)))
                    }
                    PointType::Double(p) => {
                        FnResult::Ok(PointType::Double(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp)))
                    }
                    PointType::String(p) => {
                        match p.value.parse() {
                            Ok(value) => FnResult::Ok(PointType::Double(Point::new(p.tx_id, &p.name, value, p.status, p.cot, p.timestamp))),
                            Err(err) => {
                                let message = format!("{}.add | Error conversion into<f64> value: {:?}\n\terror: {:#?}", self.id, self.point, err);
                                error!("{}", message);
                                FnResult::Err(message)
                            }
                        }
                    }
                }
            }
            FnConfPointType::String => {
                match point {
                    PointType::Bool(p) => {
                        FnResult::Ok(PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp)))
                    }
                    PointType::Int(p) => {
                        FnResult::Ok(PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp)))
                    }
                    PointType::Real(p) => {
                        FnResult::Ok(PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp)))
                    }
                    PointType::Double(p) => {
                        FnResult::Ok(PointType::String(Point::new(p.tx_id, &p.name, p.value.to_string(), p.status, p.cot, p.timestamp)))
                    }
                    PointType::String(p) => {
                        FnResult::Ok(PointType::String(Point::new(p.tx_id, &p.name, p.value, p.status, p.cot, p.timestamp)))
                    }
                }
            }
            FnConfPointType::Any => {
                FnResult::Ok(point)
            }
            FnConfPointType::Unknown => {
                panic!("{}.add | Error. FnInput does not supports unknown type, but configured in: {:#?}", self.id, self);
            }
        };
    }
}
///
/// 
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
    fn out(&mut self) -> FnResult {
        trace!("{}.out | value: {:?}", self.id, &self.point);
        self.point.clone()
    }
    //
    fn reset(&mut self) {
        self.point = self.initial.clone();
    }
}
///
/// 
impl FnInOut for FnInput {}
///
/// Global static counter of FnOut instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

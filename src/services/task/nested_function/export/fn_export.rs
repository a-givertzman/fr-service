use std::sync::{mpsc::Sender, atomic::{AtomicUsize, Ordering}};
use log::{debug, error};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_type::PointConfigType},
    core_::{
        point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, 
        types::{bool::Bool, fn_in_out_ref::FnInOutRef}
    }, 
    services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind},
};
///
/// Function | Used for export Point from Task service to another service
///  - Poiont will be sent to the queue only if:
///     - [enable] 
///         - if specified and is true (or [enable] > 0)
///         - if not specified - default is true
///     - send-to - is specified
///  - if point conf is not specified - input Point will be sent
///  - finally returns input Point to the parent function
#[derive(Debug)]
pub struct FnExport {
    id: String,
    tx_id: usize,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    conf: Option<PointConfig>,
    input: FnInOutRef,
    tx_send: Option<Sender<PointType>>,
}
//
//
impl FnExport {
    ///
    /// creates new instance of the FnExport
    /// - parent - the name of the parent entitie
    /// - enable - boolean (numeric) input enables the export if true (> 0)
    /// - conf - the configuration of the Point to be prodused, if None - input Point will be sent
    /// - input - incoming points
    /// - send - destination queue
    pub fn new(parent: impl Into<String>, enable: Option<FnInOutRef>, conf: Option<PointConfig>, input: FnInOutRef, send: Option<Sender<PointType>>) -> Self {
        let self_id = format!("{}/FnExport{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed));
        Self {
            id: self_id.clone(),
            tx_id: PointTxId::fromStr(&self_id),
            kind: FnKind::Fn,
            enable,
            conf,
            input,
            tx_send: send,
        }
    }
    ///
    /// Sending Point to the external service if 'send-to' specified
    fn send(&self, point: PointType) {
        if let Some(tx_send) = &self.tx_send {
            let (type_, name) = match &self.conf {
                Some(conf) => (conf._type.clone(), conf.name.clone()),
                None => (point.type_(), point.name()),
            };
            let point = match type_ {
                PointConfigType::Bool => {
                    PointType::Bool(Point::new(
                        self.tx_id, 
                        &name, 
                        Bool(point.value().as_bool()), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Int => {
                    PointType::Int(Point::new(
                        self.tx_id, 
                        &name, 
                        point.value().as_int(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Real => {
                    PointType::Real(Point::new(
                        self.tx_id, 
                        &name, 
                        point.value().as_real(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Double => {
                    PointType::Double(Point::new(
                        self.tx_id, 
                        &name, 
                        point.value().as_double(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::String => {
                    PointType::String(Point::new(
                        self.tx_id, 
                        &name, 
                        point.value().as_string(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Json => {
                    PointType::String(Point::new(
                        self.tx_id, 
                        &name, 
                        point.value().as_string(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
            };
            match tx_send.send(point.clone()) {
                Ok(_) => {
                    debug!("{}.out | Point sent: {:#?}", self.id, point);
                }
                Err(err) => {
                    error!("{}.out | Send error: {:#?}\n\t point: {:#?}", self.id, err, point);
                }
            };
        }
    }
}
//
//
impl FnIn for FnExport {}
//
//
impl FnOut for FnExport {
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
        let mut inputs = vec![];
        if let Some(enable) = &self.enable {
            inputs.append(&mut enable.borrow().inputs());
        }
        inputs.append(&mut self.input.borrow().inputs());
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let enable = match &self.enable {
            Some(enable) => enable.borrow_mut().out().to_bool().as_bool().value.0,
            None => true,
        };
        let point = self.input.borrow_mut().out();
        if enable {
            self.send(point.clone());
        }
        point
    }
    //
    fn reset(&mut self) {
        if let Some(enable) = &self.enable {
            enable.borrow_mut().reset();
        }
        self.input.borrow_mut().reset();
    }
}
//
//
impl FnInOut for FnExport {}
///
/// Global static counter of FnExport instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

use std::sync::{mpsc::Sender, atomic::{AtomicUsize, Ordering}};
use log::{debug, error};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_type::PointConfigType}, core_::{point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, types::{bool::Bool, fn_in_out_ref::FnInOutRef}}, services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind}
};
///
/// Specific function used for exports configured point into the Service.in-queue
///  - Poiont will be sent to the queue only if:
///     - [enable] 
///         - if provided and is true (possible pass > 0)
///         - if not provided - default is true
///     - queue name is provided
///     - Point was changed
///  - finally point will be passed to the parent function
#[derive(Debug)]
pub struct FnExport {
    id: String,
    tx_id: usize,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    conf: PointConfig,
    input: FnInOutRef,
    tx_send: Option<Sender<PointType>>,
}
///
///
impl FnExport {
    ///
    /// creates new instance of the FnExport
    /// - parent - the name of the parent entitie
    /// - enable - boolean (numeric) input enables the export if true (> 0)
    /// - conf - the configuration of the Point to be prodused
    /// - input - incoming points
    /// - send - destination queue
    pub fn new(parent: impl Into<String>, enable: Option<FnInOutRef>, conf: PointConfig, input: FnInOutRef, send: Option<Sender<PointType>>) -> Self {
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
    /// 
    fn send(&self, point: PointType) {
        if let Some(tx_send) = &self.tx_send {
            let point = match self.conf._type {
                PointConfigType::Bool => {
                    PointType::Bool(Point::new(
                        self.tx_id, 
                        &self.conf.name, 
                        Bool(point.value().as_bool()), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Int => {
                    PointType::Int(Point::new(
                        self.tx_id, 
                        &self.conf.name, 
                        point.value().as_int(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Real => {
                    PointType::Real(Point::new(
                        self.tx_id, 
                        &self.conf.name, 
                        point.value().as_real(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Double => {
                    PointType::Double(Point::new(
                        self.tx_id, 
                        &self.conf.name, 
                        point.value().as_double(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::String => {
                    PointType::String(Point::new(
                        self.tx_id, 
                        &self.conf.name, 
                        point.value().as_string(), 
                        point.status(), 
                        point.cot(), 
                        point.timestamp(),
                    ))
                }
                PointConfigType::Json => {
                    PointType::String(Point::new(
                        self.tx_id, 
                        &self.conf.name, 
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
///
///
impl FnIn for FnExport {
    //
    fn add(&mut self, _: PointType) {
        panic!("{}.add | method is not used", self.id);
    }
}
///
///
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
            Some(enable) => {
                let en = enable.borrow_mut().out();
                match en {
                    PointType::Bool(point) => point.value.0,
                    PointType::Int(point) => point.value > 0,
                    PointType::Real(point) => point.value > 0.0,
                    PointType::Double(point) => point.value > 0.0,
                    PointType::String(_) => panic!("{}.out | Type 'String' - is not supported for 'enable'", self.id),
                }
            }
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
///
///
impl FnInOut for FnExport {}
///
/// Global static counter of FnExport instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

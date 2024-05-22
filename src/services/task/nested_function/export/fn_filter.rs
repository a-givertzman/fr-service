use std::sync::{mpsc::Sender, atomic::{AtomicUsize, Ordering}};
use log::{debug, error};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_type::PointConfigType},
    core_::{point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, types::{bool::Bool, fn_in_out_ref::FnInOutRef}},
    services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind},
};
///
/// Function | Filtered input exports as configured point into the Service.in-queue
///  - Poiont will be sent to the queue only if:
///     - [pass] is true (or [pass] > 0)
///     - send-to - is specified
///     - Point was changed
///  - finally input Point will be returned to the parent function
#[derive(Debug)]
pub struct FnFilter {
    id: String,
    tx_id: usize,
    kind: FnKind,
    conf: PointConfig,
    input: FnInOutRef,
    pass: FnInOutRef,
    tx_send: Option<Sender<PointType>>,
    state: Option<PointType>,
}
//
//
impl FnFilter {
    ///
    /// creates new instance of the FnFilter
    /// - id - just for proper debugging
    /// - input - incoming points
    pub fn new(parent: impl Into<String>, conf: PointConfig, input: FnInOutRef, pass: FnInOutRef, send: Option<Sender<PointType>>) -> Self {
        let self_id = format!("{}/FnFilter{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed));
        Self {
            id: self_id.clone(),
            tx_id: PointTxId::fromStr(&self_id),
            kind: FnKind::Fn,
            conf,
            input,
            pass,
            tx_send: send,
            state: None,
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
//
//
impl FnIn for FnFilter {}
//
//
impl FnOut for FnFilter {
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
        inputs.append(&mut self.pass.borrow().inputs());
        inputs.append(&mut self.input.borrow().inputs());
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let input = self.input.borrow_mut().out();
        let pass_point = self.pass.borrow_mut().out();
        let pass = pass_point.to_bool().as_bool().value.0;
        debug!("{}.out | pass: {:?}", self.id, pass);
        match &self.state {
            Some(state) => {
                if input.value() != state.value() {
                    if pass {
                        self.state = Some(input.clone());
                        debug!("{}.out | Sending point: {:?}", self.id, input);
                        self.send(input.clone());
                    }
                }
            }
            None => {
                if pass {
                    self.state = Some(input.clone());
                    debug!("{}.out | Sending point: {:?}", self.id, input);
                    self.send(input.clone());
                }
            }
        }
        input
    }
    //
    fn reset(&mut self) {
        self.input.borrow_mut().reset();
        self.pass.borrow_mut().reset();
    }
}
//
//
impl FnInOut for FnFilter {}
///
/// Global static counter of FnFilter instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

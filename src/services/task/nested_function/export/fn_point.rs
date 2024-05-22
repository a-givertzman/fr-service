use std::sync::{mpsc::Sender, atomic::{AtomicUsize, Ordering}};
use log::{debug, error};
use crate::{
    conf::point_config::{point_config::PointConfig, point_config_type::PointConfigType}, core_::{point::{point::Point, point_tx_id::PointTxId, point_type::PointType}, types::{bool::Bool, fn_in_out_ref::FnInOutRef}}, services::task::nested_function::{fn_::{FnIn, FnInOut, FnOut}, fn_kind::FnKind}
};
///
/// Specific function used for exports configured point into the Service.in-queue
///  - Poiont will be sent to the queue only if:
///     - queue name provided
///     - Point was changed
///  - finally point will be passed to the parent function
#[derive(Debug)]
pub struct FnPoint {
    id: String,
    tx_id: usize,
    kind: FnKind,
    conf: PointConfig,
    input: Option<FnInOutRef>,
    tx_send: Option<Sender<PointType>>,
    state: Option<PointType>,
}
//
//
impl FnPoint {
    ///
    /// creates new instance of the FnPoint
    /// - id - just for proper debugging
    /// - input - incoming points
    pub fn new(parent: impl Into<String>, conf: PointConfig, input: Option<FnInOutRef>, send: Option<Sender<PointType>>) -> Self {
        let self_id = format!("{}/FnPoint{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed));
        Self {
            id: self_id.clone(),
            tx_id: PointTxId::fromStr(&self_id),
            kind: FnKind::Fn,
            conf,
            input,
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
impl FnIn for FnPoint {}
//
//
impl FnOut for FnPoint {
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
        match &self.input {
            Some(input) => input.borrow().inputs(),
            None => vec![],
        }
    }
    //
    fn out(&mut self) -> PointType {
        match &self.input {
            Some(input) => {
                let point = input.borrow_mut().out();
                match &self.state {
                    Some(state) => {
                        if &point != state {
                            self.state = Some(point.clone());
                            self.send(point.clone());
                        }
                    }
                    None => {
                        self.state = Some(point.clone());
                        self.send(point.clone());
                    }
                }
                point
            }
            None => panic!("{}.out | Input is not configured for the Point '{}'", self.id, self.conf.name),
        }
    }
    //
    fn reset(&mut self) {
        if let Some(input) = &self.input {
            input.borrow_mut().reset();
        }
    }
}
//
//
impl FnInOut for FnPoint {}
///
/// Global static counter of FnPoint instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

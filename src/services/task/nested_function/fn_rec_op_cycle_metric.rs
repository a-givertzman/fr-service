use std::sync::{atomic::{AtomicUsize, Ordering}, mpsc::Sender};
use log::{debug, error, warn};
use crate::core_::{
    point::{point::Point, point_type::PointType},
    types::{bool::Bool, fn_in_out_ref::FnInOutRef},
};
use super::{fn_::{FnInOut, FnOut, FnIn}, fn_kind::FnKind};
///
/// Function | Creates SQL requests on [op-cycle] falling edge:
/// - Operating cycle SQL request (id, start, stop)
/// - Operating cycle metrics SQL requests (cycle_id, pid, metric_id, value)
/// - Returns [enable] input
#[derive(Debug)]
pub struct FnRecOpCycleMetric {
    id: String,
    kind: FnKind,
    enable: Option<FnInOutRef>,
    tx_send: Option<Sender<PointType>>,
    op_cycle: FnInOutRef,
    inputs: Vec<FnInOutRef>,
    values: Vec<PointType>,
    prev: bool,
    rising: bool,
    falling: bool,
}
//
// 
impl FnRecOpCycleMetric {
    ///
    /// Creates new instance of the FnRecOpCycleMetric
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, enable: Option<FnInOutRef>, tx_send: Option<Sender<PointType>>, op_cycle: FnInOutRef, inputs: Vec<FnInOutRef>) -> Self {
        Self { 
            id: format!("{}/FnRecOpCycleMetric{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind:FnKind::Fn,
            enable,
            tx_send,
            op_cycle,
            inputs,
            values: vec![],
            prev: false,
            rising: false,
            falling: false,
        }
    }
    ///
    /// Sending Point to the external service if 'send-to' specified
    fn send(&self, point: &PointType) {
        match &self.tx_send {
            Some(tx_send) => match tx_send.send(point.clone()) {
                Ok(_) => {
                    debug!("{}.out | Point sent: {:#?}", self.id, point);
                }
                Err(err) => {
                    error!("{}.out | Send error: {:#?}\n\t point: {:#?}", self.id, err, point);
                }
            }
            None => warn!("{}.out | Point can't be sent - send-to is not specified", self.id),
        }
    }
}
//
// 
impl FnIn for FnRecOpCycleMetric {}
//
// 
impl FnOut for FnRecOpCycleMetric {
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
        inputs.append(&mut self.op_cycle.borrow().inputs());
        for input in &self.inputs {
            inputs.append(&mut input.borrow().inputs());
        }
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let enable = match &mut self.enable {
            Some(en) => en.borrow_mut().out().to_bool().as_bool().value.0,
            None => true,
        };
        let (op_cycle, tx_id, status, cot, timestamp) = {
            let op_cycle = self.op_cycle.borrow_mut().out();
            (
                op_cycle.to_bool().as_bool().value.0,
                op_cycle.tx_id(),
                op_cycle.status(),
                op_cycle.cot(),
                op_cycle.timestamp(),
            )
        };
        if op_cycle & (! self.prev) {
            warn!("{}.out | Operating Cycle - STARTED", self.id);
            self.rising = true;
            self.falling = false
        };
        if (! op_cycle) & self.prev {
            warn!("{}.out | Operating Cycle - FINISHED", self.id);
            self.falling = true;
            self.rising = false;
        };
        self.prev = op_cycle;
        if self.falling {
            self.falling = false;
            warn!("{}.out | Operating Cycle - SENDING...", self.id);
            let log_values: Vec<String> = self.values.iter().map(|point| {
                format!("{}: {}", point.name(), point.value().to_string())
            }).collect();
            warn!("{}.out | Operating Cycle - values ({}): {:#?}", self.id, self.values.len(), log_values);
            for value in &self.values {
                self.send(value);
            }
        }
        if self.rising && enable {
            warn!("{}.out | Operating Cycle - values", self.id);
            self.values.clear();
            for input in &self.inputs {
                let value = input.borrow_mut().out();
                warn!("{}.out | Operating Cycle - value: {:?}", self.id, value);
                self.values.push(value)
            }
            let log_values: Vec<String> = self.values.iter().map(|point| {
                format!("{}: {}", point.name(), point.value().to_string())
            }).collect();
            warn!("{}.out | Operating Cycle - values: {:#?}", self.id, log_values);
        }
        PointType::Bool(
            Point::new(
                tx_id,
                &self.id,
                Bool(enable),
                status,
                cot,
                timestamp,
            )
        )
    }
    //
    fn reset(&mut self) {
        if let Some(enable) = &self.enable {
            enable.borrow_mut().reset();
        }
        for input in &self.inputs {
            input.borrow_mut().reset();
        }

    }
}
//
// 
impl FnInOut for FnRecOpCycleMetric {}
///
/// Global static counter of FnRecOpCycleMetric instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

use std::sync::atomic::{AtomicUsize, Ordering};
use log::warn;

use crate::{
    conf::point_config::point_config_type::PointConfigType, core_::{
        point::{point::Point, point_type::PointType}, 
        types::fn_in_out_ref::FnInOutRef,
    }, services::task::nested_function::{
        fn_::{FnIn, FnInOut, FnOut},
        fn_kind::FnKind,
    }
};
///
/// Function | Returns filtered input:
/// - if factor is not specified:
///     - new input value returned if |prev - [input]| > [threshold]
/// - if factor is specified:
///     - each cycle: delta = |prev - [input]| * factor
///     - new input value returned if delta >= [threshold]
#[derive(Debug)]
pub struct FnThreshold {
    id: String,
    kind: FnKind,
    threshold: FnInOutRef,
    factor: Option<FnInOutRef>,
    input: FnInOutRef,
    value: Option<PointType>,
    delta: Point<f64>,
}
//
// 
impl FnThreshold {
    ///
    /// Creates new instance of the FnThreshold
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, threshold: FnInOutRef, factor: Option<FnInOutRef>, input: FnInOutRef) -> Self {
        Self { 
            id: format!("{}/FnThreshold{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind: FnKind::Fn,
            threshold,
            factor,
            input,
            value: None,
            delta: Point::new_double(0, "", 0.0),
        }
    }    
}
//
// 
impl FnIn for FnThreshold {}
//
// 
impl FnOut for FnThreshold { 
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
        self.input.borrow().inputs()
    }
    //
    //
    fn out(&mut self) -> PointType {
        let input = self.input.borrow_mut().out();
        let input_type = input.type_();
        let input = input.to_double().as_double();
        match &self.value {
            Some(value) => {
                let threshold = self.threshold.borrow_mut().out().to_double().as_double();
                match &self.factor {
                    Some(factor) => {
                        let factor = factor.borrow_mut().out().to_double().as_double();
                        self.delta = self.delta.clone() + ((input.clone() - value.to_double().as_double()) * factor).abs();
                        warn!("{}.out | Integral delta: {}", self.id, self.delta.value);
                        if self.delta >= threshold {
                            self.value = Some(PointType::Double(input));
                        }
                    }
                    None => {
                        let delta = (input.clone() - value.to_double().as_double()).abs();
                        warn!("{}.out | Absolute delta: {}", self.id, delta.value);
                        if delta >= threshold {
                            self.value = Some(PointType::Double(input));
                        }
                    }
                }
            }
            None => {
                self.value = Some(PointType::Double(input));
            }
        }
        let value = match &self.value {
            Some(value) => match input_type {
                PointConfigType::Int => value.to_int(),
                PointConfigType::Real => value.to_real(),
                PointConfigType::Double => value.to_double(),
                _ => panic!("{}.out | Illegal type of input {:?}", self.id, input_type),
            }
            None => panic!("{}.out | Internal error - self.value is not initialised", self.id),
        };
        warn!("{}.out | value: {:?}", self.id, value);
        value
    }
    //
    //
    fn reset(&mut self) {
        self.threshold.borrow_mut().reset();
        if let Some(factor) = &self.factor {
            factor.borrow_mut().reset();
        }
        self.input.borrow_mut().reset();
        self.value = None;
    }
}
//
// 
impl FnInOut for FnThreshold {}
///
/// Global static counter of FnThreshold instances
static COUNT: AtomicUsize = AtomicUsize::new(1);

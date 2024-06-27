use std::sync::atomic::{AtomicUsize, Ordering};
use chrono::Utc;
use log::trace;
use crate::{
    core_::{
        cot::cot::Cot, point::{point::Point, point_tx_id::PointTxId, point_type::PointType},
        status::status::Status, types::{bool::Bool, fn_in_out_ref::FnInOutRef},
    },
    services::task::nested_function::{
        fn_::{FnInOut, FnIn, FnOut}, fn_kind::FnKind,
    },

};
///
/// Function | Returns bitwise OR of all inputs
/// 
/// Example
/// 
/// ```yaml
/// fn BitOr:
///     input1: point int '/App/Service/Point.Name1'
///     input2: point int '/App/Service/Point.Name2'
/// fn BitOr:
///     input1: point bool '/App/Service/Point.Name1'
///     input2: point bool '/App/Service/Point.Name2'
/// ```
#[derive(Debug)]
pub struct FnBitOr {
    id: String,
    kind: FnKind,
    inputs: Vec<FnInOutRef>,
}
//
// 
impl FnBitOr {
    ///
    /// Creates new instance of the FnBitOr
    #[allow(dead_code)]
    pub fn new(parent: impl Into<String>, inputs: Vec<FnInOutRef>) -> Self {
        Self { 
            id: format!("{}/FnBitOr{}", parent.into(), COUNT.fetch_add(1, Ordering::Relaxed)),
            kind:FnKind::Fn,
            inputs,
        }
    }
}
//
// 
impl FnIn for FnBitOr {}
//
// 
impl FnOut for FnBitOr {
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
        for input in &self.inputs {
            inputs.append(&mut input.borrow().inputs());
        }
        inputs
    }
    //
    fn out(&mut self) -> PointType {
        let tx_id = PointTxId::from_str(&self.id);
        let mut inputs = self.inputs.iter();
        let mut value: PointType;
        match inputs.next() {
            Some(first) => {
                value = first.borrow_mut().out();
                while let Some(input) = inputs.next() {
                    let input = input.borrow_mut().out();
                    trace!("{}.out | input '{}': {:?}", self.id, input.name(), input.value());
                    value = match &value {
                        PointType::Bool(val) => {
                            let input_val = input.try_as_bool().unwrap_or_else(|_| panic!("{}.out | Incopatable types, expected '{:?}', but input '{}' has type '{:?}'", self.id, value.type_(), input.name(), input.type_()));
                            PointType::Bool(
                                Point::new(
                                    tx_id,
                                    &format!("{}.out", self.id),
                                    Bool(val.value.0 | input_val.value.0),
                                    Status::Ok,
                                    Cot::Inf,
                                    Utc::now(),
                                )
                            )
                        }
                        PointType::Int(val) => {
                            let input_val = input.try_as_int().unwrap_or_else(|_| panic!("{}.out | Incopatable types, expected '{:?}', but input '{}' has type '{:?}'", self.id, value.type_(), input.name(), input.type_()));
                            PointType::Int(
                                Point::new(
                                    tx_id,
                                    &format!("{}.out", self.id),
                                    val.value | input_val.value,
                                    Status::Ok,
                                    Cot::Inf,
                                    Utc::now(),
                                )
                            )
                        }
                        PointType::Real(_) => {
                            panic!("{}.out | Not implemented for Real", self.id);
                        }
                        PointType::Double(_) => {
                            panic!("{}.out | Not implemented for Double", self.id);
                        }
                        PointType::String(_) => {
                            panic!("{}.out | Not implemented for String", self.id);
                        }
                    };
                }
            },
            None => panic!("{}.out | At least one input must be specified", self.id),
        };
        // trace!("{}.out | value: {:#?}", self.id, value);
        value
    }
    //
    fn reset(&mut self) {
        for input in &self.inputs {
            input.borrow_mut().reset();
        }
    }
}
//
// 
impl FnInOut for FnBitOr {}
///
/// Global static counter of FnBitOr instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);

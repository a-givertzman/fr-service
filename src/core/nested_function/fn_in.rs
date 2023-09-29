#![allow(non_snake_case)]

use log::trace;
use std::fmt::Debug;

use super::{fn_::{FnInput, FnOutput}, fn_reset::FnReset};

#[derive(Clone, Debug)]
pub struct FnIn<TIn> {
    // input: Box<dyn FnOutput<bool>>,
    value: TIn,
}
impl<TIn: Clone> FnIn<TIn> {
    #[allow(dead_code)]
    pub fn new(initial: TIn) -> Self {
        Self { value: initial.clone() }
    }
}
impl<TIn: Debug + Clone + Default> FnInput<TIn> for FnIn<TIn> {
    ///
    fn add(&mut self, value: TIn) {
        self.value = value;
        trace!("FnIn.add | value: {:?}", self.value);
    }
}

impl<TIn: Clone + Debug + Default> FnOutput<TIn> for FnIn<TIn> {
    ///
    fn out(&mut self) -> TIn {
        trace!("FnIn.out | value: {:?}", self.value);
        let value = self.value.clone();
        value
    }
}

impl<TIn: Clone + Debug + Default> FnReset for FnIn<TIn> {
    fn reset(&mut self) {
        self.value = Default::default();
        trace!("FnIn.reset | value: {:?}", self.value);
    }
}

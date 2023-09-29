#![allow(non_snake_case)]

use log::debug;
use std::fmt::Debug;

use super::fn_::{FnInput, FnOutput};

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
impl<TIn: Debug> FnInput<TIn> for FnIn<TIn> {
    ///
    fn add(&mut self, value: TIn) {
        self.value = value;
        debug!("FnIn.add | value: {:?}", self.value);
    }
}

impl<TIn: Clone + Debug> FnOutput<TIn> for FnIn<TIn> {
    ///
    fn out(&mut self) -> TIn {
        debug!("FnIn.out | value: {:?}", self.value);
        let value = self.value.clone();
        value
    }
}

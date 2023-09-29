#![allow(non_snake_case)]

use super::fn_reset::FnReset;


pub trait FnInput<TIn>: FnReset {
    fn add(&mut self, value: TIn);
}

// #[derive(Sized)]
pub trait FnOutput<TOut>: FnReset {
    fn out(&mut self) -> TOut;
    // fn print(&self);
}

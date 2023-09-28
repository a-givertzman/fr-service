#![allow(non_snake_case)]


pub trait FnInput<TIn> {
    fn add(&mut self, value: TIn);
}

// #[derive(Sized)]
pub trait FnOutput<TOut> {
    fn out(&mut self) -> TOut;
    // fn print(&self);
}

use std::{fmt::Debug, cell::{Cell, RefCell}, rc::Rc};

use log::debug;


// pub enum FnType<T> {
//     Count(FnCount<T>)
// }
pub trait FnInput<TIn> {
    fn add(&mut self, value: TIn);
}

// #[derive(Sized)]
pub trait FnOutput<TOut> {
    fn out(&mut self) -> TOut;
    // fn print(&self);
}





///
/// Counts number of raised fronts of boolean input
pub struct FnCount<TIn> {
    input: Rc<RefCell<dyn FnOutput<bool>>>,
    inputValue: bool,
    count: TIn,
}

impl<TIn> FnCount<TIn> {
    pub fn new(initial: TIn, input: Rc<RefCell<dyn FnOutput<bool>>>) -> Self {
        Self { 
            input,
            inputValue: false,
            count: initial ,
        }
    }
}

impl FnOutput<u128> for FnCount<u128> {
    ///
    fn out(&mut self) -> u128 {
        // debug!("FnCount.out | input: {:?}", self.input.print());
        let value = self.input.borrow_mut().out();
        debug!("FnCount.out | input.out: {:?}", &value);
        if (!self.inputValue) && value {
            self.count += 1;
        }
        self.inputValue = value;
        self.count
    }
}


#[derive(Clone, Debug)]
pub struct FnIn<TIn> {
    // input: Box<dyn FnOutput<bool>>,
    value: TIn,
}
impl<TIn: Clone> FnIn<TIn> {
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


pub trait NestedFunction<TIn, TOut> {
    fn add(&self, value: TIn) {

    }
    fn state() -> TOut {
        
    }
}

pub struct FnCount<TIn, TOut> {
    value: TIn,
}

impl<TIn, TOut> NestedFunction<TIn, TOut> for FnCount<TIn, TOut> {
    ///
    fn add(&self, value: TIn) {
        self.value = value
    }
    ///
    fn state() -> TOut {
        self.v       
    }
}
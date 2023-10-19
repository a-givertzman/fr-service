mod core_;
mod services;

use std::marker::PhantomData;
trait SpecializationTest<T> {
    type Target;
    // fn special(&self) -> &T;
    // fn target(&self) -> &Self::Target;
}

trait Out<O> {
    fn out(&self) ->O ;
}

#[derive(Debug)]
struct Ikea<S, T, Q> 
    where S: SpecializationTest<T, Target=Q>
{
    pub value: Q,
    pub inner: Vec<S>, 
    _marker: (PhantomData<T>, PhantomData<Q>)
}

impl<S> Out<S::Target> for Ikea<S, i64, f64>
    where S : SpecializationTest<i64, Target=f64>
{
    fn out(&self) ->f64 {
        self.value
    }
}

impl<S> Out<S::Target> for Ikea<S, f64, i64>
    where S: SpecializationTest<f64, Target=i64>
{
    fn out(&self) ->i64 {
        self.value
    }
}
impl<S> Out<S::Target> for Ikea<S, f64, f64>
    where S: SpecializationTest<f64, Target=f64>
{
    fn out(&self) ->f64 {
        self.value
    }
}

impl<S> Out<S::Target> for Ikea<S, f64, bool>
    where S: SpecializationTest<f64, Target=bool>
{
    fn out(&self) ->bool {
        self.value
    }
}

fn main() {
    let ikea = Ikea{ 
        value: true,
        inner: vec![
            Inner{input: 4.4, out: true, _marker: PhantomData::<f64>}
        ], 
        _marker: (PhantomData::<f64>, PhantomData::<bool>), 
    };
    let out = ikea.out();
    let r = &ikea.inner[0];
    // let ii = *r.special();
    println!("special: {:?}", r);
    // println!("special: {:?}", r.special());
    // println!("special: {:?}", r.target());
}


// #[derive(Debug)]
struct Inner<T, Q> 
    where T: SpecializationTest<T, Target=Q>
{
    pub input: T,
    pub out: Q,
    _marker: PhantomData<Q>
}
impl<T, Q> SpecializationTest<Q> for Inner<T, Q> {
    type Target = Q;
}
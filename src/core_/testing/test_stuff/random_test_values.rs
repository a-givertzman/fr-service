#![allow(non_snake_case)]

use rand::{rngs::ThreadRng, Rng};

use super::test_value::Value;

///
/// 
#[derive(Debug, Clone)]
pub struct RandomTestValues {
    id: String,
    initial: Vec<Value>,
    iterations: usize,
    rnd: ThreadRng,
}
///
/// 
impl RandomTestValues {
    ///
    /// 
    pub fn  new(parent: impl Into<String>, initial: Vec<Value>, iterations: usize) -> Self {
        Self {
            id: format!("{}/RandomTestPoints", parent.into()),
            initial,
            iterations,
            rnd: rand::thread_rng(),
        }
    }
    }
///
/// 
impl Iterator for RandomTestValues {
    type Item = Value;
    //
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterations > 0 {
            self.iterations -= 1;
            let index = self.rnd.gen_range(0..self.initial.len());
            match self.initial.get(index) {
                Some(value) => {
                    return Some(value.clone());
                },
                None => {
                    panic!("{}.next | Out of range: index {}, not in initial length 0..{}", self.id, index, self.initial.len())
                },
            };
        }
        None
    }
}
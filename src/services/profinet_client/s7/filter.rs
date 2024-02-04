#![allow(non_snake_case)]

///
/// 
pub trait Filter: std::fmt::Debug {
    type Item;
    fn value(&self) -> Self::Item;
    fn add(&mut self, value: Self::Item);
    fn next(&mut self, value: Self::Item) -> Option<Self::Item>;
    fn isChanged(&self) -> bool;
}
///
/// Pass input value as is
#[derive(Debug, Clone)]
pub struct FilterEmpty<T> {
    value: T,
    isChanged: bool,
}
///
/// 
impl<T> FilterEmpty<T> {
    pub fn new(initial: T) -> Self {
        Self { value: initial, isChanged: true }
    }
}
///
/// 
impl<T: Copy + std::fmt::Debug + std::cmp::PartialEq> Filter for FilterEmpty<T> {
    type Item = T;
    //
    //
    fn value(&self) -> Self::Item {
        self.value
    }
    //
    //
    fn add(&mut self, value: Self::Item) {
        self.value = value;
    }
    //
    //
    fn next(&mut self, value: T) -> Option<T> {
        if value != self.value {
            self.value = value;
            Some(self.value)
        } else {
            None
        }
    }
    //
    //
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}
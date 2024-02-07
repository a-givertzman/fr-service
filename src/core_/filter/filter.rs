#![allow(non_snake_case)]

///
/// 
pub trait Filter: std::fmt::Debug {
    type Item;
    ///
    /// Returns current state
    fn value(&self) -> Self::Item;
    /// - Updates state with value if value != inner
    fn add(&mut self, value: Self::Item);
    ///
    /// Returns true if last [add] was successful
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
        if value != self.value {
            self.isChanged = true;
            self.value = value;
        } else {
            self.isChanged = false;
        }
    }
    //
    //
    fn isChanged(&self) -> bool {
        self.isChanged
    }
}
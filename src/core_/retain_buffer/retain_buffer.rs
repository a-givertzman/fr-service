#![allow(non_snake_case)]

use std::collections::VecDeque;


///
/// Holds array of T
/// - push(value: T) - appends an element to the end of a buffer
///     - if capacity exeeded ferst element will be deleted
/// - pop(value: T) - returns and removes value<T> from the end of array
/// - remove(index) - returns and removes value<T> from the [index] position
/// - len() - Returns the number of elements in the buffer
pub struct RetainBuffer<T> {
    vec: VecDeque<T>,
    capacity: Option<usize>,
}
///
/// 
impl<T> RetainBuffer<T> {
    ///
    /// Creates new instance of the ReatinBuffer
    pub const fn new(capacity: Option<usize>) -> Self {
        Self { 
            vec: VecDeque::new(),
            capacity,
        }
    }
    ///
    /// Appends an element to the back of a buffer,
    /// - if capacity exeeded ferst element will be lost.
    pub fn push(&mut self, value: T) {
        match self.capacity {
            Some(capacity) => {
                if self.vec.len() >= capacity {
                    self.vec.pop_front();
                }
            },
            None => {},
        }
        self.vec.push_back(value);
    }
    ///
    /// Returns and removes first value<T> in the buffer
    pub fn popFirst(&mut self) -> Option<T> {
        self.vec.pop_front()
    }
    ///
    /// Returns and removes value<T> from the [index] position
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.vec.pop_front()
    }
    ///
    /// Returns the number of elements in the buffer
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    ///
    /// Immediately stores the content of the buffer
    pub fn store(&self) {
        todo!()
    }
}
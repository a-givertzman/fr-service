#![allow(non_snake_case)]


///
/// Holds array of T
/// - push(value: T) - appends an element to the end of a collection
///     - if max length exeeded ferst element will be deleted
/// - pop(value: T) - returns and removes value<T> from the end of array
/// - remove(index) - returns and removes value<T> from the [index] position
/// - len() - returns count of items in the buffer
pub struct RetainBuffer<T> {
    vec: Vec<T>,
}
///
/// 
impl<T> RetainBuffer<T> {
    ///
    /// Creates new instance of the ReatinBuffer
    pub const fn new(max: usize) -> Self {
        Self { vec: Vec::new() }
    }
    ///
    /// Appends an element to the back of a collection.
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }
    ///
    /// 
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}
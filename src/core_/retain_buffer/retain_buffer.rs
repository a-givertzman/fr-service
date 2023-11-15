#![allow(non_snake_case)]


///
/// Holds array of T
/// - push(value: T) - appends an element to the end of a buffer
///     - if capacity exeeded ferst element will be deleted
/// - pop(value: T) - returns and removes value<T> from the end of array
/// - remove(index) - returns and removes value<T> from the [index] position
/// - len() - Returns the number of elements in the buffer
pub struct RetainBuffer<T> {
    vec: Vec<T>,
    capacity: Option<usize>,
}
///
/// 
impl<T> RetainBuffer<T> {
    ///
    /// Creates new instance of the ReatinBuffer
    pub const fn new(capacity: Option<usize>) -> Self {
        Self { 
            vec: Vec::new(),
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
                    self.vec.remove(0);
                }
            },
            None => {},
        }
        self.vec.push(value);
    }
    ///
    /// Returns and removes value<T> from the [index] position
    pub fn remove(&mut self, index: usize) -> T {
        self.vec.remove(index)
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
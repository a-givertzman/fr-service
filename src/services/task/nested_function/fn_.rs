#![allow(non_snake_case)]

use crate::core_::point::point_type::PointType;


///
/// Input side interface for nested function
/// Used for generic access to the different kinde of functions
/// for adding new value on input side
pub trait FnIn: std::fmt::Debug {
    fn add(&mut self, point: PointType);
}
///
/// Out side interface for the function
/// Used for generic access to the different kinde of functions
/// - to get the calculated value on out side
/// - to reset the state to the initial
pub trait FnOut: std::fmt::Debug {
    ///
    /// returns calculated value
    fn out(&mut self) -> PointType;
    ///
    /// resets self state to the initial, calls reset method of all inputs 
    fn reset(&mut self);
}
///
/// Interface for nested function
/// Used for generic access to the different kinde of functions in the nested tree
pub trait FnInOut: FnIn + FnOut {
    
}
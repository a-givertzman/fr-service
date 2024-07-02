use crate::core_::point::point_type::PointType;
use super::{fn_kind::FnKind, fn_result::FnResult};
///
/// Input side interface for nested function
/// Used for generic access to the different kinde of functions
/// for adding new value on input side
pub trait FnIn: std::fmt::Debug {
    fn add(&mut self, _point: PointType) {
        panic!("FnIn.add | don't use this method, used only for FnInput")
    }
}
///
/// Out side interface for the function
/// Used for generic access to the different kinde of functions
/// - to get the calculated value on out side
/// - to reset the state to the initial
pub trait FnOut: std::fmt::Debug {
    ///
    /// Retirns it unique idetificator
    fn id(&self) -> String;
    ///
    /// Returns enum kind of the FnOut
    fn kind(&self) -> &FnKind;
    ///
    /// Returns names of inputs it depending on
    fn inputs(&self) -> Vec<String>;
    ///
    /// Evaluate calculations
    /// - Used only for FnVar
    fn eval(&mut self) {
        panic!("FnOut.eval | don't use this method, used only for FnVar")
    }
    ///
    /// - Evaluate calculations
    /// - Returns calculated value
    /// - Returns error if:
    ///   - Calculations fails
    ///   - Input not initialized
    /// - Returns None if:
    ///   - Point filtered by any kind of filtering function
    fn out(&mut self) -> FnResult<PointType, String>;
    ///
    /// resets self state to the initial, calls reset method of all inputs 
    fn reset(&mut self);
}
///
/// Interface for nested function
/// Used for generic access to the different kinde of functions in the nested tree
pub trait FnInOut: FnIn + FnOut {}
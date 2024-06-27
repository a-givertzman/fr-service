use crate::core_::point::point_type::PointType;
use super::fn_kind::FnKind;
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
    /// used only for FnVar
    /// evaluate calculations
    fn eval(&mut self) {
        panic!("FnOut.eval | don't use this method, used only for FnVar")
    }
    ///
    /// - evaluate calculations
    /// - returns calculated value
    fn out(&mut self) -> PointType;
    ///
    /// resets self state to the initial, calls reset method of all inputs 
    fn reset(&mut self);
}
///
/// Interface for nested function
/// Used for generic access to the different kinde of functions in the nested tree
pub trait FnInOut: FnIn + FnOut {}
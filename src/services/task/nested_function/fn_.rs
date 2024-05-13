use crate::core_::point::point_type::PointType;
use super::fn_kind::FnKind;
///
/// Result returning from any function out
#[derive(Debug, Clone, PartialEq)]
pub enum FnResult {
    Ok(PointType),
    Err(String),
    None,
}
///
/// 
impl FnResult {
    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Err`]
    /// case explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: Result::unwrap_or
    /// [`unwrap_or_else`]: Result::unwrap_or_else
    /// [`unwrap_or_default`]: Result::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message provided by the
    /// [`Err`]'s value.
    pub fn unwrap(self) -> PointType {
        match self {
            Self::Ok(point) => point,
            Self::Err(err) => panic!("called `FnResult::unwrap()` on an `Err` value: {}", err),
            Self::None => panic!("called `FnResult::unwrap()` on a `None` value"),
        }
    }
}
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
    fn out(&mut self) -> FnResult;
    ///
    /// resets self state to the initial, calls reset method of all inputs 
    fn reset(&mut self);
}
///
/// Interface for nested function
/// Used for generic access to the different kinde of functions in the nested tree
pub trait FnInOut: FnIn + FnOut {}
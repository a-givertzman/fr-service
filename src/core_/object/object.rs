use crate::conf::point_config::name::Name;

///
/// Interface for application object
pub trait Object {
    ///
    /// Returns Object's debug id
    fn id(&self) -> &str;
    ///
    /// Returns Object's name
    fn name(&self) -> Name;
}
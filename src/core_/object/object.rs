///
/// Interface for application object
pub trait Object {
    ///
    /// Returns Object's ID
    fn id(&self) -> &str;
}
///
/// The type of the Function config
#[derive(Debug, PartialEq)]
pub enum FnConfigType {
    Fn,
    Var,
    Const,
    Point,
    Unknown,
}

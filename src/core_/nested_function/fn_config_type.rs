///
/// The type of the Function config
#[derive(Debug, Clone, PartialEq)]
pub enum FnConfigType {
    Fn,
    Var,
    Const,
    Point,
    Metric,
    Unknown,
}

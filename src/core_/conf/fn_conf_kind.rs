///
/// The type of the Function config
#[derive(Debug, Clone, PartialEq)]
pub enum FnConfKind {
    Fn,
    Var,
    Const,
    Point,
    Metric,
    Param,
    // Unknown,
}

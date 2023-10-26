use crate::core_::types::fn_in_out_ref::FnInOutRef;

#[derive(Debug, Clone)]
pub enum TaskNodeType {
    Var(FnInOutRef),
    Metric(FnInOutRef),
}

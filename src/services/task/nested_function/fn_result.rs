use crate::core_::point::point_type::PointType;

///
/// Result returning from the Task FnOut
pub enum FnResult<T,E> {
    Ok(T),
    None,
    Err(E),
}
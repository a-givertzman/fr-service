///
/// Result returning from the Task FnOut
#[derive(Clone, Debug)]
pub enum FnResult<T, E> {
    Ok(T),
    None,
    Err(E),
}
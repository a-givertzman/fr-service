///
/// Result returning from the Task FnOut
#[derive(Clone, Debug)]
pub enum FnResult<T, E> {
    Ok(T),
    None,
    Err(E),
}
//
//
impl<T, E: std::fmt::Debug> FnResult<T, E> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(v) => v,
            Self::None => panic!("FnResult.unwrap | Called on a None value"),
            Self::Err(err) => panic!("FnResult.unwrap | Called on a error: {:#?}", err),
        }
    }
}
use crate::core_::status::status::Status;
///
/// Optional parameters of the [FnConf]
#[derive(Debug, PartialEq, Clone)]
pub struct FnConfOptions {
    pub default: Option<String>,
    pub status: Option<Status>,
}
//
//
impl Default for FnConfOptions {
    fn default() -> Self {
        Self { default: None, status: None }
    }
}
///
/// Represents path to the ssh.pub keys folder
#[derive(Debug, Clone, PartialEq)]
pub struct AuthSshPath {
    path: String,
}
///
/// 
impl AuthSshPath {
    ///
    /// Creates new instance of AuthSshPath
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
    ///
    /// 
    pub fn path(&self) -> String {
        self.path.clone()
    }
}
///
/// Kinde of auturization on the TcpServer
#[derive(Debug, Clone, PartialEq)]
pub enum TcpServerAuth {
    None,
    Secret,
    Ssh,
}
///
/// 
impl TcpServerAuth {
    fn fromStr(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "none" => TcpServerAuth::None,
            "secret" => TcpServerAuth::Secret,
            "ssh" => TcpServerAuth::Ssh,
            _ => panic!()
        }
    }
}
///
/// 
impl From<&str> for TcpServerAuth {
    fn from(value: &str) -> Self {
        Self::fromStr(value)
    }
}
///
/// 
impl From<&String> for TcpServerAuth {
    fn from(value: &String) -> Self {
        Self::fromStr(value)
    }
}
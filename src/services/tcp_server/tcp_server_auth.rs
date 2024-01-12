use crate::{core_::auth::{secret::auth_secret::AuthSecret, ssh::auth_ssh_path::AuthSshPath}, conf::conf_tree::ConfTree};

///
/// Kinde of auturization on the TcpServer
#[derive(Debug, Clone, PartialEq)]
pub enum TcpServerAuth {
    None,
    Secret(AuthSecret),
    Ssh(AuthSshPath),
}
///
/// 
impl TcpServerAuth {
    ///
    /// 
    pub fn new(value: ConfTree) -> Self {
        match value.key.to_lowercase().as_str() {
            "none" => TcpServerAuth::None,
            "secret" => {
                let token = value.conf.as_str().unwrap();
                TcpServerAuth::Secret(AuthSecret::new(token))
            },
            "ssh" => {
                let path = value.asStr("path").unwrap();
                TcpServerAuth::Ssh(AuthSshPath::new(path))
            },
            _ => panic!()
        }
    }
    // ///
    // /// 
    // fn fromStr(value: &str) -> Self {
    //     match value.to_lowercase().as_str() {
    //         "none" => TcpServerAuth::None,
    //         "secret" => TcpServerAuth::Secret(AuthSecret::new("")),
    //         "ssh" => TcpServerAuth::Ssh(AuthSshPath::new("path")),
    //         _ => panic!()
    //     }
    // }
}
// ///
// /// 
// impl From<&str> for TcpServerAuth {
//     fn from(value: &str) -> Self {
//         Self::fromStr(value)
//     }
// }
// ///
// /// 
// impl From<&String> for TcpServerAuth {
//     fn from(value: &String) -> Self {
//         Self::fromStr(value)
//     }
// }
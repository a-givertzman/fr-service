use log::info;

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
        info!("TcpServerAuth.new | value: {:?}", value);
        if value.conf.is_string() {

        } else if value.conf.is_mapping() {

        } else {
            panic!();
        }
        match value.key.to_lowercase().as_str() {
            "auth" => {
                match value.conf.as_str() {
                    Some("none" | "None") => TcpServerAuth::None,
                    _ => panic!("TcpServerAuth.new | Unknown value in 'auth', 'none' or 'None' expected"),
                }
            },
            "auth-secret" => {
                let token = match value.asStr("pass") {
                    Ok(token) => token,
                    Err(_) => panic!("TcpServerAuth.new | 'pass' - not found in 'auth-secret'"),
                };
                TcpServerAuth::Secret(AuthSecret::new(token))
            },
            "auth-ssh" => {
                let path = match value.asStr("pass") {
                    Ok(path) => path,
                    Err(_) => panic!("TcpServerAuth.new | 'path' - not found in 'auth-ssh'"),
                };
                TcpServerAuth::Ssh(AuthSshPath::new(path))
            },
            _ => panic!(),
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
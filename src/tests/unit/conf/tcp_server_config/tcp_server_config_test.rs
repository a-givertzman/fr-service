#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::info;
    use std::sync::Once;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::conf::tcp_server_config::TcpServerConfig; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn initOnce() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach() -> () {
    
    }
    
    #[test]
    fn test_TcpServer_config() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test TcpServerConfig";
        println!("{}", selfId);
        let testData = [
            format!(r#"
                service TcpServer:
                    cycle: 1 ms
                    reconnect: 1 s  # default 3 s
                    address: 127.0.0.1:8080
                    auth: none      # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                    in queue link:
                        max-length: 10000
                    out queue: MultiQueue.in-queue
            "#),
            format!(r#"
                service TcpServer:
                    cycle: 1 ms
                    reconnect: 1 s  # default 3 s
                    address: 127.0.0.1:8080
                    auth-secret:    # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                        pass: secret-password
                    in queue link:
                        max-length: 10000
                    out queue: MultiQueue.in-queue
            "#),
            format!(r#"
                service TcpServer:
                    cycle: 1 ms
                    reconnect: 1 s  # default 3 s
                    address: 127.0.0.1:8080
                    auth-ssh:    # auth: none / auth-secret: pass: ... / auth-ssh: path: ...
                        path: /home/scada/.ssh/
                    in queue link:
                        max-length: 10000
                    out queue: MultiQueue.in-queue
            "#),
        ];
        for conf in testData {
            let conf = serde_yaml::from_str(&conf).unwrap();
            let conf = TcpServerConfig::fromYamlValue(&conf);
            info!("conf: \n{:?}", conf);
            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
}

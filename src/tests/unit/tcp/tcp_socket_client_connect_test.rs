#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use std::{sync::{Once, atomic::{AtomicBool, Ordering}, Arc}, time::Duration, thread, net::TcpListener};
    use crate::{core_::debug::debug_session::{DebugSession, LogLevel, Backtrace}, tcp::tcp_socket_client_connect::TcpSocketClientConnect}; 
    
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
    fn test_success() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("success connection");
        let addr = "127.0.0.1:9995";
        let timeout = Duration::from_millis(3500); // ms
        let mut connect = TcpSocketClientConnect::new("test", addr);

        let ok = Arc::new(AtomicBool::new(false));
        let okRef = ok.clone();

        let connectExit = connect.exit();
        thread::spawn(move || {
            info!("Preparing test TCP server...");
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("Preparing test TCP server - ok");
                    match listener.accept() {
                        Ok((_socket, addr)) => {
                            info!("incoming connection - ok\n\t{:?}", addr);
                        },
                        Err(err) => {
                            info!("incoming connection - error: {:?}", err);
                        },
                    }
                },
                Err(err) => {
                    connectExit.send(true).unwrap();
                    okRef.store(false, Ordering::SeqCst);
                    panic!("Preparing test TCP server - error: {:?}", err);
                },
            };
        });

        let connectExit = connect.exit();
        let okRef = ok.clone();
        thread::spawn(move || {
            info!("Waiting for connection...");
            thread::sleep(timeout);
            okRef.store(false, Ordering::SeqCst);
            warn!("Tcp socket was not connected in {:?}", timeout);
            debug!("stopping...");
            connectExit.send(true).unwrap();
        });
        info!("Connecting...");
        let tcpStream = connect.connect(Duration::from_millis(1000));
        if tcpStream.is_some() {
            ok.store(true, Ordering::SeqCst);
            info!("connected: {:?}", tcpStream);
        }
        assert!(ok.load(Ordering::SeqCst) == true, "\nresult: {:?}\ntarget: {:?}", ok, true);
    }

    #[test]
    fn test_failure() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("failure connection");
        let timeout = Duration::from_millis(1500); // ms
        let mut connect = TcpSocketClientConnect::new("test", "127.0.0.1:9996");
        let connectExit = connect.exit();
        let ok = Arc::new(AtomicBool::new(false));
        let okRef = ok.clone();
        thread::spawn(move || {
            info!("Waiting for connection...");
            thread::sleep(timeout);
            okRef.store(false, Ordering::SeqCst);
            warn!("Tcp socket was not connected in {:?}", timeout);
            debug!("stopping...");
            connectExit.send(true).unwrap();
        });
        info!("Connecting...");
        let tcpStream = connect.connect(Duration::from_millis(1000));
        if tcpStream.is_some() {
            ok.store(true, Ordering::SeqCst);
            info!("connected: {:?}", tcpStream);
        }
        assert!(ok.load(Ordering::SeqCst) == false, "\nresult: {:?}\ntarget: {:?}", ok, false);
    }

}

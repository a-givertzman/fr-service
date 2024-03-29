#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use std::{sync::{Once, atomic::{AtomicBool, Ordering}, Arc}, time::Duration, thread, net::TcpListener};
    use testing::session::test_session::TestSession;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::tcp::tcp_client_connect::TcpClientConnect; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {
    
    }
    
    #[test]
    fn test_success_connection() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        println!("test success connection");
        let addr = "127.0.0.1:".to_owned() + &TestSession::free_tcp_port_str();
        let timeout = Duration::from_millis(3500); // ms
        let mut connect = TcpClientConnect::new("test", &addr, Duration::from_millis(500));

        let ok = Arc::new(AtomicBool::new(false));
        let okRef = ok.clone();

        // let connectExit = connect.exit();
        thread::spawn(move || {
            info!("Preparing test TCP server...");
            thread::sleep(Duration::from_millis(300));
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
                    // connectExit.send(true);
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
        for _ in 0..10 {
            match connect.connect() {
                Some(tcpStream) => {
                    ok.store(true, Ordering::SeqCst);
                    info!("connected: {:?}", tcpStream);
                    connect.exit().send(true).unwrap();
                    break;
                },
                None => {
                    warn!("not connected");
                },
            };
            thread::sleep(Duration::from_millis(100));
        }
        assert!(ok.load(Ordering::SeqCst) == true, "\nresult: connected - {:?}\ntarget: connected - {:?}", ok, true);
    }

    #[test]
    fn test_failure_connection() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        println!("test failure connection");
        let timeout = Duration::from_millis(1500); // ms
        let addr = "127.0.0.1:".to_owned() + &TestSession::free_tcp_port_str();
        let mut connect = TcpClientConnect::new("test", &addr, Duration::from_millis(500));
        let connectExit = connect.exit();
        let ok = Arc::new(AtomicBool::new(false));
        let okRef = ok.clone();
        thread::spawn(move || {
            info!("Waiting for connection...");
            thread::sleep(timeout);
            okRef.store(false, Ordering::SeqCst);
            warn!("Tcp socket was not connected in {:?}", timeout);
            debug!("Thread | stopping...");
            connectExit.send(true).unwrap();
            debug!("Thread | stopping - ok");
        });
        info!("Connecting...");
        match connect.connect() {
            Some(tcpStream) => {
                ok.store(true, Ordering::SeqCst);
                info!("connected: {:?}", tcpStream);
            },
            None => {
                warn!("not connected");
            },
        };
        assert!(ok.load(Ordering::SeqCst) == false, "\nresult: connected - {:?}\ntarget: connected - {:?}", ok, false);
    }

}

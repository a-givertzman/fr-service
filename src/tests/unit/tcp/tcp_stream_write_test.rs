#![allow(non_snake_case)]

use crate::tcp::steam_read::StreamRead;
#[cfg(test)]
mod tests {
    use log::{warn, info, debug, error};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, thread, net::{TcpListener, TcpStream}, io::Read};
    use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, net::connection_status::ConnectionStatus, testing::test_session::TestSession}, tcp::tcp_stream_write::TcpStreamWrite, tests::unit::tcp::tcp_stream_write_test::MockStreamRead}; 
    
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
    fn test_() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_");
        let mut sent = 0;
        let received = Arc::new(Mutex::new(vec![]));
        let testData = vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
        ];
        let mut tcpStreamWrite = TcpStreamWrite::new(
            "test",
            true,
            Some(10000),
            Box::new(MockStreamRead { buffer: testData.clone()}),
        );
        let count = testData.len();
        let addr = "127.0.0.1:".to_owned() + &TestSession::freeTcpPortStr();

        mockTcpServer(addr.clone(), count, testData, received.clone());

        let mut stream = TcpStream::connect(addr).unwrap();
        while sent < count {
            match tcpStreamWrite.write(&mut stream) {
                Ok(_) => {
                    sent += 1
                },
                Err(err) => {
                    panic!("{}", err);
                },
            };
            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        assert!(sent == received.lock().unwrap().len(), "\nresult: {:?}\ntarget: {:?}", sent, received.lock().unwrap().len());
    }
    ///
    /// TcpServer setup
    fn mockTcpServer(addr: String, count: usize, testData: Vec<Vec<u8>>, received: Arc<Mutex<Vec<Vec<u8>>>>) {
        let mut sent = 0;
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            let mut rng = rand::thread_rng();
            match TcpListener::bind(addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok");
                    let mut acceptCount = 2;
                    // let mut maxReadErrors = 3;
                    while acceptCount > 0 {
                        acceptCount -= 1;
                        match listener.accept() {
                            Ok((mut _socket, addr)) => {
                                info!("TCP server | accept connection - ok\n\t{:?}", addr);
                                let mut bytes = vec![];
                                for _ in 0..count {
                                    match _socket.read(&mut bytes) {
                                        Ok(_) => {
                                            received.lock().unwrap().push(bytes.clone());
                                            bytes.clear();
                                        },
                                        Err(err) => {
                                            warn!("{:?}", err);
                                        },
                                    }
                                }
                                info!("TCP server | all sent: {:?}", sent);
                                while received.lock().unwrap().len() < count {
                                    thread::sleep(Duration::from_micros(100));
                                }
                                // while received.len() < count {}
                            },
                            Err(err) => {
                                warn!("incoming connection - error: {:?}", err);
                            },
                        }
                    }
                },
                Err(err) => {
                    // connectExit.send(true).unwrap();
                    // okRef.store(false, Ordering::SeqCst);
                    panic!("Preparing test TCP server - error: {:?}", err);
                },
            };
        });
    }
}

struct MockStreamRead<T> {
    buffer: Vec<T>
}
impl<T> StreamRead<T, String> for MockStreamRead<T> {
    fn read(&mut self) -> Result<T, String> {
        match self.buffer.pop() {
            Some(value) => Ok(value),
            None => Err(format!("Buffer is empty")),
        }
    }
}
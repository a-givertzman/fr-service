#![allow(non_snake_case)]

use crate::tcp::steam_read::StreamRead;
#[cfg(test)]
mod tests {
    use log::{warn, info, debug, error};
    use rand::Rng;
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
    fn randomBytes(len: usize) -> Vec<u8> {
        let mut rnd = rand::thread_rng();
        let mut bytes = vec![];
        for _ in 0..len {
            bytes.push(rnd.gen_range(0..255));
        }
        bytes
    }

    #[test]
    fn test_() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_");
        let count = 100000;
        let maxTestDuration = Duration::from_secs(10);
        let mut sent = 0;
        let received = Arc::new(Mutex::new(vec![]));
        let messageLen = 10;
        let mut testData = vec![];
        for _ in 0..count {
            testData.push(randomBytes(messageLen))
        }
        let mut tcpStreamWrite = TcpStreamWrite::new(
            "test",
            true,
            Some(10000),
            Box::new(MockStreamRead { buffer: testData.clone()}),
        );
        let addr = "127.0.0.1:".to_owned() + &TestSession::freeTcpPortStr();

        mockTcpServer(addr.clone(), count, messageLen, received.clone());
        thread::sleep(Duration::from_micros(100));
        let mut stream = TcpStream::connect(addr).unwrap();
        thread::sleep(Duration::from_micros(100));

        let timer = Instant::now();
        while sent < count {
            match tcpStreamWrite.write(&mut stream) {
                Ok(_) => {
                    sent += 1;
                },
                Err(err) => {
                    panic!("sent: {}/{}, error: {}", sent, count, err);
                },
            };
            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        let waitDuration = Duration::from_millis(10);
        let mut waitAttempts = maxTestDuration.as_micros() / waitDuration.as_micros();
        while received.lock().unwrap().len() < count {
            debug!("waiting while all data beeng received {}/{}...", received.lock().unwrap().len(), count);
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} points taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), maxTestDuration);
        }
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", count);
        println!("sent events: {:?}", sent);
        let received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent == count, "sent: {:?}\ntarget: {:?}", sent, count);
        assert!(received.len() == count, "received: {:?}\ntarget: {:?}", received.len(), count);

        // assert!(sent == received.lock().unwrap().len(), "\nresult: {:?}\ntarget: {:?}", sent, received.lock().unwrap().len());
    }
    ///
    /// TcpServer setup
    fn mockTcpServer(addr: String, count: usize, messageLen: usize, received: Arc<Mutex<Vec<Vec<u8>>>>) {
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            match TcpListener::bind(&addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok ({})", addr);
                    let mut acceptCount = 2;
                    // let mut maxReadErrors = 3;
                    while acceptCount > 0 {
                        acceptCount -= 1;
                        match listener.accept() {
                            Ok((mut _socket, addr)) => {
                                info!("TCP server | accept connection - ok\n\t{:?}", addr);
                                let mut buffer = Vec::new();
                                for _ in 0..count {
                                    let mut bytes = vec![0u8; messageLen];
                                    match _socket.read(&mut bytes) {
                                        Ok(_) => {
                                            buffer.append(&mut bytes);
                                            if buffer.len() >= messageLen {
                                                let v = buffer.drain(0..messageLen).collect();
                                                received.lock().unwrap().push(v);
                                            }
                                        },
                                        Err(err) => {
                                            warn!("{:?}", err);
                                        },
                                    }
                                }
                                info!("TCP server | all received: {:?}", received.lock().unwrap().len());
                                while received.lock().unwrap().len() < count {
                                    thread::sleep(Duration::from_micros(100));
                                }
                                thread::sleep(Duration::from_micros(100));
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
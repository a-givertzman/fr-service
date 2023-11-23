#![allow(non_snake_case)]

use crate::tcp::steam_read::StreamRead;
#[cfg(test)]
mod tests {
    use log::{warn, info, debug, error};
    use rand::Rng;
    use std::{sync::{Once, Arc, Mutex, atomic::{AtomicUsize, Ordering, AtomicU8}}, time::{Duration, Instant}, thread, net::{TcpListener, TcpStream}, io::Read};
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
    static index: AtomicU8 = AtomicU8::new(0);
    fn randomBytes(len: usize) -> Vec<u8> {
        let mut rnd = rand::thread_rng();
        let mut bytes = vec![];
        // for _ in 0..len {
        //     bytes.push(rnd.gen_range(0..255));
        // }
        let ix = index.load(Ordering::SeqCst);
        for i in ix..ix + 10 {
            bytes.push(i);
        }
        index.fetch_add(10, Ordering::SeqCst);
        bytes
    }

    #[test]
    fn test_() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_");
        let count = 10;
        let maxTestDuration = Duration::from_secs(10);
        let sent = Arc::new(AtomicUsize::new(0));
        let received = Arc::new(Mutex::new(vec![]));
        let messageLen = 10;
        let mut testData = vec![];
        for _ in 0..count {
            testData.push(randomBytes(messageLen))
        }
        info!("testData: {:?}", testData);
        let mut tcpStreamWrite = TcpStreamWrite::new(
            "test",
            true,
            Some(10000),
            Box::new(MockStreamRead { buffer: testData.clone()}),
        );
        let addr = "127.0.0.1:".to_owned() + &TestSession::freeTcpPortStr();

        mockTcpServer(addr.clone(), count, messageLen, sent.clone(), received.clone());
        thread::sleep(Duration::from_micros(100));

        let timer = Instant::now();

        while sent.load(Ordering::SeqCst) < count {
            // warn!("sent: {}/{}", sent, count);
            match TcpStream::connect(&addr) {
                Ok(mut stream) => {
                    'inner: while sent.load(Ordering::SeqCst) < count {
                        match tcpStreamWrite.write(&mut stream) {
                            ConnectionStatus::Active(result) => {
                                match result {
                                    Ok(sentBytes) => {
                                        sent.fetch_add(1, Ordering::SeqCst);
                                        warn!("sent: {}/{} \t bytes: {}", sent.load(Ordering::SeqCst), count, sentBytes);
                                    },
                                    Err(err) => {
                                        warn!("sent: {}/{}, socket write error: {}", sent.load(Ordering::SeqCst), count, err);
                                    },
                                }
                            },
                            ConnectionStatus::Closed(err) => {
                                warn!("sent: {}/{}, socket closed, error: {}", sent.load(Ordering::SeqCst), count, err);
                                break 'inner;
                            }
                        };
                    }
                },
                Err(err) => {
                    warn!("sent: {}/{}, connection error: {}", sent.load(Ordering::SeqCst), count, err);
                    thread::sleep(Duration::from_millis(100));
                },
            };
            assert!(timer.elapsed() < maxTestDuration, "Transfering {}/{} messages taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), maxTestDuration);
            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            warn!("sent total: {}/{}", sent.load(Ordering::SeqCst), count);
        }
        let waitDuration = Duration::from_millis(10);
        let mut waitAttempts = maxTestDuration.as_micros() / waitDuration.as_micros();
        while received.lock().unwrap().len() < count {
            debug!("waiting while all data beeng received {}/{}...", received.lock().unwrap().len(), count);
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} messages taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), maxTestDuration);
        }
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", count);
        println!("sent events: {:?}", sent);
        let mut received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent.load(Ordering::SeqCst) == count, "sent: {:?}\ntarget: {:?}", sent, count);
        assert!(received.len() == count, "received: {:?}\ntarget: {:?}", received.len(), count);
        for target in testData {
            let result = match received.first() {
                Some(bytes) => {
                    debug!("\nresult: {:?}\ntarget: {:?}", bytes, target);
                    bytes
                },
                None => panic!("received is empty"),
            };
            assert!(result == &target, "\nresult: {:?}\ntarget: {:?}", result, target);
            received.remove(0);
        }
    }
    ///
    /// TcpServer setup
    fn mockTcpServer(addr: String, count: usize, messageLen: usize, sent: Arc<AtomicUsize>, received: Arc<Mutex<Vec<Vec<u8>>>>) {
        thread::spawn(move || {
            info!("TCP server | Preparing test server...");
            let mut state = 0;
            match TcpListener::bind(&addr) {
                Ok(listener) => {
                    info!("TCP server | Preparing test server - ok ({})", addr);
                    let mut acceptCount = 2;
                    // let mut maxReadErrors = 3;
                    while acceptCount > 0 {
                        acceptCount -= 1;
                        info!("TCP server | waiting connection...");
                        match listener.accept() {
                            Ok((mut _socket, addr)) => {
                                info!("TCP server | accept connection - ok\n\t{:?}", addr);
                                let mut buffer = Vec::new();
                                while received.lock().unwrap().len() < count {
                                    let mut bytes = vec![0u8; messageLen];
                                    match _socket.read(&mut bytes) {
                                        Ok(_) => {
                                            buffer.append(&mut bytes);
                                            if buffer.len() >= messageLen {
                                                let v = buffer.drain(0..messageLen).collect();
                                                debug!("TCP server | received: {:?}", v);
                                                received.lock().unwrap().push(v);
                                                // if (state == 0) && sent.load(Ordering::SeqCst) as f64 / count as f64 > 0.333 {
                                                //     state = 1;
                                                //     let duration = Duration::from_millis(500);
                                                //     debug!("TCP server | beaking socket connection for {:?}", duration);
                                                //     _socket.shutdown(std::net::Shutdown::Both).unwrap();
                                                //     thread::sleep(duration);
                                                //     debug!("TCP server | beaking socket connection for {:?} - elapsed, restoring...", duration);
                                                //     break;
                                                // }
                                            }
                                        },
                                        Err(err) => {
                                            warn!("{:?}", err);
                                        },
                                    }
                                }
                                info!("TCP server | all received: {:?}", received.lock().unwrap().len());
                                // if state > 0 {
                                //     while received.lock().unwrap().len() < count {
                                //         info!("TCP server | await while all being received: {:?}", received.lock().unwrap().len());
                                //         thread::sleep(Duration::from_millis(200));
                                //     }
                                // }
                            },
                            Err(err) => {
                                warn!("TCP server | incoming connection - error: {:?}", err);
                            },
                        }
                    }
                },
                Err(err) => {
                    // connectExit.send(true).unwrap();
                    // okRef.store(false, Ordering::SeqCst);
                    panic!("TCP server | Preparing server - error: {:?}", err);
                },
            };
        });
    }
}

struct MockStreamRead<T> {
    buffer: Vec<T>
}
impl<T: Sync> StreamRead<T, String> for MockStreamRead<T> {
    fn read(&mut self) -> Result<T, String> {
        match self.buffer.first() {
            Some(_) => Ok(self.buffer.remove(0)),
            None => Err(format!("Buffer is empty")),
        }
    }
}
#![allow(non_snake_case)]

use crate::{core_::{failure::recv_error::RecvError, object::object::Object}, tcp::steam_read::StreamRead};
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use rand::Rng;
    use std::{sync::{Once, Arc, Mutex, atomic::{AtomicUsize, Ordering}}, time::{Duration, Instant}, thread, net::{TcpListener, TcpStream}, io::Read};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::session::test_session::TestSession;
    use crate::{core_::net::connection_status::ConnectionStatus, tcp::tcp_stream_write::TcpStreamWrite, tests::unit::tcp::tcp_stream_write_test::MockStreamRead}; 
    
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
    static INDEX: AtomicUsize = AtomicUsize::new(0);
    fn randomBytes(len: usize) -> Vec<u8> {
        let mut rnd = rand::thread_rng();
        let mut bytes = vec![];
        let ix = INDEX.load(Ordering::SeqCst);
        for _ in ix..ix + len {
            let b = rnd.gen_range(0..255);
            bytes.push(b);
        }
        INDEX.fetch_add(10, Ordering::SeqCst);
        bytes
    }

    #[test]
    fn test_TcpStreamWrite() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test TcpStreamWrite";
        println!("\n{}", self_id);
        let count = 1000;
        let test_duration = Duration::from_secs(10);
        let sent = Arc::new(AtomicUsize::new(0));
        let received = Arc::new(Mutex::new(vec![]));
        let messageLen = 10;
        let mut test_data = vec![];
        for _ in 0..count {
            test_data.push(randomBytes(messageLen))
        }
        info!("test_data: {:?}", test_data);
        let mut tcpStreamWrite = TcpStreamWrite::new(
            "test",
            true,
            Some(10000),
            Box::new(MockStreamRead::new( self_id, test_data.clone())),
        );
        let addr = "127.0.0.1:".to_owned() + &TestSession::free_tcp_port_str();

        mockTcpServer(addr.clone(), count, messageLen, received.clone());
        thread::sleep(Duration::from_micros(100));

        let timer = Instant::now();

        while sent.load(Ordering::SeqCst) < count {
            // warn!("sent: {}/{}", sent, count);
            match TcpStream::connect(&addr) {
                Ok(mut stream) => {
                    'inner: while sent.load(Ordering::SeqCst) < count {
                        match tcpStreamWrite.write(&mut stream, &None) {
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
            assert!(timer.elapsed() < test_duration, "Transfering {}/{} messages taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), test_duration);
            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            warn!("sent total: {}/{}", sent.load(Ordering::SeqCst), count);
        }
        let waitDuration = Duration::from_millis(10);
        let mut waitAttempts = test_duration.as_micros() / waitDuration.as_micros();
        while received.lock().unwrap().len() < count {
            debug!("waiting while all data beeng received {}/{}...", received.lock().unwrap().len(), count);
            thread::sleep(waitDuration);
            waitAttempts -= 1;
            assert!(waitAttempts > 0, "Transfering {}/{} messages taks too mach time {:?} of {:?}", received.lock().unwrap().len(), count, timer.elapsed(), test_duration);
        }
        println!("elapsed: {:?}", timer.elapsed());
        println!("total test events: {:?}", count);
        println!("sent events: {:?}", sent);
        let mut received = received.lock().unwrap();
        println!("recv events: {:?}", received.len());
        assert!(sent.load(Ordering::SeqCst) == count, "sent: {:?}\ntarget: {:?}", sent, count);
        assert!(received.len() == count, "received: {:?}\ntarget: {:?}", received.len(), count);
        for target in test_data {
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
                                            }
                                        },
                                        Err(err) => {
                                            warn!("{:?}", err);
                                        },
                                    }
                                }
                                info!("TCP server | all received: {:?}", received.lock().unwrap().len());
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
    id: String,
    buffer: Vec<T>
}
impl<T> MockStreamRead<T> {
    pub fn new(parent: &str, buffer: Vec<T>) -> Self {
        Self { id: format!("{}/{}", parent, "MockStreamRead"), buffer }
    }
}
impl<T> Object for MockStreamRead<T> {
    fn id(&self) -> &str {
        &self.id
    }
}
impl<T: Sync> StreamRead<T, RecvError> for MockStreamRead<T> {
    fn read(&mut self) -> Result<T, RecvError> {
        match self.buffer.first() {
            Some(_) => Ok(self.buffer.remove(0)),
            None => Err(RecvError::Timeout),   //Err(format!("Buffer is empty")),
        }
    }
}
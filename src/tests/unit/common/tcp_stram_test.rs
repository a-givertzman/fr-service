#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{info, warn, debug};
    use std::{sync::{Once, mpsc}, net::{TcpStream, TcpListener}, io::{Read, Write, BufReader}, thread::{self, JoinHandle}, time::Duration};
    use crate::core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, constants::constants::RECV_TIMEOUT, testing::{test_session::TestSession, test_stuff::{wait::WaitTread, max_test_duration::TestDuration}}}; 
    
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

    #[ignore = "common - all must be ignored"]
    #[test]
    fn test_tcp_stream_read() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test TcpStream read on close";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let tcpPort = TestSession::freeTcpPortStr();
        let tcpAddr = format!("127.0.0.1:{}", tcpPort);
        let handle = server(&tcpAddr).unwrap();
        thread::sleep(Duration::from_millis(100));
        match TcpStream::connect(tcpAddr) {
            Ok(stream) => {
                match stream.set_read_timeout(Some(RECV_TIMEOUT)) {
                    Ok(_) => {
                        info!("{}.setStreamTimout | Socket set read timeout {:?} - ok", selfId, RECV_TIMEOUT);
                    },
                    Err(err) => {
                        warn!("{}.setStreamTimout | Socket set read timeout error {:?}", selfId, err);
                    },
                }
                let stream = BufReader::new(stream);
                for byte in stream.bytes() {
                    debug!("{}.run | received byte: {:?}", selfId, byte);
                }
            },
            Err(err) => {
                panic!("{}.run | TcpStream::connect error: {:?}", selfId, err);
            },
        }
        debug!("{}.run | TcpStream::read finished", selfId);
        handle.wait().unwrap();
        testDuration.exit();
    }
    ///
    /// 
    fn server(addr: &str) -> Result<JoinHandle<()>, std::io::Error> {
        let selfId = "Emuleted TcpServer";
        let addr = addr.to_string();
        info!("{}.run | Preparing thread...", selfId);
        let handle = thread::Builder::new().name(format!("{}.run", selfId.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", selfId);
            match TcpListener::bind(addr.clone()) {
                Ok(listener) => {
                    info!("{}.run | Open socket {} - ok", selfId, addr);
                    for stream in listener.incoming() {
                        // if exit.load(Ordering::SeqCst) {
                        //     debug!("{}.run | Detected exit", selfId);
                        //     break;
                        // }
                        // match stream {
                        //     Ok(mut stream) => {
                        //         let mut buf = vec![];
                        //         match stream.read(&mut buf) {
                        //             Ok(len) => {
                        //                 debug!("{}.run | received {} bytes", selfId, len);
                        //             },
                        //             Err(err) => {
                        //                 warn!("{}.run | TcpListener::bind error: {:?}", selfId, err);
                        //             },
                        //         }
                        //     },
                        //     Err(err) => {
                        //         panic!("{}.run | TcpListener::incoming error: {:?}", selfId, err);
                        //     },
                        // }
                        match stream {
                            Ok(mut stream) => {
                                match stream.set_read_timeout(Some(RECV_TIMEOUT)) {
                                    Ok(_) => {
                                        info!("{}.setStreamTimout | Socket set read timeout {:?} - ok", selfId, RECV_TIMEOUT);
                                    },
                                    Err(err) => {
                                        warn!("{}.setStreamTimout | Socket set read timeout error {:?}", selfId, err);
                                    },
                                }                
                                let mut buf = vec![0, 1, 2, 3];
                                match stream.write(&mut buf) {
                                    Ok(len) => {
                                        // debug!("{}.run | received {} bytes", selfId, len);
                                        info!("{}.run | sent {} bytes - ok", selfId, len);
                                        thread::sleep(Duration::from_secs(3));
                                        drop(stream);
                                    },
                                    Err(err) => {
                                        warn!("{}.run | TcpListener::bind error: {:?}", selfId, err);
                                    },
                                }
                            },
                            Err(err) => {
                                panic!("{}.run | TcpListener::incoming error: {:?}", selfId, err);
                            },
                        }
                        break;
                    }
                },
                Err(err) => {
                    warn!("{}.run | TcpListener::bind error: {:?}", selfId, err);
                },
            };
            info!("{}.run | Exit...", selfId);
        });
        handle
    }
}

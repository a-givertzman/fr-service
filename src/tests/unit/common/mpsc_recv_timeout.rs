#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug, error};
    use std::{sync::{Once, mpsc::{self, RecvTimeoutError}}, time::{Duration, Instant}, thread::{self, JoinHandle}, any::Any};
    use crate::core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, types::type_of::TypeOf, constants::constants::RECV_TIMEOUT}; 
    
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
    fn test_mpsc_receiver() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        println!("test mpsc::Receiver");
        let selfId = "test";
        let (send, recv) = mpsc::channel();
        let iterations = 3;
        let _h = thread::Builder::new().name(selfId.to_string()).spawn(move || {
            for value in 0..=iterations {
                send.send(value).unwrap();
                thread::sleep(Duration::from_millis(1000))
            }
            drop(send);
        }).unwrap();

        let mut exit = false;
        while !exit {
            match recv.recv_timeout(RECV_TIMEOUT) {
                Ok(value) => {
                    info!("value: {}", value);
                },
                Err(err) => {
                    match err {
                        RecvTimeoutError::Timeout => {
                            error!("debug: {}", err);
                        },
                        RecvTimeoutError::Disconnected => {
                            error!("error: {}", err);
                            thread::sleep(Duration::from_millis(1000));
                            exit = true;
                        },
                    }
                },
            };
            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        waitForThread(_h).unwrap();
    }
    ///
    /// 
    fn waitForThread(thd: JoinHandle<()>) -> Result<(), Box<dyn Any + Send>>{
        let thdId = format!("{:?}-{:?}", thd.thread().id(), thd.thread().name());
        info!("Waiting for thread: {:?}...", thdId);
        let r = thd.join();
        match &r {
            Ok(_) => {
                info!("Waiting for thread: '{}' - finished", thdId);
            },
            Err(err) => {
                error!("Waiting for thread '{}' error: {:?}", thdId, err);                
            },
        }
        r
    }
}

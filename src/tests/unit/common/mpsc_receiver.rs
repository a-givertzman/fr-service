#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use std::{sync::{Once, mpsc}, time::{Duration, Instant}};
    use crate::core_::debug::debug_session::{DebugSession, LogLevel, Backtrace}; 
    
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
        let (send, recv) = mpsc::channel();
        let iterations = 10000;
        for value in 0..=iterations {
            send.send(value).unwrap();
        }
        drop(send);
        let mut value = -1;
        while value < iterations {
            value = recv.recv_timeout(Duration::from_millis(100)).unwrap();
            info!("value: {}", value);
            // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
    }
}

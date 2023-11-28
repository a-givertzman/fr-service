#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use crate::core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, retain_buffer::retain_buffer::RetainBuffer}; 
    
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
    fn test_retain_buffer() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_retain_buffer");
        let mut buffer = RetainBuffer::new("test", "", Some(3));
        buffer.push(11);
        buffer.push(12);
        buffer.push(13);
        let result = buffer.popFirst().unwrap();
        let target = 11;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst().unwrap();
        let target = 12;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst().unwrap();
        let target = 13;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst();
        let target = None;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }

    #[test]
    fn test_retain_buffer_capacity() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test_retain_buffer");
        let mut buffer = RetainBuffer::new("test", "", Some(3));
        buffer.push(11);
        let result = buffer.len();
        let target = 1;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        buffer.push(12);
        let result = buffer.len();
        let target = 2;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        buffer.push(13);
        let result = buffer.len();
        let target = 3;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst().unwrap();
        let target = 11;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst().unwrap();
        let target = 12;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst().unwrap();
        let target = 13;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst();
        let target = None;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.len();
        let target = 0;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        buffer.push(14);
        buffer.push(15);
        buffer.push(16);
        buffer.push(17);
        let result = buffer.popFirst().unwrap();
        let target = 15;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst().unwrap();
        let target = 16;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst().unwrap();
        let target = 17;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        let result = buffer.popFirst();
        let target = None;
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
    }
}

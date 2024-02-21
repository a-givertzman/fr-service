#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::core_::cot::cot::{Cot, Direction}; 
    
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
    fn test_cot() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test Template";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            // match
            (true, Cot::Inf, Direction::Read),
            (true, Cot::Act, Direction::Write),
            (true, Cot::ActCon, Direction::Read),
            (true, Cot::ActErr, Direction::Read),
            (true, Cot::Req, Direction::Write),
            (true, Cot::ReqCon, Direction::Read),
            (true, Cot::ReqErr, Direction::Read),
            // not match
            (false, Cot::Inf, Direction::Write),
            (false, Cot::Act, Direction::Read),
            (false, Cot::ActCon, Direction::Write),
            (false, Cot::ActErr, Direction::Write),
            (false, Cot::Req, Direction::Read),
            (false, Cot::ReqCon, Direction::Write),
            (false, Cot::ReqErr, Direction::Write),
        ];
        for (target, left, right) in test_data {
            let result = left & right;
            println!("cot: {:?}, direction: {:?} | result: {}", left, right, result);
            println!("left: {:#08b}({:?}), right: {:#08b}({:?}) | result: {:#08b}({:?})", left, left, right, right, result, result);
            assert!((result > 0) == target, "\nresult: {:?}\ntarget: {:?}", result, left as u32 & right as u32);
            assert!(right.contains(left) == target, "\nresult: {:?}\ntarget: {:?}", result, left as u32 & right as u32);
        }
        test_duration.exit();
    }
}

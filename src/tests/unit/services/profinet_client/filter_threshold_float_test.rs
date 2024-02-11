#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::core_::filter::{filter::Filter, filter_threshold::FilterThreshold};
    
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
    fn test_FilterThresholdAbs_pos() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test FilterThresholdAbs 0.0 - 1.0 - 0.0";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let testData = [
            (0.0, 0.0),
            (0.1, 0.0),
            (0.2, 0.2),
            (0.3, 0.2),
            (0.4, 0.4),
            (0.5, 0.4),
            (0.6, 0.6),
            (0.7, 0.6),
            (0.8, 0.8),
            (0.9, 0.8),
            (1.0, 1.0),
            (1.0, 1.0),
            (0.9, 1.0),
            (0.8, 0.8),
            (0.7, 0.8),
            (0.6, 0.6),
            (0.5, 0.6),
            (0.4, 0.4),
            (0.3, 0.4),
            (0.2, 0.2),
            (0.1, 0.2),
            (0.0, 0.0),
        ];
        let threasold = 0.15;
        let mut filter = FilterThreshold::new(0.0, threasold, 0.0);
        let mut prev = 0.0;
        for (value, target) in testData {
            filter.add(value);
            let diff = (prev as f64 - (value as f64)).abs();
            if diff > threasold {
                prev = value;
            }
            let result = filter.value();
            println!("{}    in: {}   |   out: {}   |   diff: {}", selfId, value, result, diff);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }

    #[test]
    fn test_FilterThresholdAbs_neg() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test FilterThresholdAbs (-1.0) - 1.0 - (-1.0)";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let testData = [
            (-1.0, -1.0),
            (-0.9, -1.0),
            (-0.8, -0.8),
            (-0.7, -0.8),
            (-0.6, -0.6),
            (-0.5, -0.6),
            (-0.4, -0.4),
            (-0.3, -0.4),
            (-0.2, -0.2),
            (-0.1, -0.2),
            (0.0, 0.0),
            (0.1, 0.0),
            (0.2, 0.2),
            (0.3, 0.2),
            (0.4, 0.4),
            (0.5, 0.4),
            (0.6, 0.6),
            (0.7, 0.6),
            (0.8, 0.8),
            (0.9, 0.8),
            (1.0, 1.0),
            (1.0, 1.0),
            (0.9, 1.0),
            (0.8, 0.8),
            (0.7, 0.8),
            (0.6, 0.6),
            (0.5, 0.6),
            (0.4, 0.4),
            (0.3, 0.4),
            (0.2, 0.2),
            (0.1, 0.2),
            (0.0, 0.0),
            (-0.1, 0.0),
            (-0.2, -0.2),
            (-0.3, -0.2),
            (-0.4, -0.4),
            (-0.5, -0.4),
            (-0.6, -0.6),
            (-0.7, -0.6),
            (-0.8, -0.8),
            (-0.9, -0.8),
            (-1.0, -1.0),
        ];
        let threasold = 0.15;
        let mut filter = FilterThreshold::new(0.0, threasold, 0.0);
        let mut prev = 0.0;
        for (value, target) in testData {
            filter.add(value);
            let diff = (prev as f64 - (value as f64)).abs();
            if diff > threasold {
                prev = value;
            }
            let result = filter.value();
            println!("{}    in: {}   |   out: {}   |   diff: {}", selfId, value, result, diff);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }
    
    
    #[test]
    fn test_FilterThreshold_factor_pos() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test FilterThresholdAbs 0.0 - 1.0 - 0.0 | factor";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let testData = [
            (0.0, 0.0),
            (0.1, 0.0),
            (0.2, 0.0),
            (0.3, 0.0),
            (0.4, 0.4),
            (0.5, 0.4),
            (0.6, 0.4),
            (0.7, 0.4),
            (0.8, 0.8),
            (0.9, 0.8),
            (1.0, 0.8),
            (1.0, 0.8),
            (0.9, 0.8),
            (0.8, 0.8),
            (0.7, 0.8),
            (0.6, 0.8),
            (0.5, 0.8),
            (0.4, 0.8),
            (0.3, 0.3),
            (0.2, 0.3),
            (0.1, 0.3),
            (0.0, 0.3),
        ];
        let threasold = 1.0;
        let mut filter = FilterThreshold::new(0.0, threasold, 1.5);
        let mut prev = 0.0;
        for (value, target) in testData {
            filter.add(value);
            let diff = (prev as f64 - (value as f64)).abs();
            if diff > threasold {
                prev = value;
            }
            let result = filter.value();
            println!("{}    in: {}   |   out: {}   |   diff: {}", selfId, value, result, diff);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }

    #[test]
    fn test_FilterThreshold_factor_neg() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test FilterThresholdAbs (-1.0) - 1.0 - (-1.0) | factor";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let testData = [
            (-1.0, -1.0),
            (-0.9, -1.0),
            (-0.8, -1.0),
            (-0.7, -1.0),
            (-0.6, -0.6),
            (-0.5, -0.6),
            (-0.4, -0.6),
            (-0.3, -0.6),
            (-0.2, -0.2),
            (-0.1, -0.2),
            (0.0, -0.2),
            (0.1, -0.2),
            (0.2, 0.2),
            (0.3, 0.2),
            (0.4, 0.2),
            (0.5, 0.2),
            (0.6, 0.6),
            (0.7, 0.6),
            (0.8, 0.6),
            (0.9, 0.6),
            (1.0, 1.0),
            (1.0, 1.0),
            (0.9, 1.0),
            (0.8, 1.0),
            (0.7, 1.0),
            (0.6, 0.6),
            (0.5, 0.6),
            (0.4, 0.6),
            (0.3, 0.6),
            (0.2, 0.2),
            (0.1, 0.2),
            (0.0, 0.2),
            (-0.1, 0.2),
            (-0.2, -0.2),
            (-0.3, -0.2),
            (-0.4, -0.2),
            (-0.5, -0.2),
            (-0.6, -0.6),
            (-0.7, -0.6),
            (-0.8, -0.6),
            (-0.9, -0.6),
            (-1.0, -1.0),
        ];
        let threasold = 1.0;
        let mut filter = FilterThreshold::new(0.0, threasold, 1.5);
        let mut prev = 0.0;
        for (value, target) in testData {
            filter.add(value);
            let diff = (prev as f64 - (value as f64)).abs();
            if diff > threasold {
                prev = value;
            }
            let result = filter.value();
            println!("{}    in: {}   |   out: {}   |   diff: {}", selfId, value, result, diff);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        testDuration.exit();
    }        
}

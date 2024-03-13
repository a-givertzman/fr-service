#[cfg(test)]

mod tests {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration; 
    use crate::conf::point_config::point_name::PointName; 
    
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
    fn test_point_name() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test PointName";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            ("", "Point.Name.0", "/Point.Name.0"),
            ("Path1", "", "/Path1"),
            ("Path", "Point.Name.1", "/Path/Point.Name.1"),
            ("Path", "/Point.Name.2", "/Path/Point.Name.2"),
            ("Path/", "Point.Name.3", "/Path/Point.Name.3"),
            ("Path/", "/Point.Name.4", "/Path/Point.Name.4"),
            ("/Path/", "Point.Name.5", "/Path/Point.Name.5"),
            ("/Path/", "/Point.Name.6", "/Path/Point.Name.6"),
        ];
        for (parent, name, target) in test_data {
            let result = PointName::new(parent, name).full();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
}

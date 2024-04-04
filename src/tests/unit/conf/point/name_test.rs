#[cfg(test)]

mod name {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use log::debug;
    use testing::stuff::max_test_duration::TestDuration; 
    use crate::conf::point_config::name::Name; 
    ///
    /// 
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
    fn init_each() -> () {}
    ///
    /// 
    #[test]
    fn join() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
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
        for (parent, me, target) in test_data {
            let name = Name::new(parent, me);
            debug!("Display | '{}' + '{}': \t'{}'", parent, me, name);
            debug!("Debug   | {:?} + {:?}: \t{:?}", parent, me, name);
            debug!("Debug   | {:#?} + {:#?}: \t{:#?}", parent, me, name);
            let result = name.join();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            let result = name.parent();
            assert!(result == parent, "\nresult: {:?}\ntarget: {:?}", result, parent);
            let result = name.me();
            assert!(result == me, "\nresult: {:?}\ntarget: {:?}", result, me);
        }
        test_duration.exit();
    }
    ///
    /// 
    #[test]
    fn into_string() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
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
        for (parent, me, target) in test_data {
            let name = Name::new(parent, me);
            debug!("Display | '{}' + '{}': \t'{}'", parent, me, name);
            debug!("Debug   | {:?} + {:?}: \t{:?}", parent, me, name);
            debug!("Debug   | {:#?} + {:#?}: \t{:#?}", parent, me, name);
            let result: String = name.clone().into();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
            let result = name.parent();
            assert!(result == parent, "\nresult: {:?}\ntarget: {:?}", result, parent);
            let result = name.me();
            assert!(result == me, "\nresult: {:?}\ntarget: {:?}", result, me);
        }
        test_duration.exit();
    }
}

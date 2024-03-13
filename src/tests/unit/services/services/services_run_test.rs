#[cfg(test)]

mod services {
    use log::{debug, trace};
    use std::{sync::Once, env, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::services::services::Services;
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
    fn run() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let mut path = env::current_dir().unwrap();
        path.push("src/tests/unit/services/services/");
        std::env::set_current_dir(path).unwrap();
        debug!("dir: {:?}", env::current_dir());
        // let path = "./src/tests/unit/services/services/services_test_points.yaml";
        // let config = TaskConfig::read(path);
        // trace!("config: {:?}", &config);
        // println!(" points: {:?}", config.points());
        let services = Services::new(self_id);
        services.run().unwrap();
        println!("\n");
        // println!(" points count: {:?}", points_count);
        // for point in points {
        //     println!("\t {:?}", point);
        // }
        // assert!(points_count == target, "\nresult: {:?}\ntarget: {:?}", points_count, target);
        test_duration.exit();
    }
}


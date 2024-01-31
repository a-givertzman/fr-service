#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{trace, info, debug};
    use std::{sync::{Once, Arc, Mutex}, env, time::{Instant, Duration}};
    
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::test_stuff::{random_test_values::RandomTestValues, test_value::Value, wait::WaitTread, max_test_duration::TestDuration}}, 
        conf::task_config::TaskConfig, 
        services::{task::{task::Task, task_test_receiver::TaskTestReceiver, task_test_producer::TaskTestProducer}, service::Service, services::Services},
    };
    
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
    fn test_task_points() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test Task.points";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_points.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
        println!(" points: {:?}", config.points());
        let services = Arc::new(Mutex::new(Services::new(selfId)));        
        let task = Arc::new(Mutex::new(Task::new(selfId, config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());
        let target  = 3;
        let points = services.lock().unwrap().points();
        let pointsCount = points.len();
        println!("\n");
        println!(" points count: {:?}", pointsCount);
        for point in points {
            println!("\t {:?}", point);
        }
        assert!(pointsCount == target, "\nresult: {:?}\ntarget: {:?}", pointsCount, target);
        testDuration.exit();
    }
}

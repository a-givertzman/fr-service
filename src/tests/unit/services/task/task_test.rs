#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{trace, info, debug};
    use std::{sync::{Once, Arc, Mutex}, env, time::{Instant, Duration}};
    
    use crate::{
        core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::test_stuff::{random_test_values::RandomTestValues, test_value::Value, wait::WaitTread, max_test_duration::MaxTestDuration}}, 
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
    fn test_task_struct() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_struct");
        let selfId = "test";
        let maxTestDuration = MaxTestDuration::new(selfId, Duration::from_secs(10));
        maxTestDuration.run().unwrap();

        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_struct.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
        
        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            selfId,
            "in-queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        
        let testData = RandomTestValues::new(
            selfId, 
            vec![], 
            iterations, 
        );
        let testData: Vec<Value> = testData.collect();
        let totalCount = testData.len();
        assert!(totalCount == iterations, "\nresult: {:?}\ntarget: {:?}", totalCount, iterations);
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            selfId, 
            "Task.recv-queue",
            services.clone(),
            testData,
        )));
        
        let task = Arc::new(Mutex::new(Task::new(selfId, config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());
        
        let receiverHandle = receiver.lock().unwrap().run().unwrap();
        let producerHandle = producer.lock().unwrap().run().unwrap();
        trace!("task runing...");
        let time = Instant::now();
        let taskHandle = task.lock().unwrap().run().unwrap();
        trace!("task runing - ok");
        producerHandle.wait().unwrap();
        receiverHandle.wait().unwrap();
        debug!("task.lock.exit...");
        task.lock().unwrap().exit();
        debug!("task.lock.exit - ok");
        taskHandle.wait().unwrap();
        let sent = producer.lock().unwrap().sent().lock().unwrap().len();
        let result = receiver.lock().unwrap().received().lock().unwrap().len();
        println!(" elapsed: {:?}", time.elapsed());
        println!("    sent: {:?}", sent);
        println!("received: {:?}", result);
        assert!(sent == iterations, "\nresult: {:?}\ntarget: {:?}", sent, iterations);
        assert!(result == iterations, "\nresult: {:?}\ntarget: {:?}", result, iterations);
        maxTestDuration.exit();
    }


    #[ignore = "TODO - transfered values assertion not implemented yet"]
    #[test]
    fn test_task_tranfer() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_transfer");
        let selfId = "test";
        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_struct.yaml";
        // let path = "./src/tests/unit/task/task_test.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
    
        let services = Arc::new(Mutex::new(Services::new(selfId)));
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            selfId,
            "in-queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        
        let testData = RandomTestValues::new(
            selfId, 
            vec![
                Value::Float(f64::MAX),
                Value::Float(f64::MIN),
                Value::Float(f64::MIN_POSITIVE),
                Value::Float(-f64::MIN_POSITIVE),
                Value::Float(0.11),
                Value::Float(1.33),
            ], 
            iterations, 
        );
        let testData: Vec<Value> = testData.collect();
        // let totalCount = testData.len();
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            selfId,
            "Task.recv-queue",
            services.clone(),
            testData,
        )));
    
        let task = Arc::new(Mutex::new(Task::new(selfId, config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());

        let receiverHandle = receiver.lock().unwrap().run().unwrap();
        let producerHandle = producer.lock().unwrap().run().unwrap();
        trace!("task runing...");
        let time = Instant::now();
        task.lock().unwrap().run().unwrap();
        trace!("task runing - ok");
        producerHandle.join().unwrap();
        receiverHandle.join().unwrap();
        let producerSent = producer.lock().unwrap().sent();
        let sent = producerSent.lock().unwrap();
        let receiverReceived = receiver.lock().unwrap().received();
        let mut received = receiverReceived.lock().unwrap();
        println!(" elapsed: {:?}", time.elapsed());
        println!("    sent: {:?}", sent.len());
        println!("received: {:?}", received.len());
        assert!(sent.len() == iterations, "\nresult: {:?}\ntarget: {:?}", sent.len(), iterations);
        assert!(received.len() == iterations, "\nresult: {:?}\ntarget: {:?}", received.len(), iterations);
        for sentPoint in sent.iter() {
            let recvPoint = received.pop().unwrap();
            assert!(&recvPoint == sentPoint, "\nresult: {:?}\ntarget: {:?}", recvPoint, sentPoint);
        }
    }
}


#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{trace, info};
    use std::{sync::{Once, Arc, Mutex}, env, time::Instant};
    
    use crate::{
        core_::debug::debug_session::{DebugSession, LogLevel, Backtrace}, 
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
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_struct");
        
        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_struct.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
        
        let services = Arc::new(Mutex::new(Services::new("test")));
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            "in-queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        
        
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            iterations, 
            "Task.recv-queue",
            services.clone(),
        )));
        
        let task = Arc::new(Mutex::new(Task::new("test", config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());
        
        let receiverHandle = receiver.lock().unwrap().run().unwrap();
        let producerHandle = producer.lock().unwrap().run().unwrap();
        trace!("task runing...");
        let time = Instant::now();
        task.lock().unwrap().run().unwrap();
        trace!("task runing - ok");
        producerHandle.join().unwrap();
        receiverHandle.join().unwrap();
        let sent = producer.lock().unwrap().sent().lock().unwrap().len();
        let result = receiver.lock().unwrap().received().lock().unwrap().len();
        println!(" elapsed: {:?}", time.elapsed());
        println!("    sent: {:?}", sent);
        println!("received: {:?}", result);
        assert!(sent == iterations, "\nresult: {:?}\ntarget: {:?}", sent, iterations);
        assert!(result == iterations, "\nresult: {:?}\ntarget: {:?}", result, iterations);
    }


    #[ignore = "TODO - transfered values asertion not implemented yet"]
    #[test]
    fn test_task_tranfer() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_transfer");
        
        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_struct.yaml";
        // let path = "./src/tests/unit/task/task_test.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
    
        let services = Arc::new(Mutex::new(Services::new("test")));
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            "in-queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            iterations, 
            "Task.recv-queue",
            services.clone(),
        )));
    
        let task = Arc::new(Mutex::new(Task::new("test", config, services.clone())));
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


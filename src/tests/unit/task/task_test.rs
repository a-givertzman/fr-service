#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{trace, info};
    use std::{sync::{Once, mpsc::{Sender, Receiver, self}}, env, time::Instant};
    
    use crate::{
        core_::{conf::task_config::TaskConfig, debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::point_type::PointType}, 
        services::{task::{task::Task, task_test_receiver::TaskTestReceiver, task_test_producer::TaskTestProducer}, queues::queues::Queues},
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
        
        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/task/task_test_struct.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
    
        let mut queues = Queues::new();
        let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        let (apiSend, apiRecv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        queues.addRecvQueue("recv-queue", recv);
        queues.addSendQueue("api-queue", apiSend);
    
        let mut receiver = TaskTestReceiver::new();
        
        let testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];
        receiver.run(apiRecv, iterations, testValues);
    
        let mut producer = TaskTestProducer::new(iterations, send);
        producer.run();
    
        let mut task = Task::new(config, queues);
        trace!("task tuning...");
        let time = Instant::now();
        task.run();
        trace!("task tuning - ok");
        producer.join();
        receiver.join();
        // thread::sleep(Duration::from_millis(200));
        trace!("task stopping...");
        task.exit();
        receiver.exit();
        trace!("task stopping - ok");
        println!("elapsed: {:?}", time.elapsed());
        println!("received: {:?}", receiver.received());
    
        // trace!("task: {:?}", &task);
        // assert_eq!(config, target);
    }


    // #[test]
    fn test_task_tranfer() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        info!("test_task_transfer");
        
        let iterations = 10;
        
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/task/task_test.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
    
        let mut queues = Queues::new();
        let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        let (apiSend, apiRecv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
        queues.addRecvQueue("recv-queue", recv);
        queues.addSendQueue("api-queue", apiSend);
    
        let mut receiver = TaskTestReceiver::new();
        
        let testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];
        receiver.run(apiRecv, iterations, testValues);
    
        let mut producer = TaskTestProducer::new(iterations, send);
        producer.run();
    
        let mut task = Task::new(config, queues);
        trace!("task tuning...");
        let time = Instant::now();
        task.run();
        trace!("task tuning - ok");
        producer.join();
        receiver.join();
        // thread::sleep(Duration::from_millis(200));
        trace!("task stopping...");
        task.exit();
        receiver.exit();
        trace!("task stopping - ok");
        println!("elapsed: {:?}", time.elapsed());
        println!("received: {:?}", receiver.received());
    
        // trace!("task: {:?}", &task);
        // assert_eq!(config, target);
    }
}


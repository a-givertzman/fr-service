#![allow(non_snake_case)]
// #[cfg(test)]
// mod tests;
mod core_;
mod conf;
mod services;
mod tcp;

use log::{trace, info, debug};
use std::{sync::{Once, mpsc::{Sender, Receiver, self}}, env, time::{Instant, Duration}, fs, thread};


use crate::{
    core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::point_type::PointType}, 
    conf::task_config::TaskConfig, 
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

// fn boxQueueSend(input: QueueSendMpscChannel<PointType>) -> Box<dyn QueueSend<String>> {
//     Box::new(input)
// }


fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_task");
    
    let producers = 3;
    let iterations = 10;
    
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/task_test.yaml";
    assert!(fs::metadata(path).is_ok());
    let config = TaskConfig::read(path);
    for confNode in config.nodes.clone() {
        debug!("config node: {:?}", &confNode);
    }
    // debug!("config: {:?}", &config);

    let mut queues = Queues::new();
    let (send1, recv1): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
    let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
    let (apiSend, apiRecv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
    queues.addRecvQueue("recv-queue", recv);
    queues.addSendQueue("api-queue", apiSend);
    let mut receiver = TaskTestReceiver::new();
    
    let testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];
    receiver.run(apiRecv, iterations * producers, testValues);

    let mut producer1 = TaskTestProducer::new(iterations, vec![send1.clone(), send.clone()]);
    let mut producer2 = TaskTestProducer::new(iterations, vec![send1.clone(), send.clone()]);
    let mut producer3 = TaskTestProducer::new(iterations, vec![send1, send]);
    producer1.run();
    producer2.run();
    producer3.run();

    let mut task = Task::new(config, queues);
    info!("task runing...");
    let time = Instant::now();
    task.run();
    info!("task runing - ok");
    println!("elapsed: {:?}", time.elapsed());
    println!("received: {:?}", receiver.received());
    let resRecv = receiver.getInputValues();
    let mut count = 1;
    while count < iterations * producers {
        match recv1.recv() {
            Ok(point) => {
                match resRecv.recv() {
                    Ok(result) => {
                        match point {
                            PointType::Bool(point) => {
                            },
                            PointType::Int(point) => {
                                
                            },
                            PointType::Float(point) => {                
                                count += 1;
                                let value = point.value;
                                let target = format!(
                                    "insert into {} (id, value, timestamp) values ({}, {}, {});", 
                                    "table_name", 
                                    "sqlSelectMetric", 
                                    point.value + 0.2 + 0.05,
                                    point.value + 2.224,
                                );
                                let result = result.asString().value;
                                debug!("count: {}\ntarget: {}\nresult: {}", count, target, result);
                                assert_eq!(result, target);
                            },
                            PointType::String(point) => {
                
                            },
                        }
                    },
                    Err(_) => {},
                };
            },
            Err(_) => {},
        };
    }
    thread::sleep(Duration::from_millis(300));
    assert_eq!(receiver.received(), iterations * producers);
    assert_eq!(count, iterations * producers);
    // producer1.join();
    // producer2.join();
    // producer3.join();
    // receiver.join();
    // thread::sleep(Duration::from_millis(200));
    trace!("task stopping...");
    task.exit();
    receiver.exit();
    trace!("task stopping - ok");
        
}


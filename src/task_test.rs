#![allow(non_snake_case)]
// #[cfg(test)]
// mod tests;
mod core_;
mod services;

use log::{trace, info};
use std::{sync::{Once, mpsc::{Sender, Receiver, self}}, env, time::Instant};


use crate::{core_::{conf::task_config::TaskConfig, debug::debug_session::{DebugSession, LogLevel, Backtrace}, point::point_type::PointType}, services::{task::{task::Task, task_test_receiver::TaskTestReceiver, task_test_producer::TaskTestProducer}, queues::queues::Queues}};

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
    DebugSession::init(LogLevel::Info, Backtrace::Short);
    initOnce();
    initEach();
    info!("test_task");
    
    let producers = 3;
    let iterations = 10_000;
    
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
    receiver.run(apiRecv, iterations * producers, testValues);

    let mut producer1 = TaskTestProducer::new(iterations, send.clone());
    let mut producer2 = TaskTestProducer::new(iterations, send.clone());
    let mut producer3 = TaskTestProducer::new(iterations, send);
    producer1.run();
    producer2.run();
    producer3.run();

    let mut task = Task::new(config, queues);
    trace!("task tuning...");
    let time = Instant::now();
    task.run();
    trace!("task tuning - ok");
    producer1.join();
    producer2.join();
    producer3.join();
    receiver.join();
    // thread::sleep(Duration::from_millis(200));
    trace!("task stopping...");
    task.exit();
    receiver.exit();
    trace!("task stopping - ok");
    println!("elapsed: {:?}", time.elapsed());
    println!("received: {:?}", receiver.received());
    assert_eq!(receiver.received(), iterations * producers);
}


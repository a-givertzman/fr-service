#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, info};
use std::{sync::{Once, mpsc::{Sender, Receiver, self}}, env, thread, time::{Duration, Instant}};

use crate::{core_::{conf::task_config::TaskConfig, debug::debug_session::{DebugSession, LogLevel}, point::point_type::PointType}, services::task::{task::Task, queue_send_mpsc_channel::QueueSendMpscChannel, queue_send::QueueSend}, tests::unit::task::{task_test_receiver::TaskTestReceiver, task_test_producer::TaskTestProducer}};

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


#[test]
fn test_task() {
    DebugSession::init(LogLevel::Debug);
    initOnce();
    initEach();
    info!("test_task");
    
    let iterations = 10;
    
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/tests/unit/task/task_test.yaml";
    let config = TaskConfig::read(path);
    trace!("config: {:?}", &config);

    let (send, recv): (Sender<PointType>, Receiver<PointType>) = mpsc::channel();
    let (apiSend, apiRecv): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut receiver = TaskTestReceiver::new();
    
    let testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];
    receiver.run(apiRecv, testValues);

    let mut producer = TaskTestProducer::new(iterations, send);
    producer.run();

    let mut task = Task::new(config, apiSend, recv);
    trace!("task tuning...");
    let time = Instant::now();
    task.run();
    trace!("task tuning - ok");
    producer.join();
    thread::sleep(Duration::from_millis(100));
    trace!("task stopping...");
    task.exit();
    receiver.exit();
    trace!("task stopping - ok");
    println!("elapsed: {:?}", time.elapsed());
    info!("Received points: {}", receiver.received());

    // trace!("task: {:?}", &task);
    // assert_eq!(config, target);
}


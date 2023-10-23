#![allow(non_snake_case)]
#[cfg(test)]
use log::{trace, info};
use std::{sync::{Once, mpsc::{Sender, Receiver, self}}, env, thread, time::Duration};

use crate::{core_::{conf::task_config::TaskConfig, debug::debug_session::{DebugSession, LogLevel}, point::point_type::PointType}, services::task::{task::Task, queue_send_mpsc_channel::QueueSendMpscChannel, queue_send::QueueSend}, tests::unit::task::task_test_receiver::TaskTestReceiver};

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
    
    // let (initial, switches) = initEach();
    trace!("dir: {:?}", env::current_dir());
    let path = "./src/tests/unit/task/task_test.yaml";
    let config = TaskConfig::read(path);
    trace!("config: {:?}", &config);

    let (send, recv): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut receiver = TaskTestReceiver::new();
    
    let testValues = vec![0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.0];
    receiver.run(recv, testValues);

    let mut task = Task::new(config, send);
    trace!("task tuning...");
    task.run();
    trace!("task tuning - ok");
    thread::sleep(Duration::from_secs_f32(5.0));
    trace!("task stopping...");
    task.exit();
    receiver.exit();
    trace!("task stopping - ok");
    // trace!("task: {:?}", &task);
    // assert_eq!(config, target);
}


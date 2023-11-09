#![allow(non_snake_case)]

use std::sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}};

use log::{info, debug};

use crate::core_::point::point_type::PointType;

///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
pub struct ApiClient {
    id: String,
    inQueue: Receiver<PointType>,
    send: Sender<PointType>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl ApiClient {
    ///
    /// 
    pub fn new(id: String) -> Self {        //, conf: ServiceCong
        let (send, recv) = mpsc::channel();
        Self {
            id,
            inQueue: recv,
            send: send,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn getLink(&self, _name: &str) -> Sender<PointType> {
        self.send.clone()
    }
    ///
    /// 
    pub fn run(&mut self) {
        info!("Task({}).run | starting...", self.id);
        let selfName = self.id.clone();
        let exit = self.exit.clone();
        let cycleInterval = self.cycle;
        let (cyclic, cycleInterval) = match cycleInterval {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let conf = self.conf.clone();
        let mut queues = self.queues.pop().unwrap();
        let recvQueue = queues.getRecvQueue(&self.conf.recvQueue);
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut cycle = TaskCycle::new(cycleInterval);
            let mut taskNodes = TaskNodes::new(&selfName);
            taskNodes.buildNodes(conf, &mut queues);
            debug!("Task({}).run | taskNodes: {:?}", selfName, taskNodes);
            'main: loop {
                cycle.start();
                trace!("Task({}).run | calculation step...", selfName);
                match recvQueue.recv() {
                    Ok(point) => {
                        debug!("Task({}).run | point: {:?}", selfName, &point);
                        taskNodes.eval(point);
                    },
                    Err(err) => {
                        warn!("Task({}).run | Error receiving from queue: {:?}", selfName, err);
                        break 'main;
                    },
                };
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                trace!("Task({}).run | calculation step - done ({:?})", selfName, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };
            info!("Task({}).run | stopped", selfName);
        }).unwrap();
        info!("Task({}).run | started", self.id);
        // h.join().unwrap();
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }

}
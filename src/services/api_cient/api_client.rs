#![allow(non_snake_case)]

use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, thread, collections::HashMap, net::TcpStream, io::Write};

use log::{info, debug, trace, warn};

use crate::{
    core_::{point::point_type::PointType, conf::api_client_config::ApiClientConfig}, 
    services::task::task_cycle::ServiceCycle, 
    tcp::tcp_socket_client_connect::TcpSocketClientConnect, 
};

use super::api_query::ApiQuery;

///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
pub struct ApiClient {
    id: String,
    recv: Vec<Receiver<PointType>>,
    send: HashMap<String, Sender<PointType>>,
    conf: ApiClientConfig,
    exit: Arc<AtomicBool>,
}
///
/// 
impl ApiClient {
    ///
    /// 
    pub fn new(id: impl Into<String>, conf: ApiClientConfig) -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            id: id.into(),
            recv: vec![recv],
            send: HashMap::from([(conf.recvQueue.clone(), send)]),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// returns sender of the ApiClient queue by name
    pub fn getLink(&self, name: &str) -> Sender<PointType> {
        match self.send.get(name) {
            Some(send) => send.clone(),
            None => panic!("ApiClient({}).run | link '{:?}' - not found", self.id, name),
        }
    }
    ///
    /// 
    fn readQueue(selfName: &str, recv: &Receiver<PointType>, buffer: &mut Vec<PointType>) {
        for _ in 0..1000 {                    
            match recv.recv() {
                Ok(point) => {
                    debug!("ApiClient({}).run | point: {:?}", selfName, &point);
                    buffer.push(point);
                },
                Err(_err) => {
                    break;
                    // warn!("ApiClient({}).run | Error receiving from queue: {:?}", selfName, err);
                },
            };
        }
    }
    ///
    /// Writing sql string to the TcpStream
    fn send(selfName: &str, sql: String, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>>{
        let query = ApiQuery::new("authToken", "id", "database", sql, true, true);
        match stream.write(query.toJson().as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => {
                warn!("ApiClient({}).run | write to tcp stream error: {:?}", selfName, err);
                Err(Box::new(err))
            },
        }
    }
    ///
    /// 
    pub fn run(&mut self) {
        info!("ApiClient({}).run | starting...", self.id);
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let recv = self.recv.pop().unwrap();
        let cycleInterval = conf.cycle;
        let (cyclic, cycleInterval) = match cycleInterval {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let reconnect = if conf.reconnectCycle.is_some() {conf.reconnectCycle.unwrap()} else {Duration::from_secs(3)};
        let _queueMaxLength = conf.recvQueueMaxLength;
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut buffer = Vec::new();
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut connect = TcpSocketClientConnect::new(selfId.clone() + "/TcpSocketClientConnect", conf.address);
            let mut stream = connect.connect(reconnect).unwrap();
            //  TcpStream::connect(conf.address).unwrap();
            'main: loop {
                cycle.start();
                trace!("ApiClient({}).run | step...", selfId);
                Self::readQueue(&selfId, &recv, &mut buffer);
                let mut count = buffer.len();
                while count > 0 {
                    match buffer.pop() {
                        Some(point) => {
                            let sql = point.asString().value;
                            match Self::send(&selfId, sql, &mut stream) {
                                Ok(_) => {},
                                Err(err) => {
                                    warn!("ApiClient({}).run | error sending API: {:?}", selfId, err);
                                },
                            }
                        },
                        None => {},
                    };
                    count -=1;
                }
                if exit.load(Ordering::SeqCst) {
                    break 'main;
                }
                trace!("ApiClient({}).run | step - done ({:?})", selfId, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };
            info!("ApiClient({}).run | stopped", selfId);
        }).unwrap();
        info!("ApiClient({}).run | started", self.id);
        // h.join().unwrap();
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
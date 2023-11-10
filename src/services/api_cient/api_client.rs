#![allow(non_snake_case)]

use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, thread, collections::{VecDeque, HashMap}, net::{TcpStream, SocketAddr}, io::Write};

use log::{info, debug, trace, warn};

use crate::{core_::{point::point_type::PointType, conf::api_client_config::ApiClientConfig}, services::task::task_cycle::ServiceCycle};

use super::api_query::ApiQuery;

///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
pub struct ApiClient {
    id: String,
    addres: SocketAddr,
    recv: Vec<Receiver<PointType>>,
    send: HashMap<String, Sender<PointType>>,
    conf: ApiClientConfig,
    cycle: Option<Duration>,
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
            addres: conf.address,
            recv: vec![recv],
            send: HashMap::from([(conf.recvQueue.clone(), send)]),
            conf: conf.clone(),
            cycle: conf.cycle.clone(),
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
        let selfName = self.id.clone();
        let exit = self.exit.clone();
        let cycleInterval = self.cycle;
        let (cyclic, cycleInterval) = match cycleInterval {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        let conf = self.conf.clone();
        let recv = self.recv.pop().unwrap();
        let _h = thread::Builder::new().name("name".to_owned()).spawn(move || {
            let mut buffer = Vec::new();
            let mut cycle = ServiceCycle::new(cycleInterval);
            let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();
            'main: loop {
                cycle.start();
                trace!("ApiClient({}).run | step...", selfName);
                Self::readQueue(&selfName, &recv, &mut buffer);
                let mut count = buffer.len();
                while count > 0 {
                    match buffer.pop() {
                        Some(point) => {
                            let sql = point.asString().value;
                            match Self::send(&selfName, sql, &mut stream) {
                                Ok(_) => {},
                                Err(err) => {
                                    warn!("ApiClient({}).run | error sending API: {:?}", selfName, err);
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
                trace!("ApiClient({}).run | step - done ({:?})", selfName, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };
            info!("ApiClient({}).run | stopped", selfName);
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
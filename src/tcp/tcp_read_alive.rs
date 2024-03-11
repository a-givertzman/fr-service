use concat_string::concat_string;
use log::{error, info, warn, LevelFilter};
use std::{
    io::BufReader, net::TcpStream, 
    sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc, Mutex}, 
    thread::{self, JoinHandle}, time::Duration,
};
use crate::{core_::{
    net::{connection_status::ConnectionStatus, protocols::jds::jds_deserialize::JdsDeserialize}, object::object::Object, point::point_type::PointType
}, services::{services::Services, task::service_cycle::ServiceCycle}};

use super::steam_read::TcpStreamRead;

///
/// Transfering points from JdsStream (socket) to the Channel Sender<PointType>
pub struct TcpReadAlive {
    id: String,
    stream_read: Arc<Mutex<dyn TcpStreamRead>>,
    send: Sender<PointType>,
    cycle: Duration,
    exit: Arc<AtomicBool>,
    exit_pair: Arc<AtomicBool>,
}
impl TcpReadAlive {
    ///
    /// Creates new instance of [TcpReadAlive]
    /// - [parent] - the ID if the parent entity
    /// - [exit] - notification from parent to exit 
    /// - [exitPair] - notification from / to sibling pair to exit 
    pub fn new(
        parent: impl Into<String>, 
        stream_read: Arc<Mutex<dyn TcpStreamRead>>,
        dest: Sender<PointType>, 
        cycle: Duration, 
        exit: Option<Arc<AtomicBool>>, 
        exit_pair: Option<Arc<AtomicBool>>
    ) -> Self {
        let self_id = format!("{}/TcpReadAlive", parent.into());
        Self {
            id: self_id.clone(),
            stream_read,
            send: dest,
            cycle,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exit_pair: exit_pair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Main loop of the [TcpReadAlive]
    pub fn run(&mut self, tcp_stream: TcpStream) -> JoinHandle<()> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let exit_pair = self.exit_pair.clone();
        let mut cycle = ServiceCycle::new(self.cycle);
        let send = self.send.clone();
        let jds_stream = self.stream_read.clone();
        info!("{}.run | Preparing thread...", self.id);
        let handle = thread::Builder::new().name(format!("{} - Read", self_id.clone())).spawn(move || {
            info!("{}.run | Preparing thread - ok", self_id);
            let mut tcp_stream = BufReader::new(tcp_stream);
            let mut jds_stream = jds_stream.lock().unwrap();
            info!("{}.run | Main loop started", self_id);
            loop {
                cycle.start();
                match jds_stream.read(&mut tcp_stream) {
                    ConnectionStatus::Active(point) => {
                        match point {
                            Ok(point) => {
                                match send.send(point) {
                                    Ok(_) => {},
                                    Err(err) => {
                                        warn!("{}.run | write to queue error: {:?}", self_id, err);
                                    },
                                };
                            },
                            Err(err) => {
                                if log::max_level() == LevelFilter::Trace {
                                    warn!("{}.run | error: {:?}", self_id, err);
                                }
                            },
                        }
                    },
                    ConnectionStatus::Closed(err) => {
                        warn!("{}.run | error: {:?}", self_id, err);
                        exit_pair.store(true, Ordering::SeqCst);
                        break;
                    },
                };
                if exit.load(Ordering::SeqCst) | exit_pair.load(Ordering::SeqCst) {
                    break;
                }
                cycle.wait();
            }
            info!("{}.run | Exit", self_id);
        }).unwrap();
        info!("{}.run | started", self.id);
        handle
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}

pub struct RouterReply {
    pass: Option<PointType>,
    retply: Option<PointType>,
}
impl RouterReply {
    pub fn new(pass: Option<PointType>, retply: Option<PointType>) -> Self {
        Self { pass, retply }
    }
}

pub struct JdsRoutes<F> {
    parent: String,
    id: String,
    services: Arc<Mutex<Services>>,
    jds_stream: JdsDeserialize,
    req_reply_send: Sender<PointType>,
    rautes: F ,
}
///
/// 
impl<F> JdsRoutes<F> {
    ///
    /// 
    pub fn new(parent: impl Into<String>, services: Arc<Mutex<Services>>, jds_stream: JdsDeserialize, req_reply_send: Sender<PointType>, rautes: F) -> Self {
        let parent = parent.into();
        let self_id = format!("{}/JdsRoutes", parent);
        Self {
            parent,
            id: self_id, 
            services,
            jds_stream,
            req_reply_send,
            rautes,
        }
    }
}
///
/// 
impl<F> Object for JdsRoutes<F> {
    fn id(&self) -> &str {
        &self.id
    }
}
///
/// 
impl<F> TcpStreamRead for JdsRoutes<F> where
    F: Fn(String, PointType, Arc<Mutex<Services>>) -> RouterReply,
    F: Send + Sync {
    ///
    /// Reads single point from source
    fn read(&mut self, tcp_stream: &mut BufReader<TcpStream>) -> ConnectionStatus<Result<PointType, String>, String> {
        match self.jds_stream.read(tcp_stream) {
            ConnectionStatus::Active(point) => {
                match point {
                    Ok(point) => {
                        let result = (&self.rautes)(self.parent.clone(), point, self.services.clone());
                        match result.retply {
                            Some(point) => if let Err(err) = self.req_reply_send.send(point) {
                                error!("{}.read | Send reply error: {:?}", self.id, err)
                            },
                            None => {},
                        };
                        match result.pass {
                            Some(point) => ConnectionStatus::Active(Ok(point)),
                            None => ConnectionStatus::Active(Err(concat_string!(self.id, ".read | Filtered by routes"))),
                        }
                    },
                    Err(err) => {
                        if log::max_level() == LevelFilter::Trace {
                            warn!("{}.read | error: {:?}", self.id, err);
                        }
                        ConnectionStatus::Active(Err(err))
                    },
                }
            },
            ConnectionStatus::Closed(err) => {
                warn!("{}.read | error: {:?}", self.id, err);
                ConnectionStatus::Closed(err)
            },
        }
    }
}
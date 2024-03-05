use concat_string::concat_string;
use log::{info, debug, trace, warn};
use std::{sync::{mpsc::{Receiver, Sender, self}, Arc, atomic::{AtomicBool, Ordering}}, time::Duration, thread::{self, JoinHandle}, collections::HashMap};
use api_tools::{api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryKind, ApiQuerySql}, api_request::ApiRequest}};
use crate::{
    core_::{point::point_type::PointType, retain_buffer::retain_buffer::RetainBuffer}, 
    conf::api_client_config::ApiClientConfig,
    services::{task::service_cycle::ServiceCycle, service::service::Service}, 
};

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
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, conf: ApiClientConfig) -> Self {
        let (send, recv) = mpsc::channel();
        let self_id = format!("{}/ApiClient({})", parent.into(), conf.name);
        Self {
            id: self_id,
            recv: vec![recv],
            send: HashMap::from([(conf.rx.clone(), send)]),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Reads all avalible at the moment items from the in-queue
    fn read_queue(self_id: &str, recv: &Receiver<PointType>, buffer: &mut RetainBuffer<PointType>) {
        let max_read_at_once = 1000;
        for (index, point) in recv.try_iter().enumerate() {   
            debug!("{}.readQueue | point: {:?}", self_id, &point);
            buffer.push(point);
            if index > max_read_at_once {
                break;
            }                 
        }
    }
    ///
    /// Writing sql string to the TcpStream
    fn send(self_id: &str, request: &mut ApiRequest, database: &str, sql: String, keep_alive: bool) -> Result<ApiReply, String> {
        let query = ApiQuery::new(
            ApiQueryKind::Sql(ApiQuerySql::new(database, sql)),
            true,
        );
        match request.fetch(&query, keep_alive) {
            Ok(reply) => {
                if log::max_level() > log::LevelFilter::Info {
                    let reply_str = std::str::from_utf8(&reply).unwrap();
                    debug!("{}.send | reply str: {:?}", self_id, reply_str);
                }
                match serde_json::from_slice(&reply) {
                    Ok(reply) => Ok(reply),
                    Err(err) => {
                        let reply = match std::str::from_utf8(&reply) {
                            Ok(reply) => reply.to_string(),
                            Err(err) => concat_string!(self_id, ".send | Error parsing reply to utf8 string: ", err.to_string()),
                        };
                        let message = concat_string!(self_id, ".send | Error parsing API reply: {:?} \n\t reply was: {:?}", err.to_string(), reply);
                        warn!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => {
                let message = concat_string!(self_id, ".send | Error sending API request: {:?}", err);
                warn!("{}", message);
                Err(message)
            },
        }
    }
}
///
/// 
impl Service for ApiClient {
    //
    //
    fn id(&self) -> &str {
        &self.id
    }
    //
    //
    fn get_link(&mut self, name: &str) -> Sender<PointType> {
        match self.send.get(name) {
            Some(send) => send.clone(),
            None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        }
    }
    //
    // 
    fn run(&mut self) -> Result<JoinHandle<()>, std::io::Error> {
        info!("{}.run | starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let recv = self.recv.pop().unwrap();
        let (cyclic, cycle_interval) = match conf.cycle {
            Some(interval) => (interval > Duration::ZERO, interval),
            None => (false, Duration::ZERO),
        };
        // let reconnect = if conf.reconnectCycle.is_some() {conf.reconnectCycle.unwrap()} else {Duration::from_secs(3)};
        let _queue_max_length = conf.rxMaxLength;
        let _handle = thread::Builder::new().name(format!("{} - main", self_id)).spawn(move || {
            let mut buffer = RetainBuffer::new(&self_id, "", Some(conf.rxMaxLength as usize));
            let mut cycle = ServiceCycle::new(cycle_interval);
            // let mut connect = TcpClientConnect::new(self_id.clone() + "/TcpSocketClientConnect", conf.address, reconnect);
            let api_keep_alive = true;
            let sql_keep_alive = true;
            let mut request = ApiRequest::new(
                &self_id, 
                conf.address, 
                conf.auth_token, 
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(&conf.database, "select 1;")), 
                    sql_keep_alive,
                ),
                api_keep_alive, 
                conf.debug,
            );
            'send: loop {
                cycle.start();
                trace!("{}.run | step...", self_id);
                Self::read_queue(&self_id, &recv, &mut buffer);
                let mut count = buffer.len();
                while count > 0 {
                    match buffer.first() {
                        Some(point) => {
                            let sql = point.as_string().value;
                            match Self::send(&self_id, &mut request, &conf.database, sql, api_keep_alive) {
                                Ok(reply) => {
                                    if reply.has_error() {
                                        warn!("{}.run | API reply has error: {:?}", self_id, reply.error);
                                    } else {
                                        buffer.popFirst();
                                    }
                                },
                                Err(err) => {
                                    warn!("{}.run | Error: {:?}", self_id, err);
                                },
                            }

                        },
                        None => {break;},
                    };
                    count -=1;
                }
                if exit.load(Ordering::SeqCst) {
                    break 'send;
                }
                trace!("{}.run | step - done ({:?})", self_id, cycle.elapsed());
                if cyclic {
                    cycle.wait();
                }
            };            
            // 'main: loop {
            //     match connect.connect() {
            //         Some(mut stream) => {
            //             match stream.set_read_timeout(Some(Duration::from_secs(10))) {
            //                 Ok(_) => {},
            //                 Err(err) => {
            //                     debug!("{}.run | TcpStream.set_timeout error: {:?}", self_id, err);
            //                 },
            //             };
            //             'send: loop {
            //                 cycle.start();
            //                 trace!("{}.run | step...", self_id);
            //                 Self::read_queue(&self_id, &recv, &mut buffer);
            //                 let mut count = buffer.len();
            //                 while count > 0 {
            //                     match buffer.first() {
            //                         Some(point) => {
            //                             let sql = point.as_string().value;
            //                             match Self::send(&self_id, &mut request, sql, api_keep_alive, &mut stream) {
            //                                 Ok(reply) => {
            //                                     if reply.hasError() {
            //                                         warn!("{}.run | API reply has error: {:?}", self_id, reply.error);
            //                                     } else {
            //                                         buffer.popFirst();
            //                                     }
            //                                 },
            //                                 Err(err) => {
            //                                     warn!("{}.run | Error: {:?}", self_id, err);
            //                                 },
            //                             }
            //                             // match Self::send(&self_id, sql, &mut stream) {
            //                             //     Ok(_) => {
            //                             //         match Self::read_all(&self_id, &mut stream) {
            //                             //             ConnectionStatus::Active(bytes) => {
            //                             //                 let reply = String::from_utf8(bytes).unwrap();
            //                             //                 debug!("{}.run | API reply: {:?}", self_id, reply);
            //                             //                 let reply: SqlReply = serde_json::from_str(&reply).unwrap();
            //                             //                 if reply.hasError() {
            //                             //                     warn!("{}.run | API reply has error: {:?}", self_id, reply.error);
            //                             //                 } else {
            //                             //                     buffer.popFirst();
            //                             //                 }
            //                             //             },
            //                             //             ConnectionStatus::Closed(err) => {
            //                             //                 warn!("{}.run | API read error: {:?}", self_id, err);
            //                             //                 break 'send;
            //                             //             },
            //                             //         };
            //                             //     },
            //                             //     Err(err) => {
            //                             //         warn!("{}.run | API sending error: {:?}", self_id, err);
            //                             //         break 'send;
            //                             //     },
            //                             // }
            //                         },
            //                         None => {break;},
            //                     };
            //                     count -=1;
            //                 }
            //                 if exit.load(Ordering::SeqCst) {
            //                     break 'main;
            //                 }
            //                 trace!("{}.run | step - done ({:?})", self_id, cycle.elapsed());
            //                 if cyclic {
            //                     cycle.wait();
            //                 }
            //             };
            //         },
            //         None => {
            //             debug!("{}.run | Not connection", self_id);
            //         },
            //     }
            //     if exit.load(Ordering::SeqCst) {
            //         break 'main;
            //     }
            //     thread::sleep(Duration::from_millis(100));
            // };
            info!("{}.run | stopped", self_id);
        });
        info!("{}.run | started", self.id);
        _handle
    }
    //
    // 
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
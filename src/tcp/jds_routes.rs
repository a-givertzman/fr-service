use std::{io::BufReader, net::TcpStream, sync::{mpsc::Sender, Arc, Mutex}};
use log::{error, warn, LevelFilter};
use crate::{
    core_::{
        object::object::Object, 
        point::point_type::PointType,
        net::{connection_status::ConnectionStatus, protocols::jds::jds_deserialize::JdsDeserialize}, 
    }, 
    services::services::Services,
};
use concat_string::concat_string;
use super::steam_read::TcpStreamRead;


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
use std::{io::BufReader, net::TcpStream, sync::{mpsc::Sender, Arc, Mutex, RwLock}};
use log::{error, warn, LevelFilter};
use crate::{
    core_::{
        net::{connection_status::ConnectionStatus, protocols::jds::jds_deserialize::JdsDeserialize}, object::object::Object, point::point_type::PointType 
    }, 
    services::{services::Services, server::tcp_server_cnnection::Shared}, tcp::steam_read::TcpStreamRead,
};
use concat_string::concat_string;


/// Container used in the JdsRoutes, contains of:
///     - pass - Point to be transmitted to the MultiQueue - returned from read method
///     - reply - Point to be sent back to the Client, contains reply in the value
#[derive(Debug)]
pub struct RouterReply {
    pass: Option<PointType>,
    retply: Option<PointType>,
}
impl RouterReply {
    pub fn new(pass: Option<PointType>, retply: Option<PointType>) -> Self {
        Self { pass, retply }
    }
}
///
/// Reaction on the JDS request. 
/// Implements behavior for different kind of JDS requests
/// - Incoming points reads from [jds_deserialize]
/// - Incoming points passed as a parameter into the [routes(parent, point, services)] calback function
/// - [routes] calback function returns RouterReply, which contains of:
///     - pass - Point to be transmitted to the MultiQueue - returned from read method
///     - reply - Point to be sent back to the Client, contains reply in the value
pub struct JdsRoutes<F> {
    parent: String,
    id: String,
    path: String,
    services: Arc<Mutex<Services>>,
    jds_deserialize: JdsDeserialize,
    req_reply_send: Sender<PointType>,
    rautes: F,
    shared: Arc<RwLock<Shared>>,
}
///
/// 
impl<F> JdsRoutes<F> {
    ///
    /// 
    pub fn new(
        parent: impl Into<String>, 
        path: impl Into<String>, 
        services: Arc<Mutex<Services>>, 
        jds_deserialize: JdsDeserialize, 
        req_reply_send: Sender<PointType>, 
        rautes: F, 
        shared: Arc<RwLock<Shared>>,
    ) -> Self {
        let parent = parent.into();
        let self_id = format!("{}/JdsRoutes", parent);
        let path = format!("{}/Jds", path.into());
        Self {
            parent,
            id: self_id, 
            path,
            services,
            jds_deserialize,
            req_reply_send,
            rautes,
            shared,
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
    //    parent, path
    F: Fn(String, String, PointType, Arc<Mutex<Services>>, Arc<RwLock<Shared>>) -> RouterReply,
    F: Send + Sync {
    ///
    /// Reads single point from source
    fn read(&mut self, tcp_stream: &mut BufReader<TcpStream>) -> ConnectionStatus<Result<PointType, String>, String> {
        match self.jds_deserialize.read(tcp_stream) {
            ConnectionStatus::Active(point) => {
                match point {
                    Ok(point) => {
                        let result = (self.rautes)(self.parent.clone(), self.path.clone(), point, self.services.clone(), self.shared.clone());
                        if let Some(point) = result.retply {
                            if let Err(err) = self.req_reply_send.send(point) {
                                error!("{}.read | Send reply error: {:?}", self.id, err)
                            }
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
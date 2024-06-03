use std::{collections::HashMap, fmt::Debug, hash::BuildHasherDefault, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread, time::Duration};
use hashers::fx_hash::FxHasher;
use indexmap::IndexMap;
use log::{debug, info, warn};
use testing::stuff::wait::WaitTread;
use crate::{
    conf::{diag_keywd::DiagKeywd, point_config::name::Name, slmp_client_config::slmp_client_config::SlmpClientConfig, tcp_client_config::TcpClientConfig}, core_::{net::protocols::jds::{jds_decode_message::JdsDecodeMessage, jds_deserialize::JdsDeserialize, jds_encode_message::JdsEncodeMessage, jds_serialize::JdsSerialize}, object::object::Object, point::{point_tx_id::PointTxId, point_type::PointType}, types::map::IndexMapFxHasher}, services::{diagnosis::diag_point::DiagPoint, safe_lock::SafeLock, service::{service::Service, service_handles::ServiceHandles}, services::Services, slmp_client::{slmp_db::SlmpDb, slmp_read::SlmpRead}}, tcp::{
        tcp_client_connect::TcpClientConnect, tcp_read_alive::TcpReadAlive, tcp_stream_write::TcpStreamWrite, tcp_write_alive::TcpWriteAlive
    } 
};
///
/// - Connects to the SLMP device (FX5 Eth module)
/// - Cyclically reads adressess from the SLMP device and yields changed to the MultiQueue
/// - Writes Point to the protocol (SLMP device) specific address
pub struct SlmpClient {
    tx_id: usize,
    id: String,
    name: Name,
    conf: SlmpClientConfig,
    services: Arc<Mutex<Services>>,
    tcp_recv_alive: Option<Arc<Mutex<TcpReadAlive>>>,
    tcp_send_alive: Option<Arc<Mutex<TcpWriteAlive>>>,
    diagnosis: Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
    exit: Arc<AtomicBool>,
}
//
// 
impl SlmpClient {
    ///
    /// Creates new instance of [ApiClient]
    /// - [parent] - the ID if the parent entity
    pub fn new(conf: SlmpClientConfig, services: Arc<Mutex<Services>>) -> Self {
        let tx_id = PointTxId::fromStr(&conf.name.join());
        let diagnosis = Arc::new(Mutex::new(conf.diagnosis.iter().map(|(keywd, conf)| {
            (keywd.to_owned(), DiagPoint::new(tx_id, conf.clone()))
        }).collect()));
        Self {
            tx_id,
            id: conf.name.join(),
            name: conf.name.clone(),
            conf: conf.clone(),
            services,
            tcp_recv_alive: None,
            tcp_send_alive: None,
            diagnosis,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
//
//
impl Object for SlmpClient {
    fn id(&self) -> &str {
        &self.id
    }
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl Debug for SlmpClient {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SlmpClient")
            .field("id", &self.id)
            .finish()
    }
}
//
// 
impl Service for SlmpClient {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let tx_send = self.services.slock().get_link(&self.conf.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self_id, err);
        });
        let conf = self.conf.clone();
        let exit = self.exit.clone();
        let exit_pair = Arc::new(AtomicBool::new(false));
        let tx_send = self.services.slock().get_link(&conf.send_to).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.id, err);
        });
        let tx_send = self.services.slock().get_link(&self.conf.tx).unwrap_or_else(|err| {
            panic!("{}.run | services.get_link error: {:#?}", self.id, err);
        });
        let read = SlmpRead::new()
        // Self::yield_diagnosis(&self.id, &self.diagnosis.clone(), &DiagKeywd::Status, Status::Ok, &tx_send);
        // Self::yield_diagnosis(&self.id, &self.diagnosis.clone(), &DiagKeywd::Connection, Status::Invalid, &tx_send);
        let handle_read = self.read(tx_send.clone());
        let handle_write = self.write(tx_send);
        info!("{}.run | started", self.id);
        match (handle_read, handle_write) {
            (Ok(handle_read), Ok(handle_write)) => {
                Ok(ServiceHandles::new(vec![
                    (format!("{}/read", self.id), handle_read),
                    (format!("{}/write", self.id), handle_write),
                ]))
            }
            (Ok(handle_read), Err(err)) => {
                self.exit();
                handle_read.wait().unwrap();
                Err(format!("{}.run | Error starting inner thread 'read': {:#?}", self.id, err))
            }
            (Err(err), Ok(handle_write)) => {
                self.exit();
                handle_write.wait().unwrap();
                Err(format!("{}.run | Error starting inner thread 'write': {:#?}", self.id, err))
            }
            (Err(read_err), Err(write_err)) => {
                Err(format!("{}.run | Error starting inner thread: \n\t  read: {:#?}\n\t write: {:#?}", self.id, read_err, write_err))
            }
        }

    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        match &self.tcp_recv_alive {
            Some(tcp_recv_alive) => {
                tcp_recv_alive.slock().exit()
            }
            None => {}
        }
        match &self.tcp_send_alive {
            Some(tcp_send_alive) => {
                tcp_send_alive.slock().exit()
            }
            None => {}
        }
    }
}

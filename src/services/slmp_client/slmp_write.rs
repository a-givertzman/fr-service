use std::{net::TcpStream, sync::{atomic::AtomicBool, mpsc::Sender, Arc, Mutex}, thread::JoinHandle};
use indexmap::IndexMap;
use log::{debug, error, info, trace, warn};
use crate::{
    conf::{diag_keywd::DiagKeywd, point_config::name::Name, slmp_client_config::{slmp_client_config::SlmpClientConfig, slmp_db_config::SlmpDbConfig}},
    core_::{
        point::point_type::PointType, status::status::Status, types::map::IndexMapFxHasher,
    },
    services::{diagnosis::diag_point::DiagPoint, slmp_client::{parse_point::ParsePoint, slmp_db::SlmpDb}},
};
/// 
/// Cyclicaly reads SLMP data ranges (DB's) specified in the [conf]
/// - exit - external signal to stop the main read cicle and exit the thread
/// - exit_pair - exit signal from / to notify 'Write' partner to exit the thread
pub struct SlmpWrite {
    tx_id: usize,
    id: String,
    name: Name,
    conf: SlmpClientConfig,
    dest: Sender<PointType>, 
    diagnosis: Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
    exit: Arc<AtomicBool>,
    exit_pair: Arc<AtomicBool>,
}
impl SlmpWrite {
    ///
    /// Creates new instance of the SlpmRead
    pub fn new(
        parent: impl Into<String>,
        tx_id: usize,
        name: Name,
        conf: SlmpClientConfig, 
        dest: Sender<PointType>,
        diagnosis: Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
        exit: Option<Arc<AtomicBool>>,
        exit_pair: Option<Arc<AtomicBool>>,
    ) -> Self {
        Self {
            tx_id,
            id: format!("{}/SlmpWrite", parent.into()),
            name,
            conf,
            dest,
            diagnosis,
            exit: exit.unwrap_or(Arc::new(AtomicBool::new(false))),
            exit_pair: exit_pair.unwrap_or(Arc::new(AtomicBool::new(false))),
        }
    }
    ///
    /// Sends diagnosis point
    fn yield_diagnosis(
        self_id: &str,
        diagnosis: &Arc<Mutex<IndexMapFxHasher<DiagKeywd, DiagPoint>>>,
        kewd: &DiagKeywd,
        value: Status,
        tx_send: &Sender<PointType>,
    ) {
        match diagnosis.lock() {
            Ok(mut diagnosis) => {
                match diagnosis.get_mut(kewd) {
                    Some(point) => {
                        debug!("{}.yield_diagnosis | Sending diagnosis point '{}' ", self_id, kewd);
                        if let Some(point) = point.next(value) {
                            if let Err(err) = tx_send.send(point) {
                                warn!("{}.yield_status | Send error: {}", self_id, err);
                            }
                        }
                    }
                    None => debug!("{}.yield_diagnosis | Diagnosis point '{}' - not configured", self_id, kewd),
                }
            }
            Err(err) => error!("{}.yield_diagnosis | Diagnosis lock error: {:#?}", self_id, err),
        }
    }
    ///
    /// Sends all configured points from the current DB with the given status
    fn yield_status(self_id: &str, status: Status, dbs: &mut IndexMapFxHasher<String, SlmpDb>, tx_send: &Sender<PointType>) {
        for (db_name, db) in dbs {
            debug!("{}.yield_status | DB '{}' - sending Invalid status...", self_id, db_name);
            match db.yield_status(status, tx_send) {
                Ok(_) => {}
                Err(err) => {
                    error!("{}.yield_status | send errors: \n\t{:?}", self_id, err);
                }
            };
        }
    }
    ///
    /// Writes point to the current DB
    ///     - Returns Ok() if succeed, Err(message) on fail
    pub fn run(&mut self, tcp_stream: TcpStream) -> Result<JoinHandle<()>, std::io::Error> {
        let mut message = String::new();
        match self.points.get(&point.name()) {
            Some(_parse_point) => {
                let bytes = match point {
                    PointType::Bool(point) => {
                        // !!! Not implemented because before write byte of the bool bits, that byte must be read from device
                        // let mut buf = [0; 16];
                        // let index = address.offset.unwrap() as usize;
                        // buf[index] = point.value.0 as u8;
                        // client.write(self.number, address.offset.unwrap(), 2, &mut buf)
                        message = format!("{}.write | Write 'Bool' to the Device - not implemented, point: {:?}", self.id, point.name);
                        Err(message)
                    }
                    PointType::Int(point) => {
                        match i16::try_from(point.value) {
                            Ok(value) => {
                                let write_data = value.to_le_bytes();
                                match self.slmp_packet.write_packet(FrameType::BinReqSt, &write_data) {
                                    Ok(write_packet) => Ok(write_packet),
                                    Err(err) => Err(err),
                                }
                            }
                            Err(err) => {
                                message = format!("{}.write | Type 'Int' to i16 conversion error: {:#?} in the point: {:#?}", self.id, err, point.name);
                                Err(message)
                            }
                        }
                    }
                    PointType::Real(point) => {
                        let write_data = point.value.to_le_bytes();
                        match self.slmp_packet.write_packet(FrameType::BinReqSt, &write_data) {
                            Ok(write_packet) => Ok(write_packet),
                            Err(err) => Err(err),
                        }
                    }
                    PointType::Double(point) => {
                        message = format!("{}.write | Write 'Double' to the Device - not implemented, point: {:?}", self.id, point.name);
                        Err(message)
                    }
                    PointType::String(point) => {
                        message = format!("{}.write | Write 'String' to the Device - not implemented, point: {:?}", self.id, point.name);
                        Err(message)
                    }
                };
                match bytes {
                    Ok(bytes) => {
                        match tcp_stream.write_all(&bytes) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(format!("{}.write | Write to socket error: {:#?}", self.id, err)),
                        }
                    }
                    Err(err) => Err(err),
                }

            }
            None => {
                Err(message)
            }
        }
    }
}
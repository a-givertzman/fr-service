use std::io::{Read, ErrorKind};
use log::{warn, trace};
use crate::core_::net::{connection_status::ConnectionStatus, protocols::jds::jds_define::JDS_END_OF_TRANSMISSION};
///
/// 
enum Status {
    Active,
    Closed,
}
///
/// Reads bytes from TcpStream
/// splits bytes sequence with Jds.endOfTransmission = 4 separator
/// returns Result<Vec, Err>
pub struct JdsDecodeMessage {
    id: String,
    // tcpStream: BufReader<TcpStream>,
    remainder: Vec<u8>,
}
///
/// 
impl JdsDecodeMessage {
    ///
    /// Creates new instance of the JdsDecodeMessage
    pub fn new(parent: impl Into<String>) -> Self {
        Self {
            id: format!("{}/JdsDecodeMessage", parent.into()),
            // tcpStream: BufReader::new(tcpStream),
            remainder: Vec::new(),
        }
    }
    ///
    /// Reads sequence of bytes from TcpStream
    pub fn read(&mut self, tcp_stream: impl Read) -> ConnectionStatus<Result<Vec<u8>, String>, String> {
        let mut bytes = self.remainder.clone();
        match Self::read_all(&self.id, &mut bytes, tcp_stream) {
            ConnectionStatus::Active(result) => {
                match result {
                    Ok(_) => {
                        self.remainder.clear();
                        ConnectionStatus::Active(Ok(bytes))
                    },
                    Err(err) => ConnectionStatus::Active(Err(err)),
                }
            },
            ConnectionStatus::Closed(err) => {
                if !bytes.is_empty() {
                    self.remainder = bytes;
                }
                ConnectionStatus::Closed(err)
            },
        }
    }
    ///
    /// bytes will be read from socket until JDS_END_OF_TRANSMISSION = 4
    /// - returns Active: if read bytes non zero length without errors
    /// - returns Closed:
    ///    - if read 0 bytes
    ///    - if on error
    fn read_all(self_id: &str, bytes: &mut Vec<u8>, stream: impl Read) -> ConnectionStatus<Result<(), String>, String> {
        for byte in stream.bytes() {
            match byte {
                Ok(byte) => {
                    // debug!("{}.read_all |     read len: {:?}", self_id, len);
                    match byte {
                        JDS_END_OF_TRANSMISSION => {
                            return ConnectionStatus::Active(Ok(()));
                        },
                        _ => {
                            bytes.push(byte);
                        },
                    };
                },
                Err(err) => {
                    warn!("{}.read_all | error reading from socket: {:?}", self_id, err);
                    warn!("{}.read_all | error kind: {:?}", self_id, err.kind());
                    match Self::match_error_kind(err.kind()) {
                        Status::Active => {
                            return ConnectionStatus::Active(Err(format!("{}.read_all | tcp stream is empty", self_id)));
                        },
                        Status::Closed => {
                            return ConnectionStatus::Closed(format!("{}.read_all | tcp stream is closed, error: {:?}", self_id, err));
                        },
                    }
                },
            };
        };
        trace!("{}.read_all | read bytes: {:?}", self_id, bytes);
        ConnectionStatus::Closed(format!("{}.read_all | tcp stream is closed", self_id))
    }
    ///
    /// 
    fn match_error_kind(kind: ErrorKind) -> Status {
        match kind {
            std::io::ErrorKind::NotFound => todo!(),
            std::io::ErrorKind::PermissionDenied => Status::Closed,
            std::io::ErrorKind::ConnectionRefused => Status::Closed,
            std::io::ErrorKind::ConnectionReset => Status::Closed,
            // std::io::ErrorKind::HostUnreachable => Status::Closed,
            // std::io::ErrorKind::NetworkUnreachable => Status::Closed,
            std::io::ErrorKind::ConnectionAborted => Status::Closed,
            std::io::ErrorKind::NotConnected => Status::Closed,
            std::io::ErrorKind::AddrInUse => Status::Closed,
            std::io::ErrorKind::AddrNotAvailable => Status::Closed,
            // std::io::ErrorKind::NetworkDown => Status::Closed,
            std::io::ErrorKind::BrokenPipe => Status::Closed,
            std::io::ErrorKind::AlreadyExists => todo!(),
            std::io::ErrorKind::WouldBlock => Status::Active,
            // std::io::ErrorKind::NotADirectory => todo!(),
            // std::io::ErrorKind::IsADirectory => todo!(),
            // std::io::ErrorKind::DirectoryNotEmpty => todo!(),
            // std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
            // std::io::ErrorKind::FilesystemLoop => todo!(),
            // std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
            std::io::ErrorKind::InvalidInput => todo!(),
            std::io::ErrorKind::InvalidData => todo!(),
            std::io::ErrorKind::TimedOut => todo!(),
            std::io::ErrorKind::WriteZero => todo!(),
            // std::io::ErrorKind::StorageFull => todo!(),
            // std::io::ErrorKind::NotSeekable => todo!(),
            // std::io::ErrorKind::FilesystemQuotaExceeded => todo!(),
            // std::io::ErrorKind::FileTooLarge => todo!(),
            // std::io::ErrorKind::ResourceBusy => todo!(),
            // std::io::ErrorKind::ExecutableFileBusy => todo!(),
            // std::io::ErrorKind::Deadlock => todo!(),
            // std::io::ErrorKind::CrossesDevices => todo!(),
            // std::io::ErrorKind::TooManyLinks => todo!(),
            // std::io::ErrorKind::InvalidFilename => todo!(),
            // std::io::ErrorKind::ArgumentListTooLong => todo!(),
            std::io::ErrorKind::Interrupted => todo!(),
            std::io::ErrorKind::Unsupported => todo!(),
            std::io::ErrorKind::UnexpectedEof => todo!(),
            std::io::ErrorKind::OutOfMemory => todo!(),
            std::io::ErrorKind::Other => todo!(),
            _ => Status::Closed,
        }
    }
}

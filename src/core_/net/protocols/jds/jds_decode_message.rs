#![allow(non_snake_case)]

use std::{net::TcpStream, io::{Read, BufReader, ErrorKind}};

use log::warn;

use crate::core_::net::connection_status::ConnectionStatus;


pub const JDS_END_OF_TRANSMISSION: u8 = 0x4;

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
    stream: BufReader<TcpStream>,
    buffer: Vec<u8>,
}
///
/// 
impl JdsDecodeMessage {
    ///
    /// Creates new instance of the JdsMessage
    pub fn new(parent: impl Into<String>, stream: TcpStream) -> Self {
        Self {
            id: format!("{}/JdsMessage", parent.into()),
            stream: BufReader::new(stream),
            buffer: Vec::new(),
        }
    }
    ///
    /// Reads sequence of bytes from TcpStream
    pub fn read(&mut self) -> ConnectionStatus<Vec<u8>> {
        let mut bytes = self.buffer.clone();
        match Self::readAll(&self.id, &mut bytes, &mut self.stream) {
            Status::Active => {
                self.buffer.clear();
                ConnectionStatus::Active(bytes)
            },
            Status::Closed => {
                if !bytes.is_empty() {
                    self.buffer = bytes;
                }
                ConnectionStatus::Closed
            },
        }
    }
    ///
    /// bytes will be read from socket until JDS_END_OF_TRANSMISSION = 4
    /// - returns Active: if read bytes non zero length without errors
    /// - returns Closed:
    ///    - if read 0 bytes
    ///    - if on error
    fn readAll(selfId: &str, bytes: &mut Vec<u8>, stream: &mut BufReader<TcpStream>) -> Status {
        for byte in stream.bytes() {
            match byte {
                Ok(byte) => {
                    // debug!("{}.readAll |     read len: {:?}", selfId, len);
                    match byte {
                        JDS_END_OF_TRANSMISSION => {
                            return Status::Active;
                        },
                        _ => {
                            bytes.push(byte);
                        },
                    };
                },
                Err(err) => {
                    warn!("{}.readAll | error reading from socket: {:?}", selfId, err);
                    warn!("{}.readAll | error kind: {:?}", selfId, err.kind());
                    return Self::matchErrorKind(err.kind())
                },
            };
        };
        Status::Closed
    }
    ///
    /// 
    fn matchErrorKind(kind: ErrorKind) -> Status {
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
            std::io::ErrorKind::WouldBlock => Status::Closed,
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
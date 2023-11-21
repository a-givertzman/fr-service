#![allow(non_snake_case)]

use std::{net::TcpStream, io::Write};

use log::{warn, LevelFilter, debug};

use crate::{tcp::steam_read::StreamRead, core_::retain_buffer::retain_buffer::RetainBuffer};

///
/// Received from in queue sequences of bites adds into the end of local buffer
/// Sends sequences of bites from the beginning of the buffer
/// Sent sequences of bites immediately removed from the buffer
/// Buffering - is optional
pub struct TcpStreamWrite {
    id: String,
    buffered: bool,
    stream: Box<dyn StreamRead<Vec<u8>, String> + Send>,
    buffer: RetainBuffer<Vec<u8>>,
}
///
/// 
impl TcpStreamWrite {
    ///
    /// Creates new instance of [TcpStreamWrite]
    pub fn new(parent: impl Into<String>, buffered: bool, bufferLength: Option<usize>, stream: Box<dyn StreamRead<Vec<u8>, String> + Send>) -> Self {
        let selfId = format!("{}/TcpStreamWrite", parent.into());
        let buffer = match buffered {
            true => RetainBuffer::new(&selfId, "", bufferLength),
            false => RetainBuffer::new(&selfId, "", Some(0))
        };
        Self {
            id: selfId,
            buffered,
            stream,
            buffer,
        }
    }
    ///
    /// 
    pub fn write(&mut self, tcpStream: &mut TcpStream) -> Result<usize, String> {
        match self.stream.read() {
            Ok(bytes) => {
                while let Some(bytes) = self.buffer.first() {
                    debug!("{}.write | bytes: {:?}", self.id, bytes);
                    match tcpStream.write(&bytes) {
                        Ok(_) => {
                            self.buffer.popFirst();
                        },
                        Err(err) => {
                            let message = format!("{}.write | error: {:?}", self.id, err);
                            if log::max_level() == LevelFilter::Trace {
                                warn!("{}", message);
                            }
                            return Err(message);
                        },
                    };
                }
                debug!("{}.write | bytes: {:?}", self.id, bytes);
                match tcpStream.write(&bytes) {
                    Ok(sent) => {Ok(sent)},
                    Err(err) => {
                        self.buffer.push(bytes);
                        let message = format!("{}.write | error: {:?}", self.id, err);
                        if log::max_level() == LevelFilter::Trace {
                            warn!("{}", message);
                        }
                        return Err(message);

                    },
                }
            },
            Err(err) => {
                let message = format!("{}.write | error: {:?}", self.id, err);
                if log::max_level() == LevelFilter::Trace {
                    warn!("{}", message);
                }
                Err(message)
            },
        }
    }
}
///
/// 
unsafe impl Sync for TcpStreamWrite {}

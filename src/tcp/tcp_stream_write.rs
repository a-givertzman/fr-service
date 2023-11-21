#![allow(non_snake_case)]

use std::{net::TcpStream, io::Write};

use log::{warn, LevelFilter};

use crate::{tcp::steam_read::StreamRead, core_::retain_buffer::retain_buffer::RetainBuffer};

///
/// Received from in queue sequences of bites adds into the end of local buffer
/// Sends sequences of bites from the beginning of the buffer
/// Sent sequences of bites immediately removed from the buffer
/// Buffering - is optional
pub struct TcpStreamWrite {
    id: String,
    buffered: bool,
    stream: Box<dyn StreamRead<Vec<u8>, String>>,
    buffer: RetainBuffer<Vec<u8>>,
}
///
/// 
impl TcpStreamWrite {
    ///
    /// Creates new instance of [TcpStreamWrite]
    pub fn new(parent: impl Into<String>, buffered: bool, bufferLength: Option<usize>, stream: Box<dyn StreamRead<Vec<u8>, String>>) -> Self {
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
    pub fn write(&mut self, tcpStream: &mut TcpStream) -> Result<(), String> {
        match self.stream.read() {
            Ok(bytes) => {
                while self.buffer.len() > 0 {
                    match self.buffer.first() {
                        Some(bytes) => {
                            match tcpStream.write(&bytes) {
                                Ok(_) => {
                                    self.buffer.remove(0);
                                },
                                Err(err) => {
                                    let message = format!("{}.write | error: {:?}", self.id, err);
                                    if log::max_level() == LevelFilter::Trace {
                                        warn!("{}", message);
                                    }
                                    return Err(message);
                                },
                            }
                        },
                        None => {},
                    }
                }
                match tcpStream.write(&bytes) {
                    Ok(_) => {Ok(())},
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
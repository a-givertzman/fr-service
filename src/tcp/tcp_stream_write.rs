use std::io::Write;
use log::{warn, LevelFilter, trace};
use crate::{
    tcp::steam_read::StreamRead, 
    core_::{retain_buffer::retain_buffer::RetainBuffer, net::connection_status::ConnectionStatus, failure::recv_error::RecvError},
};

///
/// Received from in queue sequences of bites adds into the end of local buffer
/// Sends sequences of bites from the beginning of the buffer
/// Sent sequences of bites immediately removed from the buffer
/// Buffering - is optional
pub struct TcpStreamWrite {
    id: String,
    // buffered: bool,
    stream: Box<dyn StreamRead<Vec<u8>, RecvError> + Send>,
    buffer: RetainBuffer<Vec<u8>>,
}
///
/// 
impl TcpStreamWrite {
    ///
    /// Creates new instance of [TcpStreamWrite]
    pub fn new(parent: impl Into<String>, buffered: bool, buffer_length: Option<usize>, stream: Box<dyn StreamRead<Vec<u8>, RecvError> + Send>) -> Self {
        let self_id = format!("{}/TcpStreamWrite", parent.into());
        let buffer = match buffered {
            true => RetainBuffer::new(&self_id, "", buffer_length),
            false => RetainBuffer::new(&self_id, "", Some(0))
        };
        Self {
            id: self_id,
            // buffered,
            stream,
            buffer,
        }
    }
    ///
    /// 
    pub fn write(&mut self, mut tcp_stream: impl Write) -> ConnectionStatus<Result<usize, String>, String> {
        match self.stream.read() {
            Ok(bytes) => {
                while let Some(bytes) = self.buffer.first() {
                    trace!("{}.write | bytes: {:?}", self.id, bytes);
                    match tcp_stream.write(&bytes) {
                        Ok(_) => {
                            self.buffer.popFirst();
                        },
                        Err(err) => {
                            let message = format!("{}.write | error: {:?}", self.id, err);
                            if log::max_level() == LevelFilter::Trace {
                                warn!("{}", message);
                            }
                            return ConnectionStatus::Closed(message);
                        },
                    };
                }
                trace!("{}.write | bytes: {:?}", self.id, bytes);
                match tcp_stream.write(&bytes) {
                    Ok(sent) => {ConnectionStatus::Active(Ok(sent))},
                    Err(err) => {
                        self.buffer.push(bytes);
                        let message = format!("{}.write | error: {:?}", self.id, err);
                        if log::max_level() == LevelFilter::Trace {
                            warn!("{}", message);
                        }
                        return ConnectionStatus::Closed(message);
                    },
                }
            },
            Err(err) => {
                match err {
                    RecvError::Error(err) => {
                        let message = format!("{}.write | error: {:?}", self.id, err);
                        if log::max_level() == LevelFilter::Trace {
                            warn!("{}", message);
                        }
                        ConnectionStatus::Active(Err(message))
                    },
                    RecvError::Timeout => ConnectionStatus::Active(Ok(0)),
                    RecvError::Disconnected => {
                        panic!("{}.write | channel disconnected, error: {:?}", self.id, err);
                    },
                }
            },
        }
    }
}
///
/// 
unsafe impl Sync for TcpStreamWrite {}

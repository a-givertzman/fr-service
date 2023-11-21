#![allow(non_snake_case)]

use crate::tcp::steam_read::StreamRead;

///
/// Received from in queue sequences of bites adds into the end of local buffer
/// Sends sequences of bites from the beginning of the buffer
/// Sent sequences of bites immediately removed from the buffer
/// Buffering - is optional
struct TcpStreamWrite {
    stream: Box<dyn StreamRead<Vec<u8>, String>>,
}
///
/// 
impl TcpStreamWrite {
    ///
    /// Creates new instance of [TcpStreamWrite]
    pub fn new(stream: Box<dyn StreamRead<Vec<u8>, String>>,) -> Self {
        Self {
            stream,
        }
    }
}
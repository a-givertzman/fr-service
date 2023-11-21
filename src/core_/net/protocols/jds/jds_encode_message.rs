#![allow(non_snake_case)]

use std::{net::TcpStream, io::{Read, BufReader, ErrorKind}};

use log::warn;

use crate::core_::net::connection_status::ConnectionStatus;

use super::jds_serialize::JdsSerialize;


pub const JDS_END_OF_TRANSMISSION: u8 = 0x4;

enum Status {
    Active,
    Closed,
}

///
/// Converts json string into the bytes
/// adds Jds.endOfTransmission = 4 at the end of message
/// returns Result<Vec, Err>
pub struct JdsEncodeMessage {
    id: String,
    stream: JdsSerialize,
}
///
/// 
impl JdsEncodeMessage {
    ///
    /// Creates new instance of the JdsEncodeMessage
    pub fn new(parent: impl Into<String>, stream: JdsSerialize) -> Self {
        Self {
            id: format!("{}/JdsMessage", parent.into()),
            stream,
        }
    }
    ///
    /// Returns sequence of bytes representing encoded single PointType, ends with Jds.endOfTransmission = 4
    pub fn read(&mut self) -> Vec<u8> {
        // match self.stream.read() {
        //     Status::Active => {
        //         self.buffer.clear();
        //         ConnectionStatus::Active(bytes)
        //     },
        //     Status::Closed => {
        //         if !bytes.is_empty() {
        //             self.buffer = bytes;
        //         }
        //         ConnectionStatus::Closed
        //     },
        // }
        vec![]
    }
    // ///
    // /// bytes will be read from socket until JDS_END_OF_TRANSMISSION = 4
    // /// - returns Active: if read bytes non zero length without errors
    // /// - returns Closed:
    // ///    - if read 0 bytes
    // ///    - if on error
    // fn readAll(selfId: &str, bytes: &mut Vec<u8>, stream: &mut BufReader<TcpStream>) -> Status {
    //     for byte in stream.bytes() {
    //         match byte {
    //             Ok(byte) => {
    //                 // debug!("{}.readAll |     read len: {:?}", selfId, len);
    //                 match byte {
    //                     JDS_END_OF_TRANSMISSION => {
    //                         return Status::Active;
    //                     },
    //                     _ => {
    //                         bytes.push(byte);
    //                     },
    //                 };
    //             },
    //             Err(err) => {
    //                 warn!("{}.readAll | error reading from socket: {:?}", selfId, err);
    //                 warn!("{}.readAll | error kind: {:?}", selfId, err.kind());
    //                 return Self::matchErrorKind(err.kind())
    //             },
    //         };
    //     };
    //     Status::Closed
    // }
}

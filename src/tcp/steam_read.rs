use std::{io::BufReader, net::TcpStream};

use crate::core_::{net::connection_status::ConnectionStatus, object::object::Object, point::point_type::PointType};

pub trait StreamRead<T: Sync, E>: Sync + Object {
    fn read(&mut self) -> Result<T, E>;
}
///
/// 
pub trait TcpStreamRead: Send + Sync + Object {
    fn read(&mut self, tcp_stream: &mut BufReader<TcpStream>) -> ConnectionStatus<Result<PointType, String>, String>;
}
///
/// 
#[derive(Debug, Clone)]
pub struct StreamFilter {
    cot: Option<u32>,
    name: Option<String>,
}
// ///
// /// 
// impl StreamFilter {
//     ///
//     /// Creates new instance
//     /// - cot - [Cot] - bit mask wich will be passed
//     /// - name - exact name wich passed
//     pub fn allow(cot: Option<u32>, name: Option<String>) -> Self {
//         Self { cot, name }
//     }
//     ///
//     /// Returns true if any filter creteria matched
//     pub fn pass(&self, point: &PointType) -> bool {
//         match &self.cot {
//             Some(cot) => {
//                 if *cot & point.cot() > 0 {
//                     true
//                 } else {
//                     match &self.name {
//                         Some(name) => {
//                             if name == &point.name() {
//                                 true
//                             } else {
//                                 false
//                             }
//                         },
//                         None => {
//                             false
//                         },
//                     }
//                 }
//             },
//             None => {
//                 match &self.name {
//                     Some(name) => {
//                         if name == &point.name() {
//                             true
//                         } else {
//                             false
//                         }
//                     },
//                     None => false,
//                 }
//             },
//         }
//     }
// }
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

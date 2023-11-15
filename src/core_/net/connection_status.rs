///
/// Used in the TspStream handling like Result
pub enum ConnectionStatus {
    Active(Vec<u8>),
    Closed,
}

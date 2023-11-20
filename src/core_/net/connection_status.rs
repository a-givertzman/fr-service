///
/// Used in the TspStream handling like Result
pub enum ConnectionStatus<T> {
    Active(T),
    Closed,
}

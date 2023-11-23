///
/// Used in the TspStream handling like Result
pub enum ConnectionStatus<T, E> {
    Active(T),
    Closed(E),
}

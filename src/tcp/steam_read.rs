pub trait StreamRead<T, E> {
    fn read(&mut self) -> Result<T, E>;
}
pub trait StreamRead<T: Sync, E>: Sync {
    fn read(&mut self) -> Result<T, E>;
}
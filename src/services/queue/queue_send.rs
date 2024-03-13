use crate::core_::failure::error_string::ErrorString;

///
/// Holds a reference to the specific async queue / channel implementation
/// Sharing standard interface to send data into inner queueu
pub trait QueueSend<T>: std::fmt::Debug {
    fn send(&mut self, value: T) -> Result<(), ErrorString>;
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Ok = 0,
    Obsolete = 2,
    TimeInvalid = 3,
    Invalid = 10,
    Uncnown = 99,
}

#[derive(Debug, Clone)]
pub struct QueueName {
    service: String,
    queue: String,
}
///
/// 
impl QueueName {
    ///
    /// 
    pub fn new(name: &str) -> Self {
        let parts: Vec<&str> = name.split(".").collect();
        Self {
            service: parts[0].to_string(),
            queue: parts[1].to_string(),
        }
    }
    ///
    /// 
    pub fn service(&self) -> &str {
        &self.service
    }
    ///
    /// 
    pub fn queue(&self) -> &str {
        &self.queue
    }
}
///
/// Contains Name of the Queue in the separate format
/// Service.Queue -> Service & Queue
#[derive(Debug, Clone)]
pub struct QueueName {
    service: String,
    queue: String,
}
///
/// 
impl QueueName {
    ///
    /// Creates new instance of the QueueName from the string like "Service.Queue"
    pub fn new(name: &str) -> Self {
        let parts: Vec<&str> = name.split(".").collect();
        let service = match parts.get(0) {
            Some(value) => value.to_owned().to_owned(),
            None => panic!("QueueName.new | {} does not have structure 'Service.queue'", name),
        };
        let queue = match parts.get(1) {
            Some(value) => value.to_owned().to_owned(),
            None => panic!("QueueName.new | {} does not have structure 'Service.queue'", name),
        };
        Self {
            service,
            queue,
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
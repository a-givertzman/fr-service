///
/// Contains Name of the Queue in the separate format
/// Service.Queue -> Service & Queue
#[derive(Debug, Clone)]
pub struct QueueName {
    service: String,
    queue: String,
}
///
/// Contains the Service's queue name in the format 'Servece.queue'
impl QueueName {
    ///
    /// Creates new instance of the QueueName from the string like "Service.Queue"
    pub fn new(name: &str) -> Self {
        let parts: Vec<&str> = name.split('.').collect();
        let service = match parts.first() {
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
    /// Returns the Service name
    pub fn service(&self) -> &str {
        &self.service
    }
    ///
    /// Returns the Service's queue name
    pub fn queue(&self) -> &str {
        &self.queue
    }
}
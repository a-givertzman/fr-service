///
/// Handles JDS protocol messages
struct JdsConnection {
    tx_queue_name: String,
    action_recv: Vec<Receiver<Action>>, 
    services: Arc<Mutex<Services>>, 

}
///
/// 
impl JdsConnection {
    pub fn new() -> Self {

    }
}
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex, RwLock}, thread, time::Duration};
use log::{info, trace, warn};
use crate::{core_::{object::object::Object, point::point_type::PointType}, services::{service::{service::Service, service_handles::ServiceHandles}, services::Services}};
#[cfg(test)]

mod multi_queue {
    use log::debug;
    use std::{sync::{Arc, Mutex, Once}, thread, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::{
        entities::test_value::Value, 
        stuff::{random_test_values::RandomTestValues, max_test_duration::TestDuration, wait::WaitTread},
    };
    use crate::{
        conf::multi_queue_config::MultiQueueConfig, 
        core_::object::object::Object, 
        services::{multi_queue::multi_queue::MultiQueue, service::service::Service, services::Services}, 
        tests::unit::services::multi_queue::{mock_send_service::MockSendService, multi_queue_subscription_test::MockReceiver},
    }; 
    ///
    /// 
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Test MultiQueue for broadcast subscription
    /// - events sent by multiple senders
    ///     - number of events = iterations
    /// - events received by multiple receivers
    ///     - each receiver must receive events: iterations * sender_count
    #[test]
    fn subscribe_broadcast() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "multi_queue_subscribe_test";
        println!("\n{}", self_id);
        let sender_count = 10;         // count of MockSendService's
        let receiver_count = 10;         // count of MockReceiver's
        let iterations = 1000;          // test data length of the single sender
        let total_test_events = sender_count * iterations;
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let conf = r#"
            service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:  # direct send links - are empty, because only client subscribtions will be used
        "#.to_string();
        let conf = serde_yaml::from_str(&conf).unwrap();
        let mq_conf = MultiQueueConfig::from_yaml(&conf);
        debug!("mqConf: {:?}", mq_conf);
        let services = Arc::new(Mutex::new(Services::new("test")));
        let mq_service = Arc::new(Mutex::new(MultiQueue::new("test", mq_conf, services.clone())));
        services.lock().unwrap().insert("MultiQueue", mq_service.clone());
        let mut receiver_handles = vec![];
        let mut receivers = vec![];
        for i in 0..receiver_count {
            let receiver = Arc::new(Mutex::new(MockReceiver::new(
                format!("tread{}", i),
                services.clone(),
                Some(total_test_events),
            )));
            services.lock().unwrap().insert(receiver.lock().unwrap().id(), receiver.clone());
            receivers.push(receiver);
        }
        let mq_handle = mq_service.lock().unwrap().run().unwrap();
        for receiver in &receivers {
            let h = receiver.lock().unwrap().run().unwrap();
            receiver_handles.push(h);
        }
        println!("All MockReceiver's threads - started");
        thread::sleep(Duration::from_millis(100));
        let mut senders = vec![];
        let mut sender_handles = vec![];
        let time = Instant::now();
        for i in 0..sender_count {
            let dynamic_test_data = RandomTestValues::new(
                self_id, 
                vec![
                    Value::String(format!("dynamic01{}", i)),
                    Value::String(format!("dynamic02{}", i)),
                    Value::String(format!("dynamic03{}", i)),
                    Value::String(format!("dynamic04{}", i)),
                    Value::String(format!("dynamic05{}", i)),
                    Value::String(format!("dynamic06{}", i)),
                    Value::String(format!("dynamic07{}", i)),
                ], 
                iterations, 
            );
            let dynamic_test_data: Vec<Value> = dynamic_test_data.collect();
            let sender = Arc::new(Mutex::new(MockSendService::new(
                format!("tread{}", i),
                "MultiQueue.in-queue",
                services.clone(),
                dynamic_test_data.clone(),
                None,
            )));
            services.lock().unwrap().insert(&sender.lock().unwrap().id(), sender.clone());
            senders.push(sender.clone());
            let sender_handle = sender.lock().unwrap().run().unwrap();
            sender_handles.push(sender_handle);
            // thread::sleep(Duration::from_millis(50));
        }
        for h in sender_handles {
            h.wait().unwrap()
        }
        for h in receiver_handles {
            h.wait().unwrap();
        }
        for receiver in &receivers {
            receiver.lock().unwrap().exit();
        }
        let elapsed = time.elapsed();
        println!("Total elapsed: {:?}", elapsed);
        println!("Total test events: {:?}", total_test_events);
        println!("Elapsed per event: {:?}", elapsed.div_f64(total_test_events as f64));
        let target = iterations;
        for sender in senders {
            let sent = sender.lock().unwrap().sent();
            let result = sent.lock().unwrap().len();
            println!("\t {} sent: {:?}", sender.lock().unwrap().id(), result);
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        let target = total_test_events;
        for receiver in receivers {
            let result = receiver.lock().unwrap().received.read().unwrap().len();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        mq_service.lock().unwrap().exit();
        mq_handle.wait().unwrap();
        test_duration.exit();
    }
}

///
/// Receiver with subscribtion to Multiqueue
struct MockReceiver {
    id: String,
    services: Arc<Mutex<Services>>,
    received: Arc<RwLock<Vec<PointType>>>,
    recv_limit: Option<usize>,
    exit: Arc<AtomicBool>,
}
///
/// 
impl MockReceiver {
    pub fn new(parent: impl Into<String>, services: Arc<Mutex<Services>>, recv_limit: Option<usize>) -> Self {
        let self_id = format!("{}/MockReceiver", parent.into());
        Self {
            id: self_id,
            services,
            received: Arc::new(RwLock::new(vec![])),
            recv_limit,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
///
/// 
impl Object for MockReceiver {
    fn id(&self) -> &str {
        self.id.as_str()
    }
}
///
/// 
impl Service for MockReceiver {
    //
    //
    fn run(&mut self) -> Result<ServiceHandles, String> {
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let recv_limit = self.recv_limit;
        let received = self.received.clone();
        let services = self.services.clone();
        let handle = thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            let self_id = self_id.as_str();
            let points = vec![];
            let (_, recv) = services.lock().unwrap().subscribe("MultiQueue", self_id, &points);
            match recv_limit {
                Some(recv_limit) => {
                    let mut received_len = 0;
                    while received_len < recv_limit {
                        match recv.recv_timeout(Duration::from_secs(3)) {
                            Ok(point) => {
                                received_len += 1;
                                trace!("{}.run | Received point: {:#?}", self_id, point);
                                received.write().unwrap().push(point);
                            },
                            Err(err) => match err {
                                std::sync::mpsc::RecvTimeoutError::Timeout      => warn!("{}.run | Receive error: {:#?}", self_id, err),
                                std::sync::mpsc::RecvTimeoutError::Disconnected => {},
                            },
                        }
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                },
                None => {
                    loop {
                        match recv.recv_timeout(Duration::from_secs(3)) {
                            Ok(point) => {
                                received.write().unwrap().push(point)
                            },
                            Err(err) => match err {
                                std::sync::mpsc::RecvTimeoutError::Timeout      => warn!("{}.run | Receive error: {:#?}", self_id, err),
                                std::sync::mpsc::RecvTimeoutError::Disconnected => {},
                            },
                        }
                        if exit.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                },
            }
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.to_owned(), handle)]))
            },
            Err(err) => {
                let message = format!("{}.run | Start faled: {:#?}", self.id, err);
                warn!("{}", message);
                Err(message)
            },
        }
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
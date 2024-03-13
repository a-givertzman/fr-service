#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, collections::HashMap};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        tests::unit::services::multi_queue::{mock_send_service::MockSendService, mock_multi_queue::MockMultiQueue, mock_recv_service::MockRecvService, mock_multi_queue_match::MockMultiQueueMatch}, 
        services::{services::Services, service::service::Service},
    }; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
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
    fn init_each() -> () {
    
    }

    const ITERATIONS: usize = 1_000_000;
    
    #[ignore = "Performance test | use to estimate performance of multiqueue without matching producer's id"]
    #[test]
    fn test_MultiQueue_performance() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test MultiQueue Performance";
        println!("\n{}", self_id);
        let iterations = ITERATIONS;
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        
        
        let receiverCount = 3;
        let producerCount = 3;
        let totalCount = iterations * producerCount;
        let mut receivers: HashMap<String, Arc<Mutex<MockRecvService>>> = HashMap::new();
        let mut producers: HashMap<String, MockSendService> = HashMap::new();
        let services = Arc::new(Mutex::new(Services::new(self_id)));

        for i in 0..receiverCount {
            let receiver = Arc::new(Mutex::new(MockRecvService::new(
                self_id, 
                "rx-queue", 
                Some(totalCount)
            )));
            let receiverId = format!("Receiver{}", i + 1);
            services.lock().unwrap().insert(&receiverId, receiver.clone());
            receivers.insert(receiverId.clone(), receiver);
            println!(" Receiver {} created", receiverId);
        }
        println!(" All receivers created");


        println!(" Creating Mock Multiqueue...");
        let mqService = Arc::new(Mutex::new(MockMultiQueue::new(
            self_id, 
            receivers.keys().map(|v| {
                format!("{}.rx-queue", v)
            }).collect(),
            "rx-queue", 
            services.clone(),
        )));
        println!(" Creating Mock Multiqueue - ok");
        println!(" Inserting Mock Multiqueue into Services...");
        services.lock().unwrap().insert("MultiQueue", mqService.clone());
        println!(" Inserting Mock Multiqueue into Services - ok");

        let test_data = RandomTestValues::new(
            self_id, 
            vec![
                Value::Int(i64::MIN),
                Value::Int(i64::MAX),
                Value::Int(-7),
                Value::Int(0),
                Value::Int(12),
                Value::Float(f64::MAX),
                Value::Float(f64::MIN),
                Value::Float(f64::MIN_POSITIVE),
                Value::Float(-f64::MIN_POSITIVE),
                Value::Float(0.0),
                Value::Float(1.33),
                Value::Bool(true),
                Value::Bool(false),
                Value::Bool(false),
                Value::Bool(true),
                Value::String("test1".to_string()),
                Value::String("test1test1test1test1test1test1test1test1test1test1test1test1test1test1test1".to_string()),
                Value::String("test2".to_string()),
                Value::String("test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2test2".to_string()),
            ], 
            iterations, 
        );
        let test_data: Vec<Value> = test_data.collect();

        println!(" Trying to start Multiqueue...:");
        mqService.lock().unwrap().run().unwrap();
        let mut recvHandles  = vec![];
        for (_recvId, recv) in &receivers {
            let h = recv.lock().unwrap().run().unwrap();
            recvHandles.push(h)
        }

        for i in 0..producerCount {
            let mut prod = MockSendService::new(self_id, "queue", "MultiQueue.rx-queue", services.clone(), test_data.clone(), None);
            prod.run().unwrap();
            producers.insert(format!("MockSendService{}", i), prod);
        }

        let timer = Instant::now();
        for h in recvHandles {
            h.join().unwrap();
        }
        println!("\n Elapsed: {:?}", timer.elapsed());
        println!(" Total test events: {:?}", totalCount);
        let (totalSent, allSent) = getSent(&producers);
        println!(" Sent events: {}\t{:?}", totalSent, allSent);
        let (totalReceived, allReceived) = getReceived(&receivers);
        println!(" Recv events: {}\t{:?}\n", totalReceived, allReceived);

        assert!(totalSent == totalCount, "\nresult: {:?}\ntarget: {:?}", totalSent, totalCount);
        assert!(totalReceived == totalCount * receiverCount, "\nresult: {:?}\ntarget: {:?}", totalReceived, totalCount * receiverCount);
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }
    ///
    /// 
    #[ignore = "Performance test | use to estimate performance of multiqueue with matching producer's id"]
    #[test]
    fn test_MultiQueue_match_performance() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test MultiQueue Performance with matching by producer ID";
        println!("\n{}", self_id);
        let iterations = ITERATIONS;
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        
        
        let receiverCount = 3;
        let producerCount = 3;
        let totalCount = iterations * producerCount;
        let mut receivers: HashMap<String, Arc<Mutex<MockRecvService>>> = HashMap::new();
        let mut producers: HashMap<String, MockSendService> = HashMap::new();
        let services = Arc::new(Mutex::new(Services::new(self_id)));

        for i in 0..receiverCount {
            let receiver = Arc::new(Mutex::new(MockRecvService::new(
                self_id, 
                "rx-queue", 
                Some(totalCount)
            )));
            let receiverId = format!("Receiver{}", i + 1);
            services.lock().unwrap().insert(&receiverId, receiver.clone());
            receivers.insert(receiverId.clone(), receiver);
            println!(" Receiver {} created", receiverId);
        }
        println!(" All receivers created");


        println!(" Creating Mock Multiqueue...");
        let mqService = Arc::new(Mutex::new(MockMultiQueueMatch::new(
            self_id, 
            receivers.keys().map(|v| {
                format!("{}.rx-queue", v)
            }).collect(),
            "rx-queue", 
            services.clone(),
        )));
        println!(" Creating Mock Multiqueue - ok");
        println!(" Inserting Mock Multiqueue into Services...");
        services.lock().unwrap().insert("MultiQueue", mqService.clone());
        println!(" Inserting Mock Multiqueue into Services - ok");

        let test_data = RandomTestValues::new(
            self_id, 
            vec![
                Value::Int(7),
                Value::Float(1.3),
                Value::Bool(true),
                Value::Bool(false),
                Value::String("test1".to_string()),
                Value::String("test2".to_string()),
            ], 
            iterations, 
        );
        let test_data: Vec<Value> = test_data.collect();

        println!(" Trying to start Multiqueue...:");
        mqService.lock().unwrap().run().unwrap();
        let mut recvHandles  = vec![];
        for (_recvId, recv) in &receivers {
            let h = recv.lock().unwrap().run().unwrap();
            recvHandles.push(h)
        }

        for i in 0..producerCount {
            let mut prod = MockSendService::new(self_id, "queue", "MultiQueue.rx-queue", services.clone(), test_data.clone(), None);
            prod.run().unwrap();
            producers.insert(format!("MockSendService{}", i), prod);
        }

        let timer = Instant::now();
        for h in recvHandles {
            h.join().unwrap();
        }
        println!("\n Elapsed: {:?}", timer.elapsed());
        println!(" Total test events: {:?}", totalCount);
        let (totalSent, allSent) = getSent(&producers);
        println!(" Sent events: {}\t{:?}", totalSent, allSent);
        let (totalReceived, allReceived) = getReceived(&receivers);
        println!(" Recv events: {}\t{:?}\n", totalReceived, allReceived);

        assert!(totalSent == totalCount, "\nresult: {:?}\ntarget: {:?}", totalSent, totalCount);
        assert!(totalReceived == totalCount * receiverCount, "\nresult: {:?}\ntarget: {:?}", totalReceived, totalCount * receiverCount);
        // assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        test_duration.exit();
    }    
    ///
    /// 
    fn getSent<'a>(producers: &'a HashMap<String, MockSendService>) -> (usize, HashMap<&'a str, usize>) {
        let mut totalSent = 0;
        let mut allSent: HashMap<&'a str, usize> = HashMap::new();
        for (prodId, prod) in producers {
            let sent = prod.sent().lock().unwrap().len();
            totalSent += sent;
            allSent.insert(prodId, sent);
        }
        (totalSent, allSent)
    }   
    ///
    /// 
    fn getReceived<'a>(receivers: &'a HashMap<String, Arc<Mutex<MockRecvService>>>) -> (usize, HashMap<&'a str, usize>) {
        let mut totalReceived = 0;
        let mut allReceived: HashMap<&'a str, usize> = HashMap::new();
        for (recvId, recv) in receivers {
            let recved = recv.lock().unwrap().received().lock().unwrap().len();
            totalReceived += recved;
            allReceived.insert(recvId, recved);
        }
        (totalReceived, allReceived)
    }
}
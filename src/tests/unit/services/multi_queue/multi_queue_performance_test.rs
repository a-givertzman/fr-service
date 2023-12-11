#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use log::{warn, info, debug};
    use rand::{rngs::ThreadRng, Rng};
    use std::{sync::{Once, Arc, Mutex}, time::{Duration, Instant}, collections::HashMap, thread::{self, JoinHandle}};
    use crate::{core_::{debug::debug_session::{DebugSession, LogLevel, Backtrace}, testing::test_stuff::{test_value::Value, random_test_values::RandomTestValues, max_test_duration::MaxTestDuration}, point::point_type::PointType}, tests::unit::services::multi_queue::{mock_send_service::MockSendService, mock_multi_queue::MockMultiQueue, mock_recv_service::MockRecvService, mock_multi_queue_match::MockMultiQueueMatch}, services::{services::Services, service::Service}}; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn initOnce() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach() -> () {
    
    }

    const ITERATIONS: usize = 100_000;
    
    #[ignore = "Performance test | run this test to compare performance of multiqueue with matching producer's id vs without matching"]
    #[test]
    fn test_multi_queue_performance() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test MultiQueue Performance");

        let selfId = "test";
        let iterations = ITERATIONS;
        let maxTestDuration = MaxTestDuration::new(selfId, Duration::from_secs(10));
        maxTestDuration.run().unwrap();
        
        
        let receiverCount = 3;
        let producerCount = 3;
        let totalCount = iterations * producerCount;
        let mut receivers: HashMap<String, Arc<Mutex<MockRecvService>>> = HashMap::new();
        let mut producers: HashMap<String, MockSendService> = HashMap::new();
        let services = Arc::new(Mutex::new(Services::new(selfId)));

        for i in 0..receiverCount {
            let receiver = Arc::new(Mutex::new(MockRecvService::new(
                selfId, 
                "rx-queue", 
                services.clone(),
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
            selfId, 
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

        let testData = RandomTestValues::new(
            selfId, 
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
        let testData: Vec<Value> = testData.collect();

        println!(" Trying to start Multiqueue...:");
        mqService.lock().unwrap().run().unwrap();
        let mut recvHandles  = vec![];
        for (_recvId, recv) in &receivers {
            let h = recv.lock().unwrap().run().unwrap();
            recvHandles.push(h)
        }

        for i in 0..producerCount {
            let mut prod = MockSendService::new(selfId, "queue", "MultiQueue.rx-queue", services.clone(), testData.clone());
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
        maxTestDuration.exit();
    }
    ///
    /// 
    // #[ignore = "Performance test | run this test to compare performance of multiqueue with matching producer's id vs without matching"]
    #[test]
    fn test_multi_queue_match_performance() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        info!("test MultiQueue Performance with matching by producer ID");

        let selfId = "test";
        let iterations = ITERATIONS;
        let maxTestDuration = MaxTestDuration::new(selfId, Duration::from_secs(10));
        maxTestDuration.run().unwrap();
        
        
        let receiverCount = 3;
        let producerCount = 3;
        let totalCount = iterations * producerCount;
        let mut receivers: HashMap<String, Arc<Mutex<MockRecvService>>> = HashMap::new();
        let mut producers: HashMap<String, MockSendService> = HashMap::new();
        let services = Arc::new(Mutex::new(Services::new(selfId)));

        for i in 0..receiverCount {
            let receiver = Arc::new(Mutex::new(MockRecvService::new(
                selfId, 
                "rx-queue", 
                services.clone(),
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
            selfId, 
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

        let testData = RandomTestValues::new(
            selfId, 
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
        let testData: Vec<Value> = testData.collect();

        println!(" Trying to start Multiqueue...:");
        mqService.lock().unwrap().run().unwrap();
        let mut recvHandles  = vec![];
        for (_recvId, recv) in &receivers {
            let h = recv.lock().unwrap().run().unwrap();
            recvHandles.push(h)
        }

        for i in 0..producerCount {
            let mut prod = MockSendService::new(selfId, "queue", "MultiQueue.rx-queue", services.clone(), testData.clone());
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
        maxTestDuration.exit();
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
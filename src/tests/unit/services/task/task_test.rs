#[cfg(test)]

mod task {
    use log::{trace, info, debug};
    use std::{sync::{Once, Arc, Mutex}, env, time::{Instant, Duration}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, random_test_values::RandomTestValues, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::task_config::TaskConfig, 
        services::{task::{task::Task, task_test_receiver::TaskTestReceiver, task_test_producer::TaskTestProducer}, service::service::Service, services::Services},
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
    /// 
    #[test]
    fn structure() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "test";
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        //
        // can be changed
        let iterations = 10;
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_struct.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
        
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            self_id,
            "in-queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        
        let test_data = RandomTestValues::new(
            self_id, 
            vec![], 
            iterations, 
        );
        let test_data: Vec<Value> = test_data.collect();
        let total_count = test_data.len();
        assert!(total_count == iterations, "\nresult: {:?}\ntarget: {:?}", total_count, iterations);
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            self_id, 
            "Task.recv-queue",
            Duration::ZERO,
            services.clone(),
            test_data,
        )));
        let task = Arc::new(Mutex::new(Task::new(self_id, config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());
        let receiver_handle = receiver.lock().unwrap().run().unwrap();
        let producer_handle = producer.lock().unwrap().run().unwrap();
        trace!("task runing...");
        let time = Instant::now();
        let task_handle = task.lock().unwrap().run().unwrap();
        trace!("task runing - ok");
        producer_handle.wait().unwrap();
        receiver_handle.wait().unwrap();
        debug!("task.lock.exit...");
        task.lock().unwrap().exit();
        debug!("task.lock.exit - ok");
        task_handle.wait().unwrap();
        let sent = producer.lock().unwrap().sent().lock().unwrap().len();
        let result = receiver.lock().unwrap().received().lock().unwrap().len();
        println!(" elapsed: {:?}", time.elapsed());
        println!("    sent: {:?}", sent);
        println!("received: {:?}", result);
        assert!(sent == iterations, "\nresult: {:?}\ntarget: {:?}", sent, iterations);
        assert!(result == iterations, "\nresult: {:?}\ntarget: {:?}", result, iterations);
        test_duration.exit();
    }
    ///
    /// 
    #[ignore = "TODO - transfered values assertion not implemented yet"]
    #[test]
    fn transfer() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        info!("test");
        let self_id = "test";
        //
        // Can be changed
        let iterations = 10;
        trace!("dir: {:?}", env::current_dir());
        let path = "./src/tests/unit/services/task/task_test_struct.yaml";
        // let path = "./src/tests/unit/task/task_test.yaml";
        let config = TaskConfig::read(path);
        trace!("config: {:?}", &config);
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            self_id,
            "in-queue",
            iterations,
        )));
        services.lock().unwrap().insert("TaskTestReceiver", receiver.clone());
        let test_data = RandomTestValues::new(
            self_id, 
            vec![
                Value::Float(f64::MAX),
                Value::Float(f64::MIN),
                Value::Float(f64::MIN_POSITIVE),
                Value::Float(-f64::MIN_POSITIVE),
                Value::Float(0.11),
                Value::Float(1.33),
            ], 
            iterations, 
        );
        let test_data: Vec<Value> = test_data.collect();
        // let totalCount = test_data.len();
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            self_id,
            "Task.recv-queue",
            Duration::ZERO,
            services.clone(),
            test_data,
        )));
        let task = Arc::new(Mutex::new(Task::new(self_id, config, services.clone())));
        services.lock().unwrap().insert("Task", task.clone());
        let receiver_handle = receiver.lock().unwrap().run().unwrap();
        let producer_handle = producer.lock().unwrap().run().unwrap();
        trace!("task runing...");
        let time = Instant::now();
        task.lock().unwrap().run().unwrap();
        trace!("task runing - ok");
        producer_handle.wait().unwrap();
        receiver_handle.wait().unwrap();
        let producer_sent = producer.lock().unwrap().sent();
        let sent = producer_sent.lock().unwrap();
        let receiver_received = receiver.lock().unwrap().received();
        let mut received = receiver_received.lock().unwrap();
        println!(" elapsed: {:?}", time.elapsed());
        println!("    sent: {:?}", sent.len());
        println!("received: {:?}", received.len());
        assert!(sent.len() == iterations, "\nresult: {:?}\ntarget: {:?}", sent.len(), iterations);
        assert!(received.len() == iterations, "\nresult: {:?}\ntarget: {:?}", received.len(), iterations);
        for sentPoint in sent.iter() {
            let recv_point = received.pop().unwrap();
            assert!(&recv_point == sentPoint, "\nresult: {:?}\ntarget: {:?}", recv_point, sentPoint);
        }
    }
}


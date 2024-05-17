#[cfg(test)]

mod cma_recorder {
    use log::{debug, info, trace};
    use std::{env, sync::{Arc, Mutex, Once}, thread, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::{multi_queue_config::MultiQueueConfig, point_config::name::Name, task_config::TaskConfig},
        services::{multi_queue::multi_queue::MultiQueue, safe_lock::SafeLock, service::service::Service, services::Services, task::{task::Task, task_test_receiver::TaskTestReceiver}}, tests::unit::services::cma_recorder::task_test_producer::TaskTestProducer,
    };
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    ///
    #[test]
    fn filter() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "App";
        let self_name = Name::new("", self_id);
        println!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(20));
        test_duration.run().unwrap();
        //
        // can be changed
        trace!("dir: {:?}", env::current_dir());
        let services = Arc::new(Mutex::new(Services::new(self_id)));
        let config = TaskConfig::from_yaml(
            &self_name,
            &serde_yaml::from_str(r"
                service Task RecorderTask:
                    cycle: 1 ms
                    in queue recv-queue:
                        max-length: 10000
                    subscribe:
                        /App/MultiQueue:                    # - multicast subscription to the MultiQueue
                            {cot: Inf}: []                      #   - on all points having Cot::Inf
                    fn debug debug01:
                        input fn Filter:
                            send-to: /App/MultiQueue.in-queue
                            conf point Load001:
                                type: 'Real'
                            input: point real '/App/Load'
                            pass fn Ge:
                                input1: point real '/App/Load'
                                input2: const real 1.5
                    fn debug debug02:
                        input point Load002:
                            type: 'Real'
                            input: point real '/App/RecorderTask/Load001'
                            send-to: /App/TaskTestReceiver.in-queue
            ").unwrap(),
        );
        trace!("config: {:?}", config);
        debug!("Task config points: {:#?}", config.points());

        let task = Arc::new(Mutex::new(Task::new(config, services.clone())));
        debug!("Task points: {:#?}", task.lock().unwrap().points());

        services.slock().insert(task.clone());
        let conf = MultiQueueConfig::from_yaml(
            self_id,
            &serde_yaml::from_str(r"service MultiQueue:
                in queue in-queue:
                    max-length: 10000
                out queue:
            ").unwrap(),
        );
        let multi_queue = Arc::new(Mutex::new(MultiQueue::new(conf, services.clone())));
        services.slock().insert(multi_queue.clone());
        let test_data = vec![
            (format!("/{}/Load", self_id), Value::Real(-7.035)),
            (format!("/{}/Load", self_id), Value::Real(-2.5)),
            (format!("/{}/Load", self_id), Value::Real(-5.5)),
            (format!("/{}/Load", self_id), Value::Real(-1.5)),
            (format!("/{}/Load", self_id), Value::Real(-1.0)),
            (format!("/{}/Load", self_id), Value::Real(-0.1)),
            (format!("/{}/Load", self_id), Value::Real(0.1)),
            (format!("/{}/Load", self_id), Value::Real(1.0)),
            (format!("/{}/Load", self_id), Value::Real(1.5)),
            (format!("/{}/Load", self_id), Value::Real(5.5)),
            (format!("/{}/Load", self_id), Value::Real(2.5)),
            (format!("/{}/Load", self_id), Value::Real(7.035)),
        ];
        let filtered_test_data: Vec<(String, Value)> = test_data.iter().cloned().filter(|(_, value)| {
            match value {
                Value::Bool(_value) => false,
                Value::Int(_value) => false,
                Value::Real(value) => value >= &1.5,
                Value::Double(_value) => false,
                Value::String(_value) => false,
            }
        }).collect();
        let filtered_total_count = filtered_test_data.len();
        let total_count = test_data.len();
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            self_id,
            "",
            "in-queue",
            filtered_total_count,
        )));
        services.slock().insert(receiver.clone());
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            self_id,
            &format!("/{}/MultiQueue.in-queue", self_id),
            Duration::ZERO,
            services.clone(),
            &test_data,
        )));
        services.slock().insert(producer.clone());
        let multi_queue_handle = multi_queue.lock().unwrap().run().unwrap();
        let receiver_handle = receiver.lock().unwrap().run().unwrap();
        info!("receiver runing - ok");
        let task_handle = task.lock().unwrap().run().unwrap();
        info!("task runing - ok");
        thread::sleep(Duration::from_millis(100));
        let producer_handle = producer.lock().unwrap().run().unwrap();
        info!("producer runing - ok");
        let time = Instant::now();
        receiver_handle.wait().unwrap();
        producer.lock().unwrap().exit();
        task.lock().unwrap().exit();
        task_handle.wait().unwrap();
        producer_handle.wait().unwrap();
        multi_queue.lock().unwrap().exit();
        multi_queue_handle.wait().unwrap();
        let sent = producer.lock().unwrap().sent().lock().unwrap().len();
        let result = receiver.lock().unwrap().received().lock().unwrap().len();
        println!(" elapsed: {:?}", time.elapsed());
        println!("    sent: {:?}", sent);
        println!("received: {:?}", result);
        assert!(sent == total_count, "\nresult: {:?}\ntarget: {:?}", sent, total_count);
        assert!(result == filtered_total_count, "\nresult: {:?}\ntarget: {:?}", result, total_count);
        let target_name = "/App/RecorderTask/Load002";
        for (i, result) in receiver.lock().unwrap().received().lock().unwrap().iter().enumerate() {
            let (_, target) = filtered_test_data[i].clone();
            assert!(result.value() == target, "\nresult: {:?}\ntarget: {:?}", result.value(), target);
            assert!(result.name() == target_name, "\nresult: {:?}\ntarget: {:?}", result.name(), target_name);
        };
        test_duration.exit();
    }
}

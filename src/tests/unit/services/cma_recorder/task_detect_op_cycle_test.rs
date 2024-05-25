#[cfg(test)]

mod cma_recorder {
    use log::{debug, info, trace};
    use std::{env, sync::{Arc, Mutex, Once}, thread, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::{multi_queue_config::MultiQueueConfig, point_config::name::Name, task_config::TaskConfig}, core_::aprox_eq::aprox_eq::AproxEq, services::{multi_queue::multi_queue::MultiQueue, safe_lock::SafeLock, service::service::Service, services::Services, task::{task::Task, task_test_receiver::TaskTestReceiver}}, tests::unit::services::cma_recorder::task_test_producer::TaskTestProducer
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
    fn detect_operating_cycle() {
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

                    let loadNom:    # The nominal value of the crane load
                        # input: const real 1200
                        input: point real '/App/Load.Nom'

                    let opCycleIsActive:
                        input fn Threshold:              # Triggering threshold of the operating cycle detection function on the input value based on the nominal value
                            threshold fn Mul:           # 5 % of the nominal crane load
                                input1: const real 0.05
                                input2: loadNom         # The nominal value of the crane load 
                            input fn Export:
                                send-to: /App/TaskTestReceiver.in-queue
                                input fn Smooth:
                                    factor: const real 0.125
                                    input: point real '/App/Load'
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
        //  step    input  target
            (00,    format!("/{}/Load.Nom", self_id),   Value::Real(  150.00),      0.0f32),
            (00,    format!("/{}/Load", self_id),       Value::Real(  0.00),        0.0),
            (01,    format!("/{}/Load", self_id),       Value::Real(  0.00),        0.0),
            (02,    format!("/{}/Load", self_id),       Value::Real(  3.30),        0.4125),
            (03,    format!("/{}/Load", self_id),       Value::Real(  0.10),        0.3734375),
            (04,    format!("/{}/Load", self_id),       Value::Real(  0.00),        0.3267578125),
            (05,    format!("/{}/Load", self_id),       Value::Real(  1.60),        0.4859130859375),
            (06,    format!("/{}/Load", self_id),       Value::Real(  0.00),        0.425173950195313),
            (07,    format!("/{}/Load", self_id),       Value::Real(  7.20),        1.2720272064209),
            (08,    format!("/{}/Load", self_id),       Value::Real(  0.00),        1.11302380561829),
            (09,    format!("/{}/Load", self_id),       Value::Real(  0.30),        1.011395829916),
            (10,    format!("/{}/Load", self_id),       Value::Real(  2.20),        1.1599713511765),
            (11,    format!("/{}/Load", self_id),       Value::Real(  8.10),        2.02747493227944),
            (12,    format!("/{}/Load", self_id),       Value::Real(  1.90),        2.01154056574451),
            (13,    format!("/{}/Load", self_id),       Value::Real(  0.10),        1.77259799502644),
            (14,    format!("/{}/Load", self_id),       Value::Real(  0.00),        1.55102324564814),
            (15,    format!("/{}/Load", self_id),       Value::Real(  0.00),        1.35714533994212),
            (16,    format!("/{}/Load", self_id),       Value::Real(  5.00),        1.81250217244936),
            (17,    format!("/{}/Load", self_id),       Value::Real(  2.00),        1.83593940089319),
            (18,    format!("/{}/Load", self_id),       Value::Real(  1.00),        1.73144697578154),
            (19,    format!("/{}/Load", self_id),       Value::Real(  0.00),        1.51501610380885),
            (20,    format!("/{}/Load", self_id),       Value::Real(  2.00),        1.57563909083274),
            (21,    format!("/{}/Load", self_id),       Value::Real(  4.00),        1.87868420447865),
            (22,    format!("/{}/Load", self_id),       Value::Real(  6.00),        2.39384867891882),
            (23,    format!("/{}/Load", self_id),       Value::Real( 12.00),        3.59461759405396),
            (24,    format!("/{}/Load", self_id),       Value::Real( 64.00),        11.1452903947972),
            (25,    format!("/{}/Load", self_id),       Value::Real(128.00),        25.7521290954476),
            (26,    format!("/{}/Load", self_id),       Value::Real(120.00),        37.5331129585166),
            (27,    format!("/{}/Load", self_id),       Value::Real(133.00),        49.466473838702),
            (28,    format!("/{}/Load", self_id),       Value::Real(121.00),        58.4081646088643),
            (29,    format!("/{}/Load", self_id),       Value::Real(130.00),        67.3571440327563),
            (30,    format!("/{}/Load", self_id),       Value::Real(127.00),        74.8125010286617),
            (31,    format!("/{}/Load", self_id),       Value::Real(123.00),        80.835938400079),
            (32,    format!("/{}/Load", self_id),       Value::Real(122.00),        85.9814461000691),
            (33,    format!("/{}/Load", self_id),       Value::Real(120.00),        90.2337653375605),
            (34,    format!("/{}/Load", self_id),       Value::Real( 64.00),        86.9545446703654),
            (35,    format!("/{}/Load", self_id),       Value::Real( 32.00),        80.0852265865698),
            (36,    format!("/{}/Load", self_id),       Value::Real( 24.00),        73.0745732632485),
            (37,    format!("/{}/Load", self_id),       Value::Real( 12.00),        65.4402516053425),
            (38,    format!("/{}/Load", self_id),       Value::Real(  8.00),        58.2602201546747),
            (39,    format!("/{}/Load", self_id),       Value::Real( 17.00),        53.1026926353403),
            (40,    format!("/{}/Load", self_id),       Value::Real( 10.00),        47.7148560559228),
            (41,    format!("/{}/Load", self_id),       Value::Real(  7.00),        42.6254990489324),
            (42,    format!("/{}/Load", self_id),       Value::Real(  3.00),        37.6723116678159),
            (43,    format!("/{}/Load", self_id),       Value::Real(  6.00),        33.7132727093389),
            (44,    format!("/{}/Load", self_id),       Value::Real(  4.00),        29.9991136206715),
            (45,    format!("/{}/Load", self_id),       Value::Real(  2.00),        26.4992244180876),
            (46,    format!("/{}/Load", self_id),       Value::Real(  0.00),        23.1868213658266),
            (47,    format!("/{}/Load", self_id),       Value::Real(  4.00),        20.7884686950983),
            (48,    format!("/{}/Load", self_id),       Value::Real(  2.00),        18.439910108211),
            (49,    format!("/{}/Load", self_id),       Value::Real(  1.00),        16.2599213446847),
            (50,    format!("/{}/Load", self_id),       Value::Real(  3.00),        14.6024311765991),
            (51,    format!("/{}/Load", self_id),       Value::Real(  0.00),        12.7771272795242),
            (52,    format!("/{}/Load", self_id),       Value::Real(  2.00),        11.4299863695837),
            (53,    format!("/{}/Load", self_id),       Value::Real(  1.00),        10.1262380733857),
            (54,    format!("/{}/Load", self_id),       Value::Real(  0.70),        8.94795831421249),
            (55,    format!("/{}/Load", self_id),       Value::Real(  0.80),        7.92946352493593),
            (56,    format!("/{}/Load", self_id),       Value::Real(  0.40),        6.98828058431894),
            (57,    format!("/{}/Load", self_id),       Value::Real(  0.30),        6.15224551127907),
            (58,    format!("/{}/Load", self_id),       Value::Real(  0.20),        5.40821482236919),
            (59,    format!("/{}/Load", self_id),       Value::Real(  0.10),        4.74468796957304),
            (60,    format!("/{}/Load", self_id),       Value::Real(  0.00),        4.15160197337641),
            (61,    format!("/{}/Load", self_id),       Value::Real(  0.00),        3.63265172670436),
            (62,    format!("/{}/Load", self_id),       Value::Real(  0.00),        3.17857026086631),
            (63,    format!("/{}/Load", self_id),       Value::Real(  0.00),        2.78124897825802),
        ];
        let total_count = test_data.len();
        let target_data: Vec<(i32, String, f32)> = test_data.iter().map(|(i, name, value, target)| {
            (*i, name.clone(), target.clone())
        }).collect();
        let target_count = target_data.len();
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            self_id,
            "",
            "in-queue",
            target_count,
        )));
        services.slock().insert(receiver.clone());
        let test_data: Vec<(String, Value)> = test_data.into_iter().map(|(i, name, value, target)| {
            (name, value)
        }).collect();
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
        assert!(result == target_count, "\nresult: {:?}\ntarget: {:?}", result, target_count);
        // let target_name = "/App/RecorderTask/Load002";
        for (i, result) in receiver.lock().unwrap().received().lock().unwrap().iter().enumerate() {
            let (step, name, target) = target_data[i].clone();
            assert!(result.value().as_real().aprox_eq(target, 3), "step {} \nresult: {:?}\ntarget: {:?}", step, result.value(), target);
            // assert!(result.name() == target_name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target_name);
        };
        test_duration.exit();
    }
}


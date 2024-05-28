#[cfg(test)]

mod cma_recorder {
    use log::{debug, info, trace};
    use std::{env, sync::{Arc, Mutex, Once}, thread, time::{Duration, Instant}};
    use testing::{entities::test_value::Value, stuff::{max_test_duration::TestDuration, wait::WaitTread}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{
        conf::{multi_queue_config::MultiQueueConfig, point_config::name::Name, task_config::TaskConfig},
        core_::point::point_type::PointType,
        services::{
            multi_queue::multi_queue::MultiQueue, safe_lock::SafeLock, service::service::Service, services::Services,
            task::{task::Task, task_test_receiver::TaskTestReceiver},
        },
        tests::unit::services::task::task_test_producer::TaskTestProducer,
        // tests::unit::services::cma_recorder::task_test_producer::TaskTestProducer
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
    /// Testing the Recorder's SQL generated after detected operating cycle finished
    #[test]
    fn operating_cycle_metric() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!();
        let self_id = "AppTest";
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
                        /AppTest/MultiQueue:                    # - multicast subscription to the MultiQueue
                            {cot: Inf}: []                      #   - on all points having Cot::Inf

                    fn Export:
                        send-to: /AppTest/TaskTestReceiver.in-queue
                        input: point string /AppTest/Exit
                    #
                    # The nominal value of the crane load
                    let loadNom:
                        # input: const real 150
                        input: point real '/AppTest/Load.Nom'

                    #
                    # 5 % of the nominal crane load - used for Op Cycle detection
                    let opCycleThreshold:
                        input fn Mul:
                            input1: const real 0.05
                            input2: loadNom

                    #
                    # Detect if operating cycle is active (true - isActive, false - isNotActive)
                    let opCycleIsActive:
                        input fn Export:
                            send-to: /AppTest/TaskTestReceiver.in-queue
                            conf point OpCycle:
                                type: 'Bool'
                            input fn Ge:
                                input2: opCycleThreshold
                                input1 fn Threshold:                 # Triggering threshold of the operating cycle detection function on the input value based on the nominal value
                                    threshold: opCycleThreshold
                                    input fn Smooth:
                                        factor: const real 0.125
                                        input: point real '/AppTest/Load'
                    #
                    # Detecting if any Pump is active
                    let pumpIsActive:
                        input: opCycleIsActive
                    #
                    # Detecting if any Winch is active
                    let winch1IsActive:
                        input: opCycleIsActive
                    let winch2IsActive:
                        input: opCycleIsActive
                    let winch3IsActive:
                        input: opCycleIsActive
                    #
                    # Alarm class of the operating cycle
                    # Must be >0 if one of metric is alarmed
                    let alarmClass:
                        input: const int 0

                    #
                    # Count the operating cycle ID (retained localy)
                    let opCycleId:
                        input fn Retain:
                            key: 'OperatingCycleId'
                            input fn Acc:
                                initial fn Retain:
                                    default: const int 0
                                    key: 'OperatingCycleId'
                                input fn FallingEdge:
                                    input: opCycleIsActive
                    #
                    # Crane Average load in operating cycle, ??? unit ???
                    let cycleAverageLoad:
                        input fn Average:
                            enable fn Add:
                                input1: opCycleIsActive
                                input2 fn FallingEdge:
                                    input: opCycleIsActive
                            input: point real '/AppTest/Load'   # ??? unit ???
                    #
                    # Winch1 Average load in operating cycle, ??? unit ???
                    let winch1CycleAverageLoad:
                        input fn Average:
                            enable fn Add:
                                input1: opCycleIsActive
                                input2 fn FallingEdge:
                                    input: opCycleIsActive
                            input: point real '/AppTest/Load'   # ??? unit ???
                    #
                    # Winch2 Average load in operating cycle, ??? unit ???
                    let winch2CycleAverageLoad:
                        input fn Average:
                            enable fn Add:
                                input1: opCycleIsActive
                                input2 fn FallingEdge:
                                    input: opCycleIsActive
                            input: point real '/AppTest/Load'   # ??? unit ???
                    #
                    # Winch3 Average load in operating cycle, ??? unit ???
                    let winch3CycleAverageLoad:
                        input fn Average:
                            enable fn Add:
                                input1: opCycleIsActive
                                input2 fn FallingEdge:
                                    input: opCycleIsActive
                            input: point real '/AppTest/Load'   # ??? unit ???
                    #
                    # Winch1 load-limiter-trip-count
                    let winch1LoadLimiterTripCount:
                        input fn Retain:
                            key: 'winch1-load-limiter-trip-count'
                            input fn Acc:
                                initial fn Retain:
                                    default: const int 0
                                    key: 'winch1-load-limiter-trip-count'
                                input fn RisingEdge:
                                    input: point bool '/AppTest/Winch1.Load.Limiter.Trip'
                    #
                    # Winch2 load-limiter-trip-count
                    let winch2LoadLimiterTripCount:
                        input fn Retain:
                            key: 'winch2-load-limiter-trip-count'
                            input fn Acc:
                                initial fn Retain:
                                    default: const int 0
                                    key: 'winch2-load-limiter-trip-count'
                                input fn RisingEdge:
                                    input: point bool '/AppTest/Winch2.Load.Limiter.Trip'
                    #
                    # Winch3 load-limiter-trip-count
                    let winch3LoadLimiterTripCount:
                        input fn Retain:
                            key: 'winch3-load-limiter-trip-count'
                            input fn Acc:
                                initial fn Retain:
                                    default: const int 0
                                    key: 'winch3-load-limiter-trip-count'
                                input fn RisingEdge:
                                    input: point bool '/AppTest/Winch3.Load.Limiter.Trip'
                    #
                    # crane-characteristic-number	текущее характеристическое число для крана
                    let craneEigenValue:
                        input fn Retain:
                            key: 'crane-characteristic-number'
                            input fn Add:
                                input1 fn Filter:
                                    default: const real 0.0
                                    pass: opCycleIsActive
                                    input fn Pow:
                                        input1 fn Div:
                                            input1: cycleAverageLoad
                                            input2: loadNom
                                        input2: const real 3.0
                                input2 fn Retain:
                                    default: const real 0.0
                                    key: 'crane-characteristic-number'
                    #
                    # winch1-characteristic-number	текущее характеристическое число лебедка
                    #let winch1EigenValue:
                    #    input fn Acc:
                    #        input fn Pow:
                    #            input1 fn Div:
                    #                input1: point real '/AppTest/Load'
                    #                input2: loadNom
                    #            input2: const real 3.0
                    #
                    #        input fn Retain:
                    #            key: 'winch1-characteristic-number'
                    #            input fn Add:
                    #                input1 fn Filter:
                    #                    default: const real 0.0
                    #                    pass: opCycleIsActive
                    #                    input: cycleAverageLoad
                    #                input2 fn Retain:
                    #                    default: const real 0.0
                    #                    key: 'winch1-characteristic-number'

                    ###############   Operating Cycle Metrics   ###############
                    #
                    #   table:      operating_cycle
                    #   table:      operating_cycle_metric_value
                    #
                    fn RecOpCycleMetric:
                        # send-to: /App/ApiClient.in-queue
                        send-to: /AppTest/TaskTestReceiver.in-queue
                        op-cycle: opCycleIsActive
                        # conf point OpCycleSql:
                        #     type: 'String'

                        #
                        # Operating cycle
                        input1 fn SqlMetric:
                            table: public.operating_cycle
                            sql: insert into {table} (id, timestamp_start, timestamp_stop, alarm_class) values ({opCycleId.value}, '{start.timestamp}', '{stop.timestamp}', {alarmClass.value});
                            opCycleId: opCycleId
                            start fn Filter:
                                pass fn RisingEdge:
                                    input: opCycleIsActive
                                input: point real '/AppTest/Load'
                            stop: point real '/AppTest/Load'
                            alarmClass: alarmClass

                        #
                        # Operating cycle metric Average Load
                        input2 fn SqlMetric:
                            table: public.operating_cycle_metric_value
                            sql: insert into {table} (operating_cycle_id, pid, metric_id, value) values ({opCycleId.value}, 0, 'average_load', {input.value});
                            opCycleId: opCycleId
                            input: cycleAverageLoad

                    ###############   Operating Metrics   ###############
                    #
                    #   table:      operating_metric
                    #
                        #
                        #                !!! IN SECONDS
                        # 3.1   | real | crane-total-operating-secs  | общее количество часов работы крана
                        input31 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'crane-total-operating-secs';
                            input fn Retain:
                                key: 'crane-total-operating-secs'
                                input fn Timer:
                                    initial fn Retain:
                                        default: const real 0.0
                                        key: 'crane-total-operating-secs'
                                    input: opCycleIsActive
                        #
                        # 3.2.0 | real | pump-total-operating-secs   | общее количество часов работы насосной станции (мото-секунды)
                        input320 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'pump-total-operating-secs';
                            input fn Retain:
                                key: 'pump-total-operating-secs'
                                input fn Timer:
                                    initial fn Retain:
                                        default: const real 0.0
                                        key: 'pump-total-operating-secs'
                                    input: pumpIsActive
                        #
                        # 3.2.1 | real | winch1-total-operating-secs | общее количество часов работы лебедки 1 (мото-секунды)
                        input321 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch1-total-operating-secs';
                            input fn Retain:
                                key: 'winch1-total-operating-secs'
                                input fn Timer:
                                    initial fn Retain:
                                        default: const real 0.0
                                        key: 'winch1-total-operating-secs'
                                    input: winch1IsActive
                        #
                        # 3.2.2 | real | winch2-total-operating-secs | общее количество часов работы лебедки 2 (мото-секунды)
                        input322 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch2-total-operating-secs';
                            input fn Retain:
                                key: 'winch2-total-operating-secs'
                                input fn Timer:
                                    initial fn Retain:
                                        default: const real 0.0
                                        key: 'winch2-total-operating-secs'
                                    input: winch2IsActive
                        #
                        # 3.2.3 | real | winch3-total-operating-secs | общее количество часов работы лебедки 3 (мото-секунды)
                        input323 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch3-total-operating-secs';
                            input fn Retain:
                                key: 'winch3-total-operating-secs'
                                input fn Timer:
                                    initial fn Retain:
                                        default: const real 0.0
                                        key: 'winch3-total-operating-secs'
                                    input: winch3IsActive
                        #
                        # 3.3 | int | total-operating-cycles-count | суммарное число рабочих циклов
                        input33 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'total-operating-cycles-count';
                            input: opCycleId
                        #
                        # 3.4   	real	cycles-distribution-by-load-ranges	распределение циклов по диапазонам нагрузки	0.0
                        #
                        # 3.4.1 	real	cycles-0_05-0_15-load-range	циклов в диапазоне загрузки 0,05 - 0,15	0.0
                        #
                        # 3.4.2 	real	cycles-0_15-0_25_load-range	циклов в диапазоне загрузки 0,15 - 0,25	0.0
                        #
                        # 3.4.3 	real	cycles-0_25-0_35_load-range	циклов в диапазоне загрузки 0,25 - 0,35	0.0
                        #
                        # 3.4.4 	real	cycles-0_35-0_45_load-range	циклов в диапазоне загрузки 0,35 - 0,45	0.0
                        #
                        # 3.4.5 	real	cycles-0_45-0_55_load-range	циклов в диапазоне загрузки 0,45 - 0,55	0.0
                        #
                        # 3.4.6 	real	cycles-0_55-0_65_load-range	циклов в диапазоне загрузки 0,55 - 0,65	0.0
                        #
                        # 3.4.7 	real	cycles-0_65-0_75_load-range	циклов в диапазоне загрузки 0,65 - 0,75	0.0
                        #
                        # 3.4.8 	real	cycles-0_75-0_85_load-range	циклов в диапазоне загрузки 0,75 - 0,85	0.0
                        #
                        # 3.4.9 	real	cycles-0_85-0_95_load-range	циклов в диапазоне загрузки 0,85 - 0,95	0.0
                        #
                        # 3.4.10	real	cycles-0_95-1_05_load-range	циклов в диапазоне загрузки 0,95 - 1,05	0.0
                        #
                        # 3.4.11	real	cycles-1_05-1_15_load-range	циклов в диапазоне загрузки 1,05 - 1,15	0.0
                        #
                        # 3.4.12	real	cycles-1_15-1_25_load-range	циклов в диапазоне загрузки 1,15 - 1,25	0.0                     
                        #
                        # 3.4.13	real	cycles-1_15-_load-range	циклов в диапазоне загрузки 1,25 -	0.0
                        #
                        # 3.5   	real	crane-total-lifted-mass	суммарная масса поднятых грузов. тонн	0.0
                        input35 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'crane-total-lifted-mass';
                            input fn Retain:
                                key: 'crane-total-lifted-mass'
                                input fn Add:
                                    input1 fn Filter:
                                        default: const real 0.0
                                        pass: opCycleIsActive
                                        input: cycleAverageLoad
                                    input2 fn Retain:
                                        default: const real 0.0
                                        key: 'crane-total-lifted-mass'

                        #
                        # 3.5.1 	real	winch1-total-lifted-mass	суммарная масса поднятых грузов лебедка 1	0.0
                        input351 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch1-total-lifted-mass';
                            input fn Retain:
                                key: 'winch1-total-lifted-mass'
                                input fn Add:
                                    input1 fn Filter:
                                        default: const real 0.0
                                        pass: opCycleIsActive
                                        input: winch1CycleAverageLoad
                                    input2 fn Retain:
                                        default: const real 0.0
                                        key: 'winch1-total-lifted-mass'
                        #
                        # 3.5.2 	real	winch2-total-lifted-mass	суммарная масса поднятых грузов лебедка 2	0.0
                        input352 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch2-total-lifted-mass';
                            input fn Retain:
                                key: 'winch2-total-lifted-mass'
                                input fn Add:
                                    input1 fn Filter:
                                        default: const real 0.0
                                        pass: opCycleIsActive
                                        input: winch2CycleAverageLoad
                                    input2 fn Retain:
                                        default: const real 0.0
                                        key: 'winch2-total-lifted-mass'
                        #
                        # 3.5.3 	real	winch3-total-lifted-mass	суммарная масса поднятых грузов лебедка 3	0.0
                        input353 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch3-total-lifted-mass';
                            input fn Retain:
                                key: 'winch3-total-lifted-mass'
                                input fn Add:
                                    input1 fn Filter:
                                        default: const real 0.0
                                        pass: opCycleIsActive
                                        input: winch3CycleAverageLoad
                                    input2 fn Retain:
                                        default: const real 0.0
                                        key: 'winch3-total-lifted-mass'
                        #
                        # 3.6.1 	int	winch1-load-limiter-trip-count	количество срабатываний ограничителя грузоподъемности лебедка 1	0
                        input361 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch1-load-limiter-trip-count';
                            input: winch1LoadLimiterTripCount
                        #
                        # 3.6.2 	int	winch2-load-limiter-trip-count	количество срабатываний ограничителя грузоподъемности лебедка 2	0
                        input362 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch2-load-limiter-trip-count';
                            input: winch2LoadLimiterTripCount
                        #
                        # 3.6.3 	int	winch3-load-limiter-trip-count	количество срабатываний ограничителя грузоподъемности лебедка 3	0
                        input363 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'winch3-load-limiter-trip-count';
                            input: winch3LoadLimiterTripCount
                        #
                        # 3.7   	real	crane-characteristic-number	текущее характеристическое число для крана	0.0
                        input37 fn SqlMetric:
                            table: public.operating_metric
                            sql: update {table} set value = {input.value} where name = 'crane-characteristic-number';
                            input: craneEigenValue
                        #
                        # 3.7.1 	real	winch1-characteristic-number	текущее характеристическое число лебедка 1	0.0
                        #
                        # 3.7.2 	real	winch2-characteristic-number	текущее характеристическое число лебедка 2	0.0
                        #
                        # 3.7.3 	real	winch3-characteristic-number	текущее характеристическое число лебедка 3	0.0
                        #
                        # 3.8   	real	crane-load-spectrum-factor	коэффициент распределения нагрузок для крана	0.0
                        #
                        # 3.8.1 	real	winch1-load-spectrum-factor	коэффициент распределения нагрузок лебедка 1	0.0
                        #
                        # 3.8.2 	real	winch2-load-spectrum-factor	коэффициент распределения нагрузок лебедка 2	0.0
                        #
                        # 3.8.3 	real	winch3-load-spectrum-factor	коэффициент распределения нагрузок лебедка 3	0.0
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
        //  step    nape                                input                    Pp Cycle   target_thrh             target_smooth
            ("00.0",    format!("/{}/Load.Nom", self_id),   Value::Real(  150.00),     0,       00.0000,                0.0f32),
            ("00.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                0.0),
            ("01.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                0.0),
            ("02.0",    format!("/{}/Load", self_id),       Value::Real(  3.30),       0,       00.0000,                0.4125),
            ("03.0",    format!("/{}/Load", self_id),       Value::Real(  0.10),       0,       00.0000,                0.3734375),
            ("04.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                0.3267578125),
            ("05.0",    format!("/{}/Load", self_id),       Value::Real(  1.60),       0,       00.0000,                0.4859130859375),
            ("06.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                0.425173950195313),
            ("07.0",    format!("/{}/Load", self_id),       Value::Real(  7.20),       0,       00.0000,                1.2720272064209),
            ("08.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                1.11302380561829),
            ("09.0",    format!("/{}/Load", self_id),       Value::Real(  0.30),       0,       00.0000,                1.011395829916),
            ("10.0",    format!("/{}/Load", self_id),       Value::Real(  2.20),       0,       00.0000,                1.1599713511765),
            ("11.0",    format!("/{}/Load", self_id),       Value::Real(  8.10),       0,       00.0000,                2.02747493227944),
            ("12.0",    format!("/{}/Load", self_id),       Value::Real(  1.90),       0,       00.0000,                2.01154056574451),
            ("13.0",    format!("/{}/Load", self_id),       Value::Real(  0.10),       0,       00.0000,                1.77259799502644),
            ("14.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                1.55102324564814),
            ("15.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                1.35714533994212),
            ("16.0",    format!("/{}/Load", self_id),       Value::Real(  5.00),       0,       00.0000,                1.81250217244936),
            ("17.0",    format!("/{}/Load", self_id),       Value::Real(  2.00),       0,       00.0000,                1.83593940089319),
            ("17.1",    format!("/{}/Winch1.Load.Limiter.Trip", self_id),       Value::Bool(true),       0,       00.0000,                1.83593940089319),
            ("17.2",    format!("/{}/Winch1.Load.Limiter.Trip", self_id),       Value::Bool(false),       0,       00.0000,                1.83593940089319),
            ("17.3",    format!("/{}/Winch2.Load.Limiter.Trip", self_id),       Value::Bool(true),       0,       00.0000,                1.83593940089319),
            ("17.4",    format!("/{}/Winch2.Load.Limiter.Trip", self_id),       Value::Bool(false),       0,       00.0000,                1.83593940089319),
            ("17.5",    format!("/{}/Winch3.Load.Limiter.Trip", self_id),       Value::Bool(true),       0,       00.0000,                1.83593940089319),
            ("17.6",    format!("/{}/Winch3.Load.Limiter.Trip", self_id),       Value::Bool(false),       0,       00.0000,                1.83593940089319),
            ("18.0",    format!("/{}/Load", self_id),       Value::Real(  1.00),       0,       00.0000,                1.73144697578154),
            ("19.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       00.0000,                1.51501610380885),
            ("20.0",    format!("/{}/Load", self_id),       Value::Real(  2.00),       0,       00.0000,                1.57563909083274),
            ("21.0",    format!("/{}/Load", self_id),       Value::Real(  4.00),       0,       00.0000,                1.87868420447865),
            ("22.0",    format!("/{}/Load", self_id),       Value::Real(  6.00),       0,       00.0000,                2.39384867891882),
            ("23.0",    format!("/{}/Load", self_id),       Value::Real( 12.00),       0,       00.0000,                3.59461759405396),
            ("24.0",    format!("/{}/Load", self_id),       Value::Real( 64.00),       1,       11.1452903947972,       11.1452903947972),
            ("25.0",    format!("/{}/Load", self_id),       Value::Real(128.00),       1,       25.7521290954476,       25.7521290954476),
            ("26.0",    format!("/{}/Load", self_id),       Value::Real(120.00),       1,       37.5331129585166,       37.5331129585166),
            ("27.0",    format!("/{}/Load", self_id),       Value::Real(133.00),       1,       49.466473838702,       49.466473838702),
            ("28.0",    format!("/{}/Load", self_id),       Value::Real(121.00),       1,       58.4081646088643,       58.4081646088643),
            ("29.0",    format!("/{}/Load", self_id),       Value::Real(130.00),       1,       67.3571440327563,       67.3571440327563),
            ("30.0",    format!("/{}/Load", self_id),       Value::Real(127.00),       1,       67.3571440327563,       74.8125010286617),
            ("31.0",    format!("/{}/Load", self_id),       Value::Real(123.00),       1,       80.835938400079,       80.835938400079),
            ("32.0",    format!("/{}/Load", self_id),       Value::Real(122.00),       1,       80.835938400079,       85.9814461000691),
            ("33.0",    format!("/{}/Load", self_id),       Value::Real(120.00),       1,       90.2337653375605,       90.2337653375605),
            ("34.0",    format!("/{}/Load", self_id),       Value::Real( 64.00),       1,       90.2337653375605,       86.9545446703654),
            ("35.0",    format!("/{}/Load", self_id),       Value::Real( 32.00),       1,       80.0852265865698,       80.0852265865698),
            ("36.0",    format!("/{}/Load", self_id),       Value::Real( 24.00),       1,       80.0852265865698,       73.0745732632485),
            ("37.0",    format!("/{}/Load", self_id),       Value::Real( 12.00),       1,       65.4402516053425,       65.4402516053425),
            ("38.0",    format!("/{}/Load", self_id),       Value::Real(  8.00),       1,       65.4402516053425,       58.2602201546747),
            ("39.0",    format!("/{}/Load", self_id),       Value::Real( 17.00),       1,       53.1026926353403,       53.1026926353403),
            ("40.0",    format!("/{}/Load", self_id),       Value::Real( 10.00),       1,       53.1026926353403,       47.7148560559228),
            ("41.0",    format!("/{}/Load", self_id),       Value::Real(  7.00),       1,       42.6254990489324,       42.6254990489324),
            ("42.0",    format!("/{}/Load", self_id),       Value::Real(  3.00),       1,       42.6254990489324,       37.6723116678159),
            ("43.0",    format!("/{}/Load", self_id),       Value::Real(  6.00),       1,       33.7132727093389,       33.7132727093389),
            ("44.0",    format!("/{}/Load", self_id),       Value::Real(  4.00),       1,       33.7132727093389,       29.9991136206715),
            ("45.0",    format!("/{}/Load", self_id),       Value::Real(  2.00),       1,       33.7132727093389,       26.4992244180876),
            ("46.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       1,       23.1868213658266,       23.1868213658266),
            ("47.0",    format!("/{}/Load", self_id),       Value::Real(  4.00),       1,       23.1868213658266,       20.7884686950983),
            ("47.1",    format!("/{}/Winch1.Load.Limiter.Trip", self_id),       Value::Bool(true),       1,       23.1868213658266,       20.7884686950983),
            ("48.0",    format!("/{}/Load", self_id),       Value::Real(  2.00),       1,       23.1868213658266,       18.439910108211),
            ("49.0",    format!("/{}/Load", self_id),       Value::Real(  1.00),       1,       23.1868213658266,       16.2599213446847),
            ("50.0",    format!("/{}/Load", self_id),       Value::Real(  3.00),       1,       14.6024311765991,       14.6024311765991),
            ("51.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       1,       14.6024311765991,       12.7771272795242),
            ("52.0",    format!("/{}/Load", self_id),       Value::Real(  2.00),       1,       14.6024311765991,       11.4299863695837),
            ("53.0",    format!("/{}/Load", self_id),       Value::Real(  1.00),       1,       14.6024311765991,       10.1262380733857),
            ("54.0",    format!("/{}/Load", self_id),       Value::Real(  0.70),       1,       14.6024311765991,       8.94795831421249),
            ("55.0",    format!("/{}/Load", self_id),       Value::Real(  0.80),       1,       14.6024311765991,       7.92946352493593),
            ("56.0",    format!("/{}/Load", self_id),       Value::Real(  0.40),       0,       6.98828058431894,       6.98828058431894),
            ("57.0",    format!("/{}/Load", self_id),       Value::Real(  0.30),       0,       6.98828058431894,       6.15224551127907),
            ("58.0",    format!("/{}/Load", self_id),       Value::Real(  0.20),       0,       6.98828058431894,       5.40821482236919),
            ("59.0",    format!("/{}/Load", self_id),       Value::Real(  0.10),       0,       6.98828058431894,       4.74468796957304),
            ("60.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       6.98828058431894,       4.15160197337641),
            ("61.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       6.98828058431894,       3.63265172670436),
            ("62.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       6.98828058431894,       3.17857026086631),
            ("63.0",    format!("/{}/Load", self_id),       Value::Real(  0.00),       0,       6.98828058431894,       2.78124897825802),
            ("64.0",    format!("/{}/Exit", self_id),       Value::String("exit".to_owned()),       0,       6.98828058431894,       2.78124897825802),
        ];
        let total_count = test_data.len();
        let (len, sum) = test_data.iter().fold((0, 0.0), |(mut len, mut sum), (i, _name, value, _op_cycle, _thrd, _smooth)| {
            if _name == &format!("/{}/Load", self_id) {
                if _op_cycle > &0 {
                    len += 1;
                    sum += value.as_real();
                }
            }
            println!("{}\taverage: {}", i, sum / (len as f32));
            (len, sum)
        });
        let target_average = sum / (len as f32);
        let target_thrd: Vec<(&str, f32)> = test_data.iter().filter_map(|(i, _name, _value, _op_cycle, thrd, _smooth)| {
            Some((*i, thrd.clone()))
        }).collect();
        let target_smooth: Vec<(&str, f32)> = test_data.iter().filter_map(|(i, _name, _value, _op_cycle, _thrd, smooth)| {
            Some((*i, smooth.clone()))
        }).collect();
        let target_op_cycle: Vec<(&str, i32)> = test_data.iter().filter_map(|(i, _name, _value, op_cycle, _thrd, _smooth)| {
            Some((*i, op_cycle.clone()))
        }).collect();
        let target_thrd_count = target_thrd.len();
        let target_smooth_count = target_smooth.len();
        let receiver = Arc::new(Mutex::new(TaskTestReceiver::new(
            self_id,
            "",
            "in-queue",
            total_count * 100,
        )));
        services.slock().insert(receiver.clone());
        let test_data: Vec<(String, Value)> = test_data.into_iter().map(|(_, name, value, _, _, _)| {
            (name, value)
        }).collect();
        let producer = Arc::new(Mutex::new(TaskTestProducer::new(
            self_id,
            &format!("/{}/MultiQueue.in-queue", self_id),
            Duration::from_millis(10),
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
        println!("target smooth   : {:?}", target_smooth_count);
        println!("target threshold: {:?}", target_thrd_count);
        for (i, result) in receiver.lock().unwrap().received().lock().unwrap().iter().enumerate() {
            println!("received: {}\t|\t{}\t|\t{:?}", i, result.name(), result.value());
            // assert!(result.name() == target_name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target_name);
        };
        assert!(sent == total_count, "\nresult: {:?}\ntarget: {:?}", sent, total_count);
        assert!(result >= total_count, "\nresult: {:?}\ntarget: {:?}", result, total_count);
        let smooth: Vec<PointType> = receiver.lock().unwrap().received().lock().unwrap().iter().cloned().filter(|point| {
            point.name() == format!("/{}/RecorderTask/Smooth", self_id)
        }).collect();
        for (i, result) in smooth.iter().enumerate() {
            println!("smooth: {}\t|\t{}\t|\t{:?}", i, result.name(), result.value());
        };
        let thrd: Vec<PointType> = receiver.lock().unwrap().received().lock().unwrap().iter().cloned().filter(|point| {
            point.name() == format!("/{}/RecorderTask/Threshold", self_id)
        }).collect();
        for (i, result) in thrd.iter().enumerate() {
            println!("threshold: {}\t|\t{}\t|\t{:?}", i, result.name(), result.value());
        };
        let op_cycle: Vec<PointType> = receiver.lock().unwrap().received().lock().unwrap().iter().cloned().filter(|point| {
            point.name() == format!("/{}/RecorderTask/OpCycle", self_id)
        }).collect();
        for (i, result) in op_cycle.iter().enumerate() {
            println!("op cycle: {}\t|\t{}\t|\t{:?}", i, result.name(), result.value());
        };
        let op_cycle_sql: Vec<PointType> = receiver.lock().unwrap().received().lock().unwrap().iter().cloned().filter(|point| {
            point.name() == format!("/{}/RecorderTask/OpCycleSql", self_id)
        }).collect();
        for (i, result) in op_cycle_sql.iter().enumerate() {
            println!("op cycle SQL: {}\t|\t{}\t|\t{:?}", i, result.name(), result.value());
        };
        println!("target average: {}", target_average);
        // let target_name = "/AppTest/RecorderTask/Smooth";
        // for (i, result) in smooth.iter().enumerate() {
        //     let (step, target) = target_smooth[i].clone();
        //     assert!(result.value().as_real().aprox_eq(target, 3), "step {} \nresult smooth: {:?}\ntarget smooth: {:?}", step, result.value(), target);
        //     assert!(result.name() == target_name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target_name);
        // };
        // let target_name = "/AppTest/RecorderTask/Threshold";
        // for (i, result) in thrd.iter().enumerate() {
        //     let (step, target) = target_thrd[i].clone();
        //     assert!(result.value().as_real().aprox_eq(target, 3), "step {} \nresult threshold: {:?}\ntarget threshold: {:?}", step, result.value(), target);
        //     assert!(result.name() == target_name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target_name);
        // };
        let target_name = "/AppTest/RecorderTask/OpCycle";
        for (i, result) in op_cycle.iter().enumerate() {
            let (step, target) = target_op_cycle[i].clone();
            assert!(result.value().as_bool() == (target > 0), "step {} \nresult op cycle: {:?}\ntarget op cycle: {:?}", step, result.value(), target);
            assert!(result.name() == target_name, "step {} \nresult: {:?}\ntarget: {:?}", step, result.name(), target_name);
        };
        test_duration.exit();
    }
}

